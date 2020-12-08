use crate::{
    error::Error,
    templates::{add_helpers, add_templates},
};
use bytes::BytesMut;
use handlebars::Handlebars;
use inflector::{
    cases::{classcase::to_class_case, snakecase::to_snake_case},
    string::pluralize::to_plural,
    string::singularize::to_singular,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap as Map, sync::Arc};
use zenkit::{
    types::{ChildList, Element, ElementCategoryId, NumericType, Workspace},
    ApiClient, ListInfo,
};

type RenderMap = Map<&'static str, Value>;

pub struct Generator<'gen> {
    hb: Handlebars<'gen>,
    buf: BytesMut,
    data: RenderMap,
}

enum BuilderType {
    New,
    Update,
}

impl<'gen> Generator<'gen> {
    /// Initialize handlebars in strict mode
    /// Load handlebars_misc_helpers for string conversions.
    /// Add our custom helpers and templates (all defined in src/templates.rs)
    pub fn init() -> Result<Self, crate::error::Error> {
        let mut hb = Handlebars::new();
        handlebars_misc_helpers::setup_handlebars(&mut hb);
        hb.set_strict_mode(true);
        hb.register_escape_fn(handlebars::no_escape); // don't escape since we are not html
        add_helpers(&mut hb)?;
        add_templates(&mut hb)?;
        Ok(Self {
            hb,
            buf: BytesMut::new(),
            data: RenderMap::new(),
        })
    }
    /// sets template value to string
    #[inline]
    fn set<T: Into<String>>(&mut self, k: &'static str, v: T) {
        self.data.insert(k, Value::String(v.into()));
    }
    /// sets template value to string or ""
    #[inline]
    fn set_opt(&mut self, k: &'static str, v: &Option<String>) {
        self.data.insert(
            k,
            Value::String(v.as_ref().unwrap_or(&"".to_string()).clone()),
        );
    }
    /// sets template value to id (unsigned int)
    #[inline]
    fn set_id(&mut self, k: &'static str, id: u64) {
        self.data
            .insert(k, Value::Number(serde_json::Number::from(id)));
    }
    /// sets template value to bool
    #[inline]
    fn set_bool(&mut self, k: &'static str, b: bool) {
        self.data.insert(k, Value::Bool(b));
    }

    /// Generate everything specific to a list
    ///  - struct MyList (with get(id), create(), and update())
    ///  - calls gen_item_impl to generate item getters and label lookups
    pub fn gen_list(&mut self, list_info: &Arc<ListInfo>) -> Result<(), Error> {
        let list = list_info.list();
        self.set("list", &list.name);
        self.set_id("list_id", list.id);
        self.set("list_uuid", &list.uuid);
        self.set("list_desc", &list.description);

        let item = if let Some(ref item_name) = list.item_name {
            // they specified item name in ui
            to_class_case(&item_name)
        } else {
            to_class_case(&list.name)
        };
        let item_plural = if let Some(ref item_name_plural) = list.item_name_plural {
            to_class_case(&item_name_plural)
        } else {
            to_plural(&item)
        };
        self.set("item", &item);
        self.set("item_plural", &item_plural);

        self.render("start_list_impl")?;

        // item impl (all getters, and label lookups)
        self.gen_item_impl(&list_info)?;

        // create item builder
        self.gen_builder(&list_info, BuilderType::New)?;
        // update item builder
        self.gen_builder(&list_info, BuilderType::Update)?;

        for f in ["list", "list_id", "list_uuid", "item"].iter() {
            self.data.remove(f);
        }

        Ok(())
    }

    /// Generate label id and lookup. (per category field)
    fn gen_get_category(&mut self, field: &Element) -> Result<(), Error> {
        if let Some(ref categories) = field.element_data.predefined_categories {
            let mut labels: Vec<&String> = Vec::new();
            for c in categories {
                // is_Field_Category() -> bool
                self.set("label", &c.name);
                self.set_id("label_id", c.id);
                self.render("category_getter_per_label")?;
                labels.push(&c.name);
            }
            self.data.remove("label");
            self.data.remove("label_id");

            // Generate lookup (label_id_for_FIELD)
            // and constant label->id lookup table
            // These are used by setters but we only need to generate once
            // and setters can access them with Item:: qualifier
            self.set_id("len", labels.len() as u64);
            labels.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            self.data.insert("labels", json!(labels));
            self.render("category_label_lookup")?;

            self.data.remove("lookup_array");
            self.data.remove("len");
            self.data.remove("labels");
        }

        self.render("category_getters_per_field")?;
        Ok(())
    }

    /// Label setters - inside builder
    fn gen_set_labels(&mut self, _list_name: &str, field: &Element) -> Result<(), Error> {
        // const LABEL...
        if let Some(ref categories) = field.element_data.predefined_categories {
            for c in categories {
                // is_Field_Category() -> bool
                self.set("label", &c.name);
                self.set_id("label_id", c.id);
                // label-specific setter
                self.render("category_setter_per_label")?;
            }
            self.data.remove("label");
            self.data.remove("label_id");
        }
        // field-specific setter
        // get_Field(s)() -> Vec<String>
        self.render("category_setter_per_field")?;

        Ok(())
    }

    /// The main list-item wrapper with all getters, and label lookups
    fn gen_item_impl(&mut self, list_info: &Arc<ListInfo>) -> Result<(), Error> {
        for field in list_info
            .fields()
            .iter()
            .filter(|f| f.deprecated_at.is_none())
        {
            self.set("field", &field.name);
            self.set("field_uuid", &field.uuid);
            self.set_id("field_id", field.id);
            self.set("field_desc", field.get_description());
            self.set_bool("field_single_value", !field.element_data.multiple);
            self.set_bool("field_multiple_value", field.element_data.multiple);

            self.render("field_const")?;

            match field.element_category {
                ElementCategoryId::Text => self.render("get_text_field"),
                ElementCategoryId::URL => self.render("get_url_field"),
                ElementCategoryId::Date => self.render("get_date_field"),
                ElementCategoryId::Persons => self.render("get_person_field"),
                ElementCategoryId::Number => {
                    match field.numeric_type() {
                        Some(NumericType::Integer) => self.render("get_int_field"),
                        //Some(NumericType::Decimal) => {}
                        //None =>
                        _ => self.render("get_float_field"),
                    }
                }
                ElementCategoryId::Checkbox => self.render("get_checkbox_field"),
                ElementCategoryId::Categories => self.gen_get_category(&field),
                ElementCategoryId::Files => self.render("get_files_field"),
                ElementCategoryId::References => {
                    if let Some(ChildList::Child(ref child_list)) = field.element_data.child_list {
                        self.set("ref_list", &child_list.name);
                    } else {
                        self.set("ref_list", "");
                    }
                    self.render("get_references")?;
                    self.data.remove("ref_list");
                    Ok(())
                }
                ElementCategoryId::SubEntries => self.render("get_subitems_field"),
                ElementCategoryId::Formula => self.render("get_formula_field"),

                // handled in misc_getters
                ElementCategoryId::UserCreatedBy
                | ElementCategoryId::UserUpdatedBy
                | ElementCategoryId::DateCreated
                | ElementCategoryId::DateUpdated
                | ElementCategoryId::DateDeprecated
                | ElementCategoryId::UserDeprecatedBy => Ok(()),

                // unimplemented ...
                ElementCategoryId::Hierarchy | ElementCategoryId::Dependencies => {
                    println!(
                        "Warning: Field getters not implemented for {}.{} type {}",
                        list_info.list().name,
                        field.name,
                        field.element_category
                    );
                    continue;
                }
            }?;
        }

        for f in [
            "field",
            "field_uuid",
            "field_id",
            "field_single_value",
            "field_multiple_value",
        ]
        .iter()
        {
            self.data.remove(f);
        }

        // end impl MyList
        self.render("end_list_impl")?;

        Ok(())
    }

    /// Generates builders
    /// This is called twice: once for NewitemBuilder, once for UpdateItemBuilder
    fn gen_builder(
        &mut self,
        list_info: &Arc<ListInfo>,
        builder_type: BuilderType, // either new or update
    ) -> Result<(), Error> {
        match builder_type {
            BuilderType::New => {
                self.set_bool("is_new_builder", true);
                self.set_bool("is_update_builder", false);
            }
            BuilderType::Update => {
                self.set_bool("is_new_builder", false);
                self.set_bool("is_update_builder", true);
            }
        }
        self.set(
            "builder",
            format!(
                "{}{}Builder",
                match builder_type {
                    BuilderType::Update => "Update",
                    BuilderType::New => "New",
                },
                // "item" was already set to be the preferred name for object
                self.data
                    .get("item")
                    .map(|s| s.as_str())
                    .unwrap_or_default()
                    .unwrap_or(&to_singular(&to_class_case(&list_info.list().name)))
            ),
        );

        self.render("start_item_builder")?;

        for field in list_info
            .fields()
            .iter()
            .filter(|f| f.deprecated_at.is_none())
        {
            self.set("field", &field.name);
            self.set("field_uuid", &field.uuid);
            self.set_id("field_id", field.id);
            self.set_bool("field_single_value", !field.element_data.multiple);
            self.set_bool("field_multiple_value", field.element_data.multiple);
            self.set("field_desc", field.get_description());

            match field.element_category {
                ElementCategoryId::Text => self.render("set_text_field"),
                ElementCategoryId::URL => self.render("set_url_field"),
                ElementCategoryId::Date => self.render("set_date_field"),
                ElementCategoryId::Persons => self.render("set_person_field"),

                ElementCategoryId::Number => {
                    match field.numeric_type() {
                        Some(NumericType::Integer) => self.render("set_int_field"),
                        //Some(NumericType::Decimal) => {}
                        //None =>
                        _ => self.render("set_float_field"),
                    }
                }
                ElementCategoryId::Checkbox => self.render("set_checkbox_field"),
                ElementCategoryId::Categories => {
                    self.gen_set_labels(&list_info.list().name, &field)
                }
                ElementCategoryId::References => {
                    if let Some(ChildList::Child(ref child_list)) = field.element_data.child_list {
                        self.set("ref_list", &child_list.name);
                    } else {
                        self.set("ref_list", "");
                    }
                    self.render("set_references")
                }
                ElementCategoryId::SubEntries => self.render("set_subitems"),

                // No api support for updating these
                ElementCategoryId::DateCreated
                | ElementCategoryId::DateUpdated
                | ElementCategoryId::DateDeprecated
                | ElementCategoryId::UserCreatedBy
                | ElementCategoryId::UserUpdatedBy
                | ElementCategoryId::UserDeprecatedBy
                | ElementCategoryId::Formula
                | ElementCategoryId::Files
                | ElementCategoryId::Hierarchy // deprecatd?
                | ElementCategoryId::Dependencies // deprecated?
                   => Ok(()),
            }?;
            self.data.remove("ref_list");
        }

        self.render("builder_execute")?;
        self.render("end_item_builder")?;

        for f in [
            "field",
            "field_uuid",
            "field_id",
            "field_single_value",
            "field_multiple_value",
            "is_update_builder",
            "is_new_builder",
            "builder",
        ]
        .iter()
        {
            self.data.remove(f);
        }
        Ok(())
    }

    /// Writes the internal buffer to the module output file
    fn write_to(&mut self, fpath: &str) -> Result<(), Error> {
        let buf = self.clone_reset();
        std::fs::write(&fpath, &buf.as_ref())?;
        Ok(())
    }

    /// Generates rust source code for Zenkit business model.
    /// Output is intended to be a crate library, generated in output dir
    pub async fn gen_workspace<'ws>(
        &mut self,
        api: &ApiClient,
        workspace: Arc<Workspace>,
        output_dir: &str,
    ) -> Result<Vec<String>, Error> {
        self.set("workspace", &workspace.name);
        self.set("workspace_uuid", &workspace.uuid);
        self.set_id("workspace_id", workspace.id);
        self.set_opt("workspace_desc", &workspace.description);

        // list of file paths generated
        let mut files: Vec<String> = Vec::new();

        // module files created
        let mut modules: Vec<String> = Vec::new();

        // generate file for each List
        for list in workspace.lists.iter() {
            let list_info = api.get_list_info(workspace.id, &list.uuid).await?;
            self.gen_list(&list_info)?;
            let mod_name = to_snake_case(&list.name);
            let fpath = format!("{}/src/{}.rs", output_dir, mod_name);
            self.write_to(&fpath)?;
            modules.push(mod_name);
            files.push(fpath);
        }
        println!("Modules: {:#?}", &modules);

        // write the crate lib.rs
        self.data.insert("modules", json!(modules));
        self.render("lib_main")?;
        let fpath = format!("{}/src/lib.rs", output_dir);
        self.write_to(&fpath)?;
        files.push(fpath);

        // write Cargo.toml.sample
        let crate_name = to_snake_case(&workspace.name);
        self.set("crate", &crate_name);
        self.render("cargo_toml")?;
        let fpath = format!("{}/Cargo.toml.sample", output_dir);
        self.write_to(&fpath)?;
        files.push(fpath);

        Ok(files)
    }

    fn render(&mut self, tmpl: &str) -> Result<(), Error> {
        self.buf
            .extend(self.hb.render(tmpl, &self.data)?.as_bytes());
        Ok(())
    }

    /// Returns copy of current buffer and empties own buffer
    pub fn clone_reset(&mut self) -> BytesMut {
        self.buf.split()
    }
}

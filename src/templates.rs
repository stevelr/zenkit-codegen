use crate::Error;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use inflector::cases::{pascalcase::to_pascal_case, screamingsnakecase::to_screaming_snake_case};

/// Helper functions - "macros" used within templates
/// These are used to ensure consistency when generating symbol names
pub fn add_helpers(hb: &mut Handlebars) -> Result<(), Error> {
    // to-workspace-uuid generates the workspace uuid symbol, e.g.: WORKSPACE_CUSTOMER_SUPPORT_UUID
    hb.register_helper(
        "to-workspace-uuid",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "WORKSPACE_{}_UUID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-workspace-id generates the workspace id symbol, e.g.: WORKSPACE_CUSTOMER_SUPPORT_ID
    hb.register_helper(
        "to-workspace-id",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "WORKSPACE_{}_ID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-workspace-name generates the workspace name symbol, e.g.: WORKSPACE_CUSTOMER_SUPPORT_NAME
    hb.register_helper(
        "to-workspace-name",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "WORKSPACE_{}_NAME",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-list-id generates the list id symbol, e.g.: LIST_TICKET_ID
    hb.register_helper(
        "to-list-id",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "LIST_{}_ID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-list-short-id generates the list short-id symbol, e.g.: LIST_TICKET_SHORT_ID
    hb.register_helper(
        "to-list-short-id",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "LIST_{}_SHORT_ID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-list-uuid generates the list id symbol, e.g.: LIST_TICKET_UUID
    hb.register_helper(
        "to-list-uuid",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "LIST_{}_UUID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-list-class generates the list struct name, e.g.: CustomersList
    hb.register_helper(
        "to-list-class",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "{}List",
                    to_pascal_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-list-name generates the symbol for the list name, e.g., LIST_CUSTOMER_NAME
    // (which evalutes to the static &str)
    hb.register_helper(
        "to-list-name",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "LIST_{}_NAME",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-field-id generates the symbol for the field id, e.g., FIELD_TITLE_ID
    hb.register_helper(
        "to-field-id",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "FIELD_{}_ID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-field-uuid generates the symbol for the field uuid, e.g., FIELD_TITLE_UUID
    hb.register_helper(
        "to-field-uuid",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "FIELD_{}_UUID",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );
    // to-field-name generates the symbol for the field name, e.g., FIELD_TITLE_NAME
    hb.register_helper(
        "to-field-name",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                out.write(&format!(
                    "FIELD_{}_NAME",
                    to_screaming_snake_case(
                        h.param(0)
                            .ok_or_else(|| RenderError::new("param not found"))?
                            .value()
                            .as_str()
                            .ok_or_else(|| RenderError::new("not string"))?
                    )
                ))?;
                Ok(())
            },
        ),
    );

    Ok(())
}

pub fn add_templates(hb: &mut Handlebars) -> Result<(), Error> {
    let templates: Vec<(&str, &str)> = vec![
        (
            // begin the main crate source file lib.rs
            "lib_main",
            r#"#![allow(dead_code, unused_imports)]
            /// Zenkit Workspace {{ workspace }}
            /// {{ workspace_desc }}
            //  {{ generated_banner }}

            use std::fmt;
            // use and re-export
            pub use zenkit::{init_api, get_api, ApiClient, ApiConfig,
                             types::{ID,DateTime,Entry,GetEntriesRequest,JsonMap,TextFormat,Utc}};

            {{#each modules ~}}
            mod {{ this }};
            pub use {{ this }}::*;
            {{/each}}

            const ZENKIT_API_TOKEN_VAR: &str = "ZENKIT_API_TOKEN";
            pub(crate) const ZENKIT_ITEM_URL_BASE: &str = "https://base.zenkit.com/i/";

            /// Errors returned by this crate
            #[derive(Debug)]
            pub enum Error {
                Message(String),
                Zenkit(String),
                NoApi,
            }

            impl fmt::Display for Error {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                    write!(
                        f,
                        "{:?}",
                        match self {
                            Error::Message(s) => s,
                            Error::Zenkit(s) => s,
                            Error::NoApi => "Api must be initialized with 'initialize_zenkit_api()' before use",
                        }
                    )
                }
            }

            impl From<zenkit::Error> for Error {
                fn from(e: zenkit::Error) -> Error {
                    Error::Zenkit(e.to_string())
                }
            }

            impl From<std::io::Error> for Error {
                fn from(e: std::io::Error) -> Error {
                    Error::Message(e.to_string())
                }
            }

            impl std::error::Error for Error {}

            /// Initialize zenkit using api token.
            /// If token is None, the value will be taken from the environment variable ZENKIT_API_TOKEN.
            /// Returns error if token is undefined
            pub fn initialize_zenkit_api(token: Option<&'_ str>, endpoint: Option<&'_ str>) -> Result<&'static ApiClient, Error>
            {
                let defaults = ApiConfig::default();
                let token = token.unwrap_or(&defaults.token).to_string();
                let endpoint = endpoint.unwrap_or(&defaults.endpoint).to_string();
                let zk = init_api(ApiConfig{token, endpoint}).map_err(|e| Error::Message(e.to_string()))?;
                Ok(zk)
            }

            /// Find id for label name using binary search
            pub(crate) fn lookup_label(lookup_list: &[(&str, ID)], label: &str) -> Option<ID> {
                match lookup_list.binary_search_by(|a| a.0.partial_cmp(label).unwrap()) {
                    Ok(index) => Some(lookup_list[index].1),
                    Err(_) => None,
                }
            }

            ///
            /// Workspace "{{ workspace }}"
            /// {{ workspace_desc }}
            ///
            pub const {{ to-workspace-id workspace }}: ID = {{ workspace_id }};
            pub const {{ to-workspace-uuid workspace }}: &str = "{{ workspace_uuid }}";
            pub const {{ to-workspace-name workspace }}: &str = "{{ workspace }}";

            // Load all entries for a list into memory
            pub(crate) async fn load_entries(list_id: ID) -> Result<Vec<Entry>, Error> {
                let mut ooo_entries: Vec<Entry> = Vec::new();
                let max_items = 500usize;
                let mut start_index = 0usize;
                let api = get_api()?;
                loop {
                    // get the items and build the index
                    let mut results: Vec<Entry> = api
                        .get_list_entries(
                            list_id,
                            &GetEntriesRequest{limit:max_items, skip:start_index, ..Default::default()}
                        ).await?;
                    if results.is_empty() {
                        break;
                    }
                    start_index += results.len();
                    ooo_entries.append(&mut results);
                }
                Ok(ooo_entries)
            }
"#,
        ),
        (
            "start_list_impl",
            r#"#![allow(dead_code, unused_imports)]
            //! {{ list_struct }} {{list_desc}}
            //  {{ generated_banner }}
            use crate::{lookup_label, Error};
            use serde_json::{self, json, Value};
            use std::{convert::AsRef, rc::Rc, str::FromStr};
            use zenkit::{get_api, types::{DateTime, Entry, File, ID, JsonMap, TextFormat, Utc}};

            /// {{ list_struct }} {{list_desc}}
            ///
            pub const {{ to-list-id list }}: ID = {{ list_id }};
            pub const {{ to-list-short-id list }}: &str = "{{ list_short_id }}";
            pub const {{ to-list-uuid list }}: &str = "{{ list_uuid }}";
            pub const {{ to-list-name list }}: &str = "{{ list }}";

            /// {{ list_struct }} - List of {{ item_plural }}
            /// {{list_desc}}
            pub struct {{ list_struct }} { }

            impl {{ list_struct }} {

                /// fetch {{ item }} by its ID
                pub async fn get( {{ to_snake_case item }}_id: ID) -> Result<{{ item }},Error> {
                    let obj = get_api()?.get_entry({{ to-list-id list }}, {{ to_snake_case item }}_id).await?;
                    Ok({{ item }}::new( Rc::new(obj)))
                }

                /// fetch {{ item }} by its UUID
                pub async fn get_by_uuid ( {{ to_snake_case ( item ) }}_uuid: &str) -> Result<{{ item }},Error> {
                    let obj = get_api()?.get_entry({{ to-list-id list }}, {{ to_snake_case item }}_uuid).await?;
                    Ok({{ item }}::new(Rc::new(obj)))
                }

                /// Returns all {{ item_plural }}
                pub async fn get_items() -> Result<Vec<{{ item }}>, Error> {
                    let entries = crate::load_entries({{ to-list-id list }}).await?;
                    let items: Vec<{{ item }}> = entries
                        .into_iter()
                        .map(|e| {{item}}::new(Rc::new(e)))
                        .collect();
                    Ok(items)
                }

                /// Initialize builder for creating a new {{item}}
                pub fn create() -> New{{ item }}Builder {
                    New{{ item }}Builder::new()
                }

                /// Creates builder for updating an existing {{item}}
                pub fn update( {{ to_snake_case ( item ) }}_id: ID) -> Update{{ item }}Builder {
                    Update{{ item }}Builder::new({{ to_snake_case ( item ) }}_id)
                }
            }

            /// {{ item }} (item of List '{{ list }}') {{list_desc}}
            pub struct {{ item }} {
                obj: Rc<Entry>
            }

            impl {{ item }} {

                /// Creates {{ item }} wrapping Zenkit Entry
                fn new(obj: Rc<Entry>) -> Self {
                    Self{ obj }
                }
            "#,
        ),
        // lookup label id from name (per field impl)
        (
            "category_label_lookup",
            r#"
            /// Returns the id for a label, or None if not valid for this field
            pub fn label_id_for_{{ to_snake_case field }}(label: &str) -> Option<ID> {
                lookup_label(&{{item}}::LABELS_{{ to_screaming_snake_case field }}, label)
            }

            /// label index used for converting from label name to id (sorted)
            const LABELS_{{ to_screaming_snake_case field }}: [(&'static str,u64);{{ len }}] = [
                {{#each labels ~}}
                ("{{ this }}", {{../item}}::LABEL_{{ to_screaming_snake_case ../field }}_{{ to_screaming_snake_case this }}_ID),
                {{/each ~}}
            ];
            "#,
        ),
        // Finish the Item list impl with some common getters
        (
            "end_list_impl",
            r#"

            /// Returns url to zenkit item
            pub fn get_zenkit_url(&self) -> String {
                format!("{}{}/{}/", crate::ZENKIT_ITEM_URL_BASE,
                    {{ to-list-short-id list }}, self.obj.short_id)
            }

            /// Returns the item's display string (based on list's primary field)
            pub fn get_display_string(&self) -> &str {
                &self.obj.display_string
            }

            /// Returns user id that created the item
            pub fn get_created_by_id(&self) -> ID {
                self.obj.created_by
            }

            /// Returns user name that created the item
            pub fn get_created_by_name(&self) -> Option<&str> {
                self.obj.created_by_displayname.as_deref()
            }

            /// Returns user id that created the item
            pub fn get_updated_by_id(&self) -> ID {
                self.obj.updated_by
            }

            /// Returns user name that last updated the item
            pub fn get_updated_by_name(&self) -> Option<&str> {
                self.obj.updated_by_displayname.as_deref()
            }

            /// Returns date the item was created, in UTC
            pub fn get_created_date(&self) -> &DateTime<Utc> {
                &self.obj.created_at
            }

            /// Returns date the item was last updated, in UTC
            pub fn get_updated_date(&self) -> &DateTime<Utc> {
                &self.obj.updated_at
            }

            /// Returns date the item was deprecated, in UTC, or None if item isn't deprecated
            pub fn get_deprecated_date(&self) -> Option<&DateTime<Utc>> {
                self.obj.deprecated_at.as_ref()
            }

            /// Returns user id that deprecated the item, or None if item isn't deprecated
            pub fn get_deprecated_by_id(&self) -> Option<ID> {
                self.obj.deprecated_by
            }

            /// Returns the default sort rank within the list
            pub fn get_sort_order(&self) -> f32 {
                self.obj.sort_order
            }

            /// Returns the item's id within the list
            pub fn get_id(&self) -> ID {
                self.obj.id
            }

            /// Returns the item's uuid
            pub fn get_uuid(&self) -> &str {
                &self.obj.uuid
            }

            /// Returns the underlying Entry object
            pub fn get_entry(&self) -> Rc<Entry> {
                self.obj.clone()
            }

            } // impl {{ item }}
"#,
        ),
        // Define New/Update builder struct and beginning of impl
        (
            "start_item_builder",
            r#"
            {{#if is_update_builder ~}}
            /// Builder for updating existing {{ item }}
            {{else ~}}
            /// Builder for creating new {{ item }}
            {{/if ~}}
            pub struct {{ builder }} {
                fields: JsonMap,
                errs: Vec<String>,
                {{#if is_update_builder~}}
                item_id: ID,
                {{/if~}}
            }

            impl {{ builder }} {
                {{#if is_update_builder}}
                /// Create update builder for {{item}} with id
                fn new(item_id: ID) -> Self {
                    Self { fields: JsonMap::new(), errs: Vec::new(), item_id }
                }
                {{else}}
                /// Create builder for new {{item}}
                fn new() -> Self {
                    Self { fields: JsonMap::new(), errs: Vec::new() }
                }
                {{/if}}

                // internal helper for setting string field
                #[inline]
                fn set_s(&mut self, k: &str, v: String) {
                    self.fields.insert(k.to_string(), Value::String(v));
                }
                // internal helper for setting json value
                #[inline]
                fn set_v(&mut self, k: &str, v: Value) {
                    self.fields.insert(k.to_string(), v);
            }
            "#,
        ),
        // } end of impl for New/Update builder
        (
            "end_item_builder",
            r#"
            } // end impl {{ builder }}
            "#,
        ),
        // const MY_FIELD_ID = ...
        // const MY_FIELD_UUID = ...
        (
            "field_const",
            r#"
            /// '{{ field }}' in '{{ list }}' {{field_desc}}
            pub const {{ to-field-id field }}: ID = {{ field_id }};
            pub const {{ to-field-uuid field }}: &'static str = "{{ field_uuid }}";
            pub const {{ to-field-name field }}: &'static str = "{{ field }}";
            "#,
        ),
        // get text field
        (
            "get_text_field",
            r#"
            /// Returns '{{ field }}' - {{ field_desc }} or None if unset
            pub fn get_{{ to_snake_case field }}(&self) -> Option<&str> {
                self.obj.fields.get("{{ field_uuid }}_text")
                    .map(|v| v.as_str())
                    .unwrap_or_default()
            }

            /// Returns the text format for '{{ field }}'
            pub fn get_{{ to_snake_case field }}_format(&self) -> TextFormat {
                self.obj.fields.get("{{ field_uuid }}_textType")
                    .map(|v| v.as_str())
                    .unwrap_or_default()
                    // if undefined or unexpected value, return "plain"
                    .map(|f| TextFormat::from_str(f).unwrap_or_default())
                    .unwrap_or_default()
            }
            "#,
        ),
        // get url field
        (
            "get_url_field",
            r#"
            /// Returns '{{ field }}' - {{ field_desc }} or None if unset
            pub fn get_{{ to_snake_case field }}(&self) -> Option<&str> {
                self.obj.fields.get("{{ field_uuid }}_link")
                    .map(|v| v.as_str())
                    .unwrap_or_default()
            }
            "#,
        ),
        // get int field
        (
            "get_int_field",
            r#"
            /// Returns '{{ field }}' - {{ field_desc }} or None if unset
            pub fn get_{{ to_snake_case field }}(&self) -> Option<i64> {
                self.obj.fields.get("{{ field_uuid }}_number")
                    .map(|n| n.as_i64())
                    .unwrap_or_default()
            }
            "#,
        ),
        // get float field
        (
            "get_float_field",
            r#"
            /// Returns '{{ field }}' - {{ field_desc }} or None if unset
            pub fn get_{{ to_snake_case field }}(&self) -> Option<f64> {
                self.obj.fields.get("{{ field_uuid }}_number")
                    .map(|n| n.as_f64())
                    .unwrap_or_default()
            }
            "#,
        ),
        // get formula field
        (
            "get_formula_field",
            r#"
            /// Returns value of formula field '{{ field }}', or None
            /// if the field isn't calculated, or if there was an error calculating the value.
            /// For description of the error, use get_{{ to_snake_case field }}_error()
            pub fn get_{{ to_snake_case field }}(&self) -> Option<f64> {
                // Only return value if there's no error
                // or if there is some other reason formula value can be null
                if let None = self.get_{{ to_snake_case field}}_error() {
                        return self.obj.fields.get("{{ field_uuid }}_value")
                            .map(|n| n.as_f64())
                            .unwrap_or_default();
                }
                None
            }

            /// Returns error message for formula {{ field }}, if any
            pub fn get_{{ to_snake_case field }}_error(&self) -> Option<&String> {
                if let Some(Value::String(s)) = self.obj.fields.get("{{ field_uuid }}_valueErrorMessage") {
                    if s.len() > 0 {
                        return Some(s)
                    }
                }
                None
            }
            "#,
        ),
        // get checkbox field
        (
            "get_checkbox_field",
            r#"
            /// Returns true if '{{ field }}' is checked.  {{ field_desc }}
            pub fn is_checked_{{ to_snake_case field }}(&self) -> bool {
                self.obj.fields.get("{{ field_uuid }}_checked")
                    .map(|n| n.as_bool())
                    .unwrap_or_default()
                    .unwrap_or_default()
            }
            "#,
        ),
        // get date field
        (
            "get_date_field",
            r#"
            /// Returns '{{ field }}' in UTC {{ field_desc }}, or None if unset
            pub fn get_{{ to_snake_case field }}(&self) -> Option<DateTime<Utc>> {
                self.obj.fields.get("{{ field_uuid }}_date")
                    .map(|v| v.as_str())
                    .unwrap_or_default()
                    .map(|s| s.parse::<DateTime<Utc>>().ok())
                    .unwrap_or_default()
            }
            "#,
        ),
        // get date x field (add "date_" prefix
        (
            "get_date_x_field",
            r#"
            /// Returns '{{ field }}' in UTC {{ field_desc }}, or None if unset
            pub fn get_date_{{ to_snake_case field }}(&self) -> Option<DateTime<Utc>> {
                self.obj.fields.get("{{ field_uuid }}_date")
                    .map(|v| v.as_str())
                    .unwrap_or_default()
                    .map(|s| s.parse::<DateTime<Utc>>().ok())
                    .unwrap_or_default()
            }
            "#,
        ),
        // category getters (each label)
        (
            "category_getter_per_label",
            r#"
            pub const LABEL_{{ to_screaming_snake_case field }}_{{ to_screaming_snake_case label }}_ID : ID = {{ label_id }};

            /// Returns true if '{{ label }}' is set on '{{field}}'
            /// {{field_desc}}
            pub fn is_{{ to_snake_case field }}_{{ to_snake_case label }}(&self) -> bool {
                self.obj.fields.get("{{ field_uuid }}_categories")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                        .any(|n| n == {{ item }}::LABEL_{{ to_screaming_snake_case field }}_{{ to_screaming_snake_case label }}_ID))
                    .unwrap_or_default()
            }
            "#,
        ),
        // category field getters
        (
            "category_getters_per_field",
            r#"
            {{#if field_single_value}}
            /// Returns label value of '{{ field }}', or None if field is unset
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}(&self) -> Option<&str> {
                self.obj.fields.get("{{ field_uuid }}_categories_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .take(1)
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("name"))
                            .filter_map(|val| val.as_str())
                            .next()
                            .unwrap_or_default()
                    )
            }

            /// Returns id value of '{{ field }}', or None if field is unset
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_id(&self) -> Option<ID> {
                self.obj.fields.get("{{ field_uuid }}_categories")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .take(1)
                            .filter_map(|val| val.as_i64())
                            .next()
                            .map(|val| val as ID)
                    )
                    .unwrap_or_default()
            }
            {{else}}
            /// Returns list of labels (as ids) set on '{{ field }}'
            /// {{field_desc}}
            pub fn get_{{ to_plural ( to_snake_case field ) }}_ids(&self) -> Vec<ID> {
                self.obj.fields.get("{{ field_uuid }}_categories_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("id"))
                            .filter_map(|val| val.as_u64())
                            .collect()
                    )
                    .unwrap_or_else(Vec::new)
            }
            /// returns list of values set on '{{ field }}'
            /// {{field_desc}}
            pub fn get_{{ to_plural ( to_snake_case field ) }}_labels(&self) -> Vec<&str> {
                self.obj.fields.get("{{ field_uuid }}_categories_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("name"))
                            .filter_map(|val| val.as_str())
                            .collect()
                    )
                    .unwrap_or_else(Vec::new)
            }
            {{/if}}
            "#,
        ),
        // get people by names or ids
        (
            "get_person_field",
            r#"
            {{#if field_single_value}}
            /// Returns the person's name in {{ field }}, or None if unset
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_name(&self) -> Option<&str> {
                self.obj.fields.get("{{ field_uuid }}_persons_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .take(1)
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("displayname"))
                            .filter_map(|val| val.as_str())
                            .next()
                    )
            }

            /// Returns the person's id in {{ field }}, or None if unset
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_id(&self) -> ID {
                self.obj.fields.get("{{ field_uuid }}_persons_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .take(1)
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("id"))
                            .filter_map(|val| val.as_u64())
                            .next()
                    )
            }
            {{else}}
            /// Returns the names of persons in {{ field }}
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_names(&self) -> Vec<&str> {
                self.obj.fields.get("{{ field_uuid }}_persons_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("displayname"))
                            .filter_map(|val| val.as_str())
                            .collect()
                    )
                    .unwrap_or_else(Vec::new)
            }
            /// Returns the ids of persons in {{ field }}
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_ids(&self) -> Vec<ID> {
                self.obj.fields.get("{{ field_uuid }}_persons_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("id"))
                            .filter_map(|val| val.as_u64())
                            .collect()
                    )
                    .unwrap_or_else(Vec::new)
            }
            {{/if}}
            "#,
        ),
        // get references
        (
            "get_references",
            r#"
            {{#if field_single_value}}
            /// Returns the reference in {{field}} (to {{ref_list}}, or None if unset
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_uuid(&self) -> Option<&str> {
                self.obj.fields.get("{{ field_uuid }}_references_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .take(1)
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("uuid"))
                            .filter_map(|val| val.as_str())
                            .next()
                    )
            }
            {{else}}
            /// Returns the references in {{ field }} to {{ref_list}}
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_uuids(&self) -> Vec<&str> {
                self.obj.fields.get("{{ field_uuid }}_references_sort")
                    .map(|v| v.as_array())
                    .unwrap_or_default()
                    .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("uuid"))
                            .filter_map(|val| val.as_str())
                            .collect()
                    )
                    .unwrap_or_else(Vec::new)
            }
            {{/if}}
            "#,
        ),
        // get subitems (subEntries)
        (
            "get_subitems_field",
            r#"
            /// Returns subitem references in {{ field }} hierarchy
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_uuids(&self) -> Vec<&str> {
                if self.is_connected_{{ to_snake_case field }}() {
                    self.obj.fields.get("{{ field_uuid }}_references_sort")
                        .map(|v| v.as_array())
                        .unwrap_or_default()
                        .map(|v| v.iter()
                            .filter_map(|val| val.as_object())
                            .filter_map(|val| val.get("uuid"))
                            .filter_map(|val| val.as_str())
                            .collect()
                        )
                        .unwrap_or_else(Vec::new)
                } else {
                    Vec::new()
                }
            }

            /// Returns parent references in {{ field }} hierarchy
            /// {{field_desc}}
            pub fn get_{{ to_snake_case field }}_parents(&self) -> Vec<&str> {
                if self.is_connected_{{ to_snake_case field }}() {
                    self.obj.fields.get("{{ field_uuid }}_parents")
                        .map(|v| v.as_array())
                        .unwrap_or_default()
                        .map(|v| v.iter().filter_map(|v| v.as_str()).collect() )
                        .unwrap_or_else(Vec::new)
                } else {
                    Vec::new()
                }
            }

            /// Returns true if this item is connected in the {{ field }} hierarchy
            /// {{field_desc}}
            pub fn is_connected_{{ to_snake_case field }}(&self) -> bool {
                self.obj.fields.get("{{ field_uuid }}_connected")
                    .map(|v| v.as_bool())
                    .unwrap_or_default()
                    .unwrap_or_default()
            }
            "#,
        ),
        // get files field
        (
            "get_files_field",
            r#"
            /// Returns files from '{{ field }}' - {{ field_desc }}
            pub fn get_{{ to_snake_case field }}(&self) -> Vec<File> {
                match self.obj.fields.get("{{ field_uuid }}_filesData") {
                    Some(v) => match serde_json::from_value::<Vec<File>>(v.clone()) {
                        Ok(files) => files,
                        Err(e) => {
                            println!("Deser files failed: {:#?}", e);
                            println!("Original data: {:#?}", v);
                            Vec::new()
                        }
                    },
                    None => Vec::new()
                }
            }
            "#,
        ),
        // set text field
        (
            "set_text_field",
            r#"
            /// Sets text value for {{ field }}. {{field_desc}}
            {{#if is_update_builder ~}}
            /// TextFormat is unchanged (to specify format, use _with_format())
            {{else ~}}
            /// Uses the field's default TextFormat. To specify format, use _with_format()
            {{/if ~}}
            pub fn set_{{ to_snake_case field }}<T:AsRef<str>>(&mut self, s: T) -> &mut {{ builder }} {
                self.set_s("{{ field_uuid }}_text", s.as_ref().to_string());
                self
            }

            /// Sets text value for {{ field }} with format
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}_with_format<T:AsRef<str>>(&mut self, s: T, tf: TextFormat) -> &mut {{ builder }} {
                self.set_s("{{ field_uuid }}_text", s.as_ref().to_string());
                self.set_s("{{ field_uuid }}_textType", tf.to_string());
                self
            }
            "#,
        ),
        // set int field
        (
            "set_int_field",
            r#"
            /// Sets {{ field }} with integer value.
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, i: i64) -> &mut {{ builder }} {
                let jn = serde_json::Number::from(i);
                self.set_v("{{ field_uuid }}_number", Value::Number(jn));
                self
            }
            "#,
        ),
        // set float field
        (
            "set_float_field",
            r#"
            /// Sets {{ field }} with float value.
            /// {{field_desc}}
            /// Generates error if float value is Infinite or NaN.
            pub fn set_{{ to_snake_case field }}(&mut self, f: f64) -> &mut {{ builder }} {
                match serde_json::Number::from_f64(f) {
                    Some(n) => self.set_v("{{ field_uuid }}_number", Value::Number(n)),
                    None => self.errs.push("Float values cannot be Infinite or NaN".to_string())
                }
                self
            }
            "#,
        ),
        // set checkbox field
        (
            "set_checkbox_field",
            r#"
            /// sets checkbox {{ field }}
            /// {{field_desc}}
            pub fn check_{{ to_snake_case field }}(&mut self, b: bool) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_checked", Value::Bool(b));
                self
            }
            "#,
        ),
        // set url field
        (
            "set_url_field",
            r#"
            /// Sets URL {{ field }}. {{field_desc}}
            pub fn set_{{ to_snake_case field }}<T:AsRef<str>>(&mut self, url: T) -> &mut {{ builder }} {
                self.set_s("{{ field_uuid }}_link", url.as_ref().to_string());
                self
            }
            "#,
        ),
        // set date field
        (
            "set_date_field",
            r#"
            /// Sets date {{ field }}. {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, date: &DateTime<Utc>) -> &mut {{ builder }} {
                self.set_s("{{ field_uuid }}_date", date.to_string());
                self
            }
            "#,
        ),
        // set person field
        (
            "set_person_field",
            r#"
            {{#if field_single_value ~}}
            /// Sets person {{ field }}. {{#if is_update_builder}}Replaces previous value{{/if}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, id: ID) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_persons", json!(vec![id]));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            {{else ~}}
            /// Sets person(s) {{ field }}. {{#if is_update_builder}}Replaces previous value(s){{/if}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_persons", json!(ids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            {{/if ~}}

            {{#if is_update_builder ~}}
            {{#if field_multiple_value ~}}
            /// Adds person(s) {{ field }}
            /// {{field_desc}}
            pub fn add_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_persons", json!(ids));
                self.set_s("updateAction", "append".to_string());
                self
            }
            /// Remove person(s) {{ field }}
            /// {{field_desc}}
            pub fn remove_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_persons", json!(ids));
                self.set_s("updateAction", "remove".to_string());
                self
            }
            {{/if ~}}
            /// Remove person{{#if field_multiple_values}}(s){{/if}} {{ field }}
            /// {{field_desc}}
            pub fn unset_{{ to_snake_case field }}(&mut self) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_persons", json!(Vec::<String>::new()));
                self.set_s("updateAction", "replace".to_string());
                self
            }
            {{/if}}
            "#,
        ),
        // set references
        (
            "set_references",
            r#"
            {{#if field_single_value ~}}
            /// Sets reference {{ field }} to item in list {{ref_list}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, uuid: &str) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(vec![uuid]));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            {{else ~}}
            /// Sets reference {{ field }} to item in list {{ref_list}}. {{#if is_update}}Replaces previous value(s){{/if}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            {{/if ~}}

            {{#if is_update_builder ~}}
            {{#if field_multiple_value ~}}
            /// Adds references to {{ field }} to item(s) in list {{ref_list}}
            /// {{field_desc}}
            pub fn add_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "append".to_string());
                {{/if~}}
                self
            }

            /// Removes references to {{ field }}
            /// {{field_desc}}
            pub fn remove_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "remove".to_string());
                {{/if~}}
                self
            }
            {{/if ~}}

            /// Removes references to {{ field }}
            /// {{field_desc}}
            pub fn unset_{{ to_snake_case field }}(&mut self) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(Vec::<String>::new()));
                self.set_s("updateAction", "replace".to_string());
                self
            }
            {{/if ~}}
            "#,
        ),
        // set subitems
        (
            "set_subitems",
            r#"
            /// Sets subitems in {{ field }} hierarchy. {{#if is_update}}Replaces previous value(s){{/if}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }

            {{#if is_update_builder ~}}
            /// Adds subitems to {{ field }} hierarchy
            /// {{field_desc}}
            pub fn add_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "append".to_string());
                {{/if~}}
                self
            }

            /// Removes subitems in {{ field }} hierarchy
            /// {{field_desc}}
            pub fn remove_{{ to_snake_case field }}(&mut self, uuids: Vec<&'_ str>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(uuids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "remove".to_string());
                {{/if~}}
                self
            }

            /// Removes all subitems in {{ field }} hierarchy
            /// {{field_desc}}
            pub fn unset_{{ to_snake_case field }}(&mut self) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_references", json!(Vec::<String>::new()));
                self.set_s("updateAction", "replace".to_string());
                self
            }
            {{/if ~}}
            "#,
        ),
        // field-label-specific setter
        // set_category
        (
            "category_setter_per_label",
            r#"
            {{#if field_single_value ~}}
            /// Sets {{field}} to {{label}}. {{field_desc}}
            pub fn set_{{ to_snake_case field }}_{{ to_snake_case label }}(&mut self) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(vec![
                    {{ item}}::LABEL_{{ to_screaming_snake_case field }}_{{ to_screaming_snake_case label }}_ID
                ]));
                {{#if is_update_builder ~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if ~}}
                self
            }
            {{/if}}
            "#,
        ),
        // field category setter
        (
            "category_setter_per_field",
            r#"
            {{#if field_single_value ~}}
            /// Set {{ field }} by label-id. {{field_desc}}
            pub fn set_{{ to_snake_case field }}_id(&mut self, id: ID) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(vec![id]));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            /// Set {{ field }} by label-name. {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, label: &str) -> &mut {{ builder }} {
                match lookup_label(&{{ item }}::LABELS_{{ to_screaming_snake_case field }},label) {
                        Some(id) => {
                            self.set_v("{{ field_uuid }}_categories", json!(vec![id]));
                            {{#if is_update_builder~}}
                            self.set_s("updateAction", "replace".to_string());
                            {{/if~}}
                        },
                        None => {
                            self.errs.push(format!("Label '{}' not found for set_{{ to_snake_case field }}_label",
                                    label));
                        }
                }
                self
            }
            {{else ~}}
            /// Sets {{ field }} with label ids. {{#if is_update_builder}}Replaces any previous values{{/if}}
            /// {{field_desc}}
            pub fn set_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(ids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "replace".to_string());
                {{/if~}}
                self
            }
            {{/if ~}}

            {{#if is_update_builder ~}}
            {{#if field_multiple_value ~}}
            /// Adds label ids to {{field}}. {{#if is_update_builder}}Appends any previous values{{/if}}
            /// {{field_desc}}
            pub fn add_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(ids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "append".to_string());
                {{/if~}}
                self
            }
            /// Removes label ids from {{field}}.
            /// {{field_desc}}
            pub fn remove_{{ to_snake_case field }}(&mut self, ids: Vec<ID>) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(ids));
                {{#if is_update_builder~}}
                self.set_s("updateAction", "remove".to_string());
                {{/if~}}
                self
            }
            {{/if ~}}

            /// Clears (unsets) {{ field }} {{field_desc}}
            pub fn unset_{{ to_snake_case field }}(&mut self) -> &mut {{ builder }} {
                self.set_v("{{ field_uuid }}_categories", json!(Vec::<String>::new()));
                self.set_s("updateAction", "replace".to_string());
                self
            }
            {{/if ~}}
            "#,
        ),
        // execute - final builder method
        (
            "builder_execute",
            r#"
            {{#if is_update_builder ~}}
            /// Sends update request to server, returning updated {{item}}.
            {{else ~}}
            /// Sends create request to server, returning new {{item}}.
            {{/if ~}}
            pub async fn execute(&mut self) -> Result<{{ item }}, Error> {
                if !self.errs.is_empty() {
                    return Err(Error::Message(
                            format!("Errors occurred in {{ builder }}: {:?}", &self.errs)));
                }
                let map = self.fields.to_owned();
                let {{ to_snake_case item }} = get_api()?
                {{#if is_update_builder}}
                    .update_entry({{ list_id }}, self.item_id, Value::Object(map))
                {{else}}
                    .create_entry({{ list_id }}, Value::Object(map))
                {{/if}}
                    .await?;
                Ok({{item }}::new(Rc::new({{ to_snake_case item }})))
            }
            "#,
        ),
        // Cargo.toml
        (
            "cargo_toml",
            r#"# Cargo.toml
# {{ generated_banner }}
[package]
name = "{{ crate }}"
version = "0.1.0"
authors = ["author <mail@example.com>"]
edition = "2018"
description = "Zenkit client library for {{ workspace }}"
keywords = ["zenkit","bindings"]
categories = ["api-bindings"]

[dependencies]
serde_json = "1.0"
zenkit = { version="0.5" }
reqwest = { version="0.11", features=["json"] }

[lib]
path = "src/lib.rs"
"#,
        ),
    ];

    for (tname, tstr) in templates.iter() {
        hb.register_template_string(tname, tstr)?;
    }
    Ok(())
}

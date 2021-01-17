CHANGELOG for zenkit-codegen https://github.com/stevelr/zenkit-codegen

v0.5.0 2021-01-17

- fix: fixed label lookup
- fix: set_choice_label(&str) renamed to set_choice,
       and set_choice(id) renamed to set_choice_id,
       so they are consistent with getter naming convention,

v0.4.1 2021-01-12

- update dependencies to zenkit-0.5, tokio-1.0, reqwest-0.11
- generates Cargo.toml if it didn't already exist
  (if one exists in target directory, uses alternate name)
- adds comment to top of generated files with timestamp
- in generated files, use zenkit-0.5, reqwest-0.11
- add more comments
- remove dead code


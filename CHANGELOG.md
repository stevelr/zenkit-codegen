CHANGELOG for zenkit-codegen https://github.com/stevelr/zenkit-codegen

v0.5.4
- added get_zenkit_url function for list items
- added const LIST_*_SHORT_ID for each list
- updated dependencies (config 0.11, handlebars_misc_helpers 0.11)


v0.5.3

- generated initialize_zenkit_api returns ApiClient instead of ()
- integrated config crate for config handling 
  There are now three ways to specify the parameters for api token and workspace;
  - in a config file, using the `-c CONFIG_FILE` option formatted in TOML like this:
```toml
[zenkit]
token = "0000"
workspace = "My Workspace"
```
  - in the environment, as ZENKIT_TOKEN and ZENKIT_WORKSPACE, respectively.
    The variable name ZENKIT_API_TOKEN is deprecated.
  - on the command line with  -t/--token and -w/--workspace.  Passing
    secrets on the command line is usually discouraged for security
    reasons.
  - updated dependencies (bytes-1.0, strum-macros-0.20)


v0.5.2 2021-01-27

- rebuilt with latest zenkit 0.6.1, which includes fix for
  parsing date fields with no time

v0.5.1 2021-01-23

- update dependency to zenkit-0.6
- added License files to repo. License (MIT or Apache-2.0) is unchanged.

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


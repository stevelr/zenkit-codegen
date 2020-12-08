Code generator to create Zenkit client library in Rust, based on
user-defined workspace schema. Like ORM for Zenkit.

Greatly simplifies writing reliable and performant Rust clients for Zenkit - can
substantially reduce (by more than 50%) the amount of code required 
to interact with Zenkit, and improves code reliability by
leveraging Rust's compile-time syntax and type validation
to ensure type-safe use of business objects.

AFAIK, this is the only Zenkit client library that can detect
schema-related code bugs at compile time, rather than at runtime.
If Zenkit workspace "schema" changes, such as, by changing the name of a
field, or changing a field type from integer to text,
running zk-codegen and recompiling the client app will cause the
compiler to flag any places where the app has a dependency
on the old field definition.

## Usage

Install with `cargo install zenkit-codegen`. The program name is
`zk-codegen`.

Set the environment variable `ZENKIT_API_TOKEN` to your api token,
which you can obtain from your account profile on zenkit.com.

```sh
# Generate source code
# Parameters are -o output_dir (will be created)
#                -w workspace_name
zk-codegen -o my-lib -w "My Workspace"

# Cargo.toml is generated with the ".sample" suffix
# so that regenerating code doesn't overwrite any edits to Cargo.toml.
cp Cargo.toml.sample Cargo.toml

# Compile your new library!
cargo build
```

If you get any errors during code generation or compilation, it's a bug.
Please file a github issue.


## Examples

There are some sample programs in the examples dir that use some of the
Zenkit workspace templates.

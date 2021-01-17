Generate Zenkit ORM client library in Rust, based on
user-defined workspace schema.

The output of this program is a new library crate that can be
imported into a Rust app, or compiled into WASM.

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
# Parameters are -o output_dir (will be created if it doesn't exist)
#                -w workspace_name
zk-codegen -o my-lib -w "My Workspace"

# The command above generates output_dir/Cargo.toml and
# output_dir/src/*.rs. The crate should build as-is
cd output_dir
cargo build
```

On subsequent runs, any files in src/ are overwritten, so that they
reflect the current schema at the time zk-codegen was run,
but, to avoid overwriting manual updates to Cargo.toml, 
new versions of that file will be saved as Cargo.toml.gen

If you get any errors during code generation or compilation, it's a bug
in this crate. Please file a github issue.


## Examples

There are some sample programs in the examples dir that use some of the
Zenkit workspace templates.

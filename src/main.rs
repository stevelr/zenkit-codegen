use clap::Clap;
use zenkit::{init_api, ApiConfig};
use config::Config;

mod error;
use error::Error;
mod gen;
use gen::Generator;
mod templates;

/// Zenkit Rust-client code generator. Source and docs at https://github.com/stevelr/zenkit-codegen
#[derive(Clap, PartialEq, Debug)]
#[clap(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"))]
struct Opt {
    /// API token. For protection of secrets such as token, passing on the command line
    /// is discouraged. It can be set in a config file (using -c) option,
    /// or in an environment variable `ZENKIT_TOKEN` (or (deprecated) `ZENKIT_API_TOKEN`)
    #[clap(short, long)]
    token: Option<String>,

    /// Workspace name, id, or uuid. Can be set in config file
    /// or in environment variable ZENKIT_WORKSPACE
    #[clap(short, long)]
    workspace: Option<String>,

    /// path to config file containing keys zenkit.token and zenkit.workspace
    /// Example (TOML format):
    /// ```toml
    /// [zenkit]
    /// token = "000000"
    /// workspace = "My Workspace"
    /// ```
    #[clap(short, long)]
    config: Option<String>,

    /// Test builder
    #[clap(long)]
    build: bool,

    /// Output directory for generated file
    #[clap(short, long)]
    output: String,
}

/// main entry point
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opt = Opt::parse();
    if let Err(e) = run(opt).await {
        eprintln!("Error: {:#?}", e);
        std::process::exit(1);
    }
}

async fn run(opt: Opt) -> Result<(), Error> {

    let settings = load_config(opt.config)?;

    // attempt to create output directory with src subdirectory
    let src_dir = format!("{}/src", &opt.output);
    std::fs::create_dir_all(&src_dir)?;

    let token = match settings.get_str("zenkit.token") {
        Ok(token) => token,
        Err(_) => settings.get_str("zenkit.api.token")
            .map_err(|_| Error::Message(
                "Missing zenkit token. add to config file with `-c` option or set in environment as ZENKIT_TOKEN".into()))?,
    };
    let api = init_api(ApiConfig { token, ..Default::default() })?;
    let workspace = match opt.workspace {
        Some(name) => name,
        None => settings.get_str("zenkit.workspace").map_err(|_| Error::Message(
                "Workspace must be specified in config file with `-c` option or in environment as ZENKIT_WORKSPACE".into())
        )?,
    };
    let ws = api.get_workspace(&workspace).await?;

    let mut gen = Generator::init()?;
    let files = gen.gen_workspace(&api, ws, &opt.output).await?;

    // Run rustfmt
    format_results(files)?;
    Ok(())
}

/// Build config from
///  - cli option "-c CONFIG-FILE"
///  - environment overrides of the form "ZENKIT_"
fn load_config(file: Option<String>) -> Result<Config, Error> {
    let mut settings = Config::default();
    if let Some(opt_path) = file {
        settings.merge(config::File::with_name(&opt_path))?;
    }
    settings.merge(config::Environment::with_prefix("zenkit").separator("_"))?;
    Ok(settings)
}

fn format_results(files: Vec<String>) -> Result<(), Error> {
    use std::process::Command;

    let mut args = vec!["-v", "--edition", "2018"];
    // include names of generated rust source files
    args.extend(
        files
            .iter()
            .filter(|f| f.ends_with(".rs"))
            .map(|s| s.as_str()),
    );
    let mut child_proc = Command::new("rustfmt").args(args).spawn()?;
    let rc = child_proc.wait()?;
    if !rc.success() {
        return Err(Error::Message("rustfmt exited with error".to_string()));
    }
    Ok(())
}

use clap::Clap;
use zenkit::{init_api, ApiConfig};

mod error;
use error::Error;
mod gen;
use gen::Generator;

mod templates;

// name of the environment variable holding the token
const ZENKIT_API_TOKEN_VAR: &str = "ZENKIT_API_TOKEN";

/// Zenkit CLI
#[derive(Clap, PartialEq, Debug)]
#[clap(name = "zk-codegen", version = "0.1")]
struct Opt {
    /// API token. Defaults to environment var ZENKIT_API_TOKEN
    #[clap(short, long)]
    token: Option<String>,

    /// Workspace name, id, or uuid.
    #[clap(short, long)]
    workspace: String,

    /// Test builder
    #[clap(long)]
    build: bool,

    /// Output directory for generated file
    #[clap(short, long)]
    output: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    if let Err(e) = run(opt).await {
        eprintln!("Error: {:#?}", e);
        std::process::exit(1);
    }
}

async fn run(opt: Opt) -> Result<(), Error> {
    // attempt to create output directory with src subdirectory
    let src_dir = format!("{}/src", &opt.output);
    std::fs::create_dir_all(&src_dir)?;

    let token = match opt.token {
        Some(token) => token,
        None => std::env::var(&ZENKIT_API_TOKEN_VAR)
            .map_err(|_| Error::Message(format!("Missing env var {}", ZENKIT_API_TOKEN_VAR)))?,
    };
    let api = init_api(ApiConfig {
        token,
        ..Default::default()
    })?;
    let ws = api.get_workspace(&opt.workspace).await?;

    let mut gen = Generator::init()?;
    let files = gen.gen_workspace(&api, ws, &opt.output).await?;

    // Run rustfmt
    format_results(files)?;
    Ok(())
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

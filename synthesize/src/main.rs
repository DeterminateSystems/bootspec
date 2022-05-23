use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use bootspec::generation::Generation;
use bootspec::v1;
use bootspec::Result;

#[derive(clap::Parser)]
struct Cli {
    generation_dir: PathBuf,
    out_path: PathBuf,
    #[clap(long)]
    version: u64,
}

fn main() -> Result<()> {
    if let Err(e) = self::cli() {
        writeln!(io::stderr(), "{}", e)?;

        std::process::exit(1);
    }

    Ok(())
}

fn cli() -> Result<()> {
    let args: Cli = clap::Parser::parse();
    let generation_dir = args.generation_dir;
    let out_path = args.out_path;
    let version = args.version;

    let versioned_spec = match version {
        v1::SCHEMA_VERSION => {
            let spec = v1::GenerationV1::synthesize(&generation_dir).map_err(|e| {
                format!(
                    "Failed to synthesize v{} bootspec for {}:\n{}",
                    version,
                    generation_dir.display(),
                    e
                )
            })?;

            Generation::V1(spec)
        }
        v => return Err(format!("Cannot synthesize for unsupported schema version {}", v).into()),
    };

    let pretty = serde_json::to_string_pretty(&versioned_spec)
        .map_err(|e| format!("Failed to make pretty JSON from bootspec:\n{}", e))?;

    fs::write(&out_path, pretty)
        .map_err(|e| format!("Failed to write JSON to '{}':\n{}", out_path.display(), e))?;

    Ok(())
}

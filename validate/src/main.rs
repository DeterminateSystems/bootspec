use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use bootspec::BootJson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = self::cli() {
        writeln!(io::stderr(), "{}", e)?;

        std::process::exit(1);
    }

    Ok(())
}

fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let Args { bootspec_path } = parse_args()?;

    let contents = fs::read_to_string(&bootspec_path)?;

    match serde_json::from_str::<BootJson>(&contents) {
        Ok(boot_json) => {
            let generation = boot_json.generation;
            writeln!(
                io::stdout(),
                "Bootspec document at '{}' DOES CONTAIN a valid v{} document.",
                bootspec_path.display(),
                generation.version()
            )?;
        }
        Err(err) => {
            return Err(format!(
                "Bootspec document at '{}' DOES NOT CONTAIN a valid document:\n{}",
                bootspec_path.display(),
                err
            )
            .into())
        }
    }

    Ok(())
}

pub struct Args {
    pub bootspec_path: PathBuf,
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if args.len() != 1 {
        writeln!(io::stderr(), "Usage: validate <bootspec_path>")?;
        std::process::exit(1);
    }

    let bootspec_path = args
        .next()
        .ok_or("Expected path to bootspec document, got none.")?
        .parse::<PathBuf>()?;

    Ok(Args { bootspec_path })
}

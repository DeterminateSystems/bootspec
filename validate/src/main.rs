use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use bootspec::generation::Generation;
use bootspec::Result;

fn main() -> Result<()> {
    if let Err(e) = self::cli() {
        writeln!(io::stderr(), "{}", e)?;

        std::process::exit(1);
    }

    Ok(())
}

fn cli() -> Result<()> {
    let Args { bootspec_path } = parse_args()?;

    let contents = fs::read_to_string(&bootspec_path)?;

    match serde_json::from_str::<Generation>(&contents) {
        Ok(generation) => writeln!(
            io::stdout(),
            "Bootspec document at '{}' IS a valid v{} document.",
            bootspec_path.display(),
            generation.version()
        )?,
        Err(err) => {
            return Err(format!(
                "Bootspec document at '{}' IS NOT a valid document:\n{}",
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

fn parse_args() -> Result<Args> {
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

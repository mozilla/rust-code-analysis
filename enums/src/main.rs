use std::path::PathBuf;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

use enums::*;

#[derive(Debug, Clone)]
enum OutputLanguage {
    Rust,
    Go,
    Json,
    CMacros,
}

impl std::str::FromStr for OutputLanguage {
    type Err = &'static str;

    fn from_str(env: &str) -> std::result::Result<Self, Self::Err> {
        match env {
            "rust" => Ok(Self::Rust),
            "go" => Ok(Self::Go),
            "json" => Ok(Self::Json),
            "c_macros" => Ok(Self::CMacros),
            _ => Err("Not a valid value, run `--help` to know valid values"),
        }
    }
}

impl OutputLanguage {
    const fn variants() -> [&'static str; 4] {
        ["rust", "go", "json", "c_macros"]
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "enums",
    version,
    author,
    about = "Generate enums for a target language to use with tree-sitter."
)]
struct Opts {
    /// Output directory.
    #[clap(long, short, default_value = ".", value_parser)]
    output: PathBuf,
    /// Target language.
    #[clap(long, short, default_value = "rust", value_parser = PossibleValuesParser::new(OutputLanguage::variants())
        .map(|s| s.parse::<OutputLanguage>().unwrap()))]
    language: OutputLanguage,
    /// File name template.
    #[clap(long, short, default_value = "language_$")]
    file_template: String,
}

fn main() {
    let opts = Opts::parse();

    match opts.language {
        OutputLanguage::Rust => {
            if let Some(err) = generate_rust(&opts.output, &opts.file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        OutputLanguage::Go => {
            if let Some(err) = generate_go(&opts.output, &opts.file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        OutputLanguage::Json => {
            if let Some(err) = generate_json(&opts.output, &opts.file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        OutputLanguage::CMacros => {
            if let Some(err) = generate_macros(&opts.output).err() {
                eprintln!("{:?}", err);
            }
        }
    }
}

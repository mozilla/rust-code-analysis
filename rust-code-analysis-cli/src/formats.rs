use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

use rust_code_analysis::FuncSpace;

#[derive(Debug, Clone)]
pub enum Format {
    Cbor,
    Json,
    Toml,
    Yaml,
}

impl Format {
    pub fn all() -> &'static [&'static str] {
        &["cbor", "json", "toml", "yaml"]
    }

    pub fn dump_formats(
        &self,
        space: &FuncSpace,
        path: &PathBuf,
        output_path: &Option<PathBuf>,
        pretty: bool,
    ) -> std::io::Result<()> {
        if output_path.is_none() {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();

            match self {
                Format::Cbor => Err(Error::new(
                    ErrorKind::Other,
                    "Cbor format cannot be printed to stdout",
                )),
                Format::Json => {
                    let json_data = if pretty {
                        serde_json::to_string_pretty(&space).unwrap()
                    } else {
                        serde_json::to_string(&space).unwrap()
                    };
                    writeln!(stdout, "{}", json_data)
                }
                Format::Toml => {
                    let toml_data = if pretty {
                        toml::to_string_pretty(&space).unwrap()
                    } else {
                        toml::to_string(&space).unwrap()
                    };
                    writeln!(stdout, "{}", toml_data)
                }
                Format::Yaml => writeln!(stdout, "{}", serde_yaml::to_string(&space).unwrap()),
            }
        } else {
            let format_ext = match self {
                Format::Cbor => ".cbor",
                Format::Json => ".json",
                Format::Toml => ".toml",
                Format::Yaml => ".yml",
            };

            let output_path = output_path.as_ref().unwrap();

            let mut file = path.as_path().file_name().unwrap().to_os_string();
            file.push(format_ext);

            let mut format_path = output_path.clone();
            format_path.push(file);

            if format_path.as_path().exists() {
                let mut new_filename = path.to_str().unwrap().to_string();
                let re = Regex::new(r"[\\:/]").unwrap();
                new_filename = re.replace_all(&new_filename, "_").to_string();
                new_filename.push_str(format_ext);
                format_path.pop();
                format_path.push(new_filename);
            }

            let mut format_file = File::create(format_path)?;
            match self {
                Format::Cbor => serde_cbor::to_writer(format_file, &space)
                    .map_err(|e| Error::new(ErrorKind::Other, e.to_string())),
                Format::Json => {
                    if pretty {
                        serde_json::to_writer_pretty(format_file, &space)
                            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
                    } else {
                        serde_json::to_writer(format_file, &space)
                            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
                    }
                }
                Format::Toml => {
                    let toml_data = if pretty {
                        toml::to_string_pretty(&space).unwrap()
                    } else {
                        toml::to_string(&space).unwrap()
                    };
                    format_file.write_all(toml_data.as_bytes())
                }
                Format::Yaml => serde_yaml::to_writer(format_file, &space)
                    .map_err(|e| Error::new(ErrorKind::Other, e.to_string())),
            }
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "cbor" => Ok(Format::Cbor),
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            format => Err(format!("{:?} is not a supported format", format)),
        }
    }
}

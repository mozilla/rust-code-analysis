use std::fs::{create_dir_all, File};
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Format {
    Cbor,
    Json,
    Toml,
    Yaml,
}

impl Format {
    pub const fn all() -> &'static [&'static str] {
        &["cbor", "json", "toml", "yaml"]
    }

    pub fn dump_formats<T: Serialize>(
        &self,
        space: T,
        path: PathBuf,
        output_path: Option<&PathBuf>,
        pretty: bool,
    ) -> std::io::Result<()> {
        if let Some(output_path) = output_path {
            let format_path = self.create_unique_output_filename(path, output_path);
            self.dump_on_file(space, format_path, pretty)
        } else {
            self.dump_stdout(space, pretty)
        }
    }

    fn dump_stdout<T: Serialize>(&self, space: T, pretty: bool) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        match self {
            Self::Cbor => Err(Error::new(
                ErrorKind::Other,
                "Cbor format cannot be printed to stdout",
            )),
            Self::Json => {
                let json_data = if pretty {
                    serde_json::to_string_pretty(&space).unwrap()
                } else {
                    serde_json::to_string(&space).unwrap()
                };
                writeln!(stdout, "{json_data}")
            }
            Self::Toml => {
                let toml_data = if pretty {
                    toml::to_string_pretty(&space).unwrap()
                } else {
                    toml::to_string(&space).unwrap()
                };
                writeln!(stdout, "{toml_data}")
            }
            Self::Yaml => writeln!(stdout, "{}", serde_yaml::to_string(&space).unwrap()),
        }
    }

    fn dump_on_file<T: Serialize>(
        &self,
        space: T,
        path: PathBuf,
        pretty: bool,
    ) -> std::io::Result<()> {
        let mut format_file = File::create(path)?;
        match self {
            Self::Cbor => serde_cbor::to_writer(format_file, &space)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string())),
            Self::Json => {
                if pretty {
                    serde_json::to_writer_pretty(format_file, &space)
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
                } else {
                    serde_json::to_writer(format_file, &space)
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
                }
            }
            Self::Toml => {
                let toml_data = if pretty {
                    toml::to_string_pretty(&space).unwrap()
                } else {
                    toml::to_string(&space).unwrap()
                };
                format_file.write_all(toml_data.as_bytes())
            }
            Self::Yaml => serde_yaml::to_writer(format_file, &space)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string())),
        }
    }

    fn create_unique_output_filename(&self, path: PathBuf, output_path: &Path) -> PathBuf {
        let format_ext = match self {
            Self::Cbor => ".cbor",
            Self::Json => ".json",
            Self::Toml => ".toml",
            Self::Yaml => ".yml",
        };

        // Remove root /
        let path = path.as_path().strip_prefix("/").unwrap_or(path.as_path());

        // Remove root ./
        let path = path.strip_prefix("./").unwrap_or(path);

        // Replace .. with . to keep files inside the output folder
        let cleaned_path: Vec<&str> = path
            .iter()
            .map(|os_str| {
                let s_str = os_str.to_str().unwrap();
                if s_str == ".." {
                    "."
                } else {
                    s_str
                }
            })
            .collect();

        // Create the filename
        let filename = cleaned_path.join("/") + format_ext;

        // Build the file path
        let format_path = output_path.join(filename);

        // Create directories
        create_dir_all(format_path.parent().unwrap()).unwrap();

        format_path
    }
}

fn print_on_stdout(content: String) {
    writeln!(std::io::stdout().lock(), "{content}").unwrap();
}

trait WriteOnStdout {
    fn write_on_stdout<T: Serialize>(content: T) {
        print_on_stdout(Self::format(content));
    }

    fn format<T: Serialize>(content: T) -> String;
}

trait WritePrettyOnStdout: WriteOnStdout {
    fn write_on_stdout_pretty<T: Serialize>(content: T, pretty: bool) {
        print_on_stdout(if pretty {
            Self::format_pretty(content)
        } else {
            Self::format(content)
        });
    }
    fn format_pretty<T: Serialize>(content: T) -> String;
}

fn handle_path(path: PathBuf, output_path: &Path, extension: &str) -> PathBuf {
    // Remove root /
    let path = path.as_path().strip_prefix("/").unwrap_or(path.as_path());

    // Remove root ./
    let path = path.strip_prefix("./").unwrap_or(path);

    // Replace .. with . to keep files inside the output folder
    let cleaned_path: Vec<&str> = path
        .iter()
        .map(|os_str| {
            let s_str = os_str.to_str().unwrap();
            if s_str == ".." {
                "."
            } else {
                s_str
            }
        })
        .collect();

    // Create the filename
    let filename = cleaned_path.join("/") + extension;

    // Build the file path
    output_path.join(filename)
}

trait WriteFile {
    const EXTENSION: &'static str;

    fn open_file(path: PathBuf, output_path: &Path) -> File {
        // Handle output path
        let format_path = handle_path(path, output_path, Self::EXTENSION);

        // Create directories
        create_dir_all(format_path.parent().unwrap()).unwrap();

        File::create(format_path).unwrap()
    }

    fn with_writer<T: Serialize>(content: T, path: PathBuf, output_path: &Path);
}

trait WritePrettyFile: WriteFile {
    fn with_pretty_writer<T: Serialize>(
        content: T,
        path: PathBuf,
        output_path: &Path,
        pretty: bool,
    );
}

struct Json;

impl WriteOnStdout for Json {
    fn format<T: Serialize>(content: T) -> String {
        serde_json::to_string(&content).unwrap()
    }
}

impl WritePrettyOnStdout for Json {
    fn format_pretty<T: Serialize>(content: T) -> String {
        serde_json::to_string_pretty(&content).unwrap()
    }
}

impl WriteFile for Json {
    const EXTENSION: &'static str = ".json";

    fn with_writer<T: Serialize>(content: T, path: PathBuf, output_path: &Path) {
        serde_json::to_writer(Self::open_file(path, output_path), &content).unwrap()
    }
}

impl WritePrettyFile for Json {
    fn with_pretty_writer<T: Serialize>(
        content: T,
        path: PathBuf,
        output_path: &Path,
        pretty: bool,
    ) {
        if pretty {
            serde_json::to_writer_pretty(Self::open_file(path, output_path), &content).unwrap();
        } else {
            Self::with_writer(content, path, output_path);
        }
    }
}

struct Toml;

impl WriteOnStdout for Toml {
    fn format<T: Serialize>(content: T) -> String {
        toml::to_string(&content).unwrap()
    }
}

impl WritePrettyOnStdout for Toml {
    fn format_pretty<T: Serialize>(content: T) -> String {
        toml::to_string_pretty(&content).unwrap()
    }
}

impl WriteFile for Toml {
    const EXTENSION: &'static str = ".toml";

    fn with_writer<T: Serialize>(content: T, path: PathBuf, output_path: &Path) {
        Self::open_file(path, output_path)
            .write_all(Self::format(content).as_bytes())
            .unwrap();
    }
}

impl WritePrettyFile for Toml {
    fn with_pretty_writer<T: Serialize>(
        content: T,
        path: PathBuf,
        output_path: &Path,
        pretty: bool,
    ) {
        if pretty {
            Self::open_file(path, output_path)
                .write_all(Self::format_pretty(&content).as_bytes())
                .unwrap();
        } else {
            Self::with_writer(content, path, output_path);
        }
    }
}

struct Yaml;

impl WriteOnStdout for Yaml {
    fn format<T: Serialize>(content: T) -> String {
        serde_yaml::to_string(&content).unwrap()
    }
}

impl WriteFile for Yaml {
    const EXTENSION: &'static str = ".yml";

    fn with_writer<T: Serialize>(content: T, path: PathBuf, output_path: &Path) {
        serde_yaml::to_writer(Self::open_file(path, output_path), &content).unwrap()
    }
}

struct Cbor;

impl WriteFile for Cbor {
    const EXTENSION: &'static str = ".cbor";

    fn with_writer<T: Serialize>(content: T, path: PathBuf, output_path: &Path) {
        serde_cbor::to_writer(Self::open_file(path, output_path), &content).unwrap()
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "cbor" => Ok(Self::Cbor),
            "json" => Ok(Self::Json),
            "toml" => Ok(Self::Toml),
            "yaml" => Ok(Self::Yaml),
            format => Err(format!("{format:?} is not a supported format")),
        }
    }
}

#[macro_use]
extern crate clap;

use clap::{App, Arg};

use enums::*;

fn main() {
    let matches = App::new("enums")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Generate enums for a target language to use with tree-sitter")
        .arg(
            Arg::with_name("output")
                .help("Sets the output directory")
                .short("o")
                .long("output")
                .value_name("OUTPUT DIR")
                .default_value(".")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("language")
                .help("Target language")
                .short("l")
                .long("language")
                .value_name("LANGUAGE")
                .default_value("rust")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file template")
                .help("File name template")
                .short("f")
                .long("file-template")
                .value_name("FILE TEMPLATE")
                .default_value("language_$")
                .takes_value(true),
        )
        .get_matches();

    let output = matches.value_of("output").unwrap();
    let language = matches.value_of("language").unwrap();
    let file_template = matches.value_of("file template").unwrap();

    match language {
        "rust" => {
            if let Some(err) = generate_rust(output, file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        "go" => {
            if let Some(err) = generate_go(output, file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        "json" => {
            if let Some(err) = generate_json(output, file_template).err() {
                eprintln!("{:?}", err);
            }
        }
        "c_macros" => {
            if let Some(err) = generate_macros(output).err() {
                eprintln!("{:?}", err);
            }
        }
        _ => {
            eprintln!("Invalid target language: {}", language);
        }
    }
}

// After adding new fields for min and max in the json (server.rs line 630) this error arose:
// error: recursion limit reached while expanding `json_internal!`
// This solution was proposed as help by the compiler
// for the full error details check here :https://github.com/mozilla/rust-code-analysis/pull/793#discussion_r817610530
#![recursion_limit = "256"]
mod web;

use clap::{crate_version, App, Arg};

use web::server;

fn main() {
    let matches = App::new("rust-code-analysis-web")
        .version(crate_version!())
        .author(&*env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
        .about("Run a web server")
        .arg(
            Arg::new("num_jobs")
                .help("Number of jobs")
                .short('j')
                .value_name("NUMBER")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("host")
                .help("Host for the web server")
                .long("host")
                .default_value("127.0.0.1")
                .takes_value(true),
        )
        .arg(
            Arg::new("port")
                .help("Port for the web server")
                .long("port")
                .default_value("8080")
                .takes_value(true),
        )
        .get_matches();

    let num_jobs = if let Ok(num_jobs) = matches.value_of("num_jobs").unwrap().parse::<usize>() {
        num_jobs
    } else {
        num_cpus::get()
    };

    let host = matches.value_of("host").unwrap();
    let port = if let Ok(port) = matches.value_of("port").unwrap().parse::<u16>() {
        port
    } else {
        eprintln!("Invalid port number");
        return;
    };
    if let Err(e) = server::run(host.to_string(), port, num_jobs) {
        eprintln!("Cannot run the server at {}:{}: {}", host, port, e);
    }
}

// After adding new fields for min and max in the json (server.rs line 630) this error arose:
// error: recursion limit reached while expanding `json_internal!`
// This solution was proposed as help by the compiler
// for the full error details check here :https://github.com/mozilla/rust-code-analysis/pull/793#discussion_r817610530
#![recursion_limit = "256"]
mod web;

use std::thread::available_parallelism;

use clap::Parser;

use web::server;

#[derive(Parser, Debug)]
#[clap(
    name = "rust-code-analysis-web",
    version,
    author,
    about = "Run a web server."
)]
struct Opts {
    /// Number of jobs.
    #[clap(long, short = 'j')]
    num_jobs: Option<usize>,
    /// Host for the web server.
    #[clap(long, short, default_value = "127.0.0.1")]
    host: String,
    /// Port for the web server.
    #[clap(long, short, default_value = "8080")]
    port: u16,
}

#[actix_web::main]
async fn main() {
    let opts = Opts::parse();

    let num_jobs = opts.num_jobs.unwrap_or_else(|| {
        available_parallelism()
            .expect("Unrecoverable: Failed to get thread count")
            .get()
    });

    if let Err(e) = server::run(&opts.host, opts.port, num_jobs).await {
        eprintln!(
            "Cannot run the server at {}:{}: {}",
            opts.host, opts.port, e
        );
    }
}

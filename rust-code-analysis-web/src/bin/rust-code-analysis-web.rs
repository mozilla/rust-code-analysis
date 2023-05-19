use std::thread::available_parallelism;

use clap::Parser;

use rust_code_analysis_web::server::run;

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
    #[clap(long, default_value = "127.0.0.1")]
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

    if let Err(e) = run(&opts.host, opts.port, num_jobs).await {
        eprintln!(
            "Cannot run the server at {}:{}: {}",
            opts.host, opts.port, e
        );
    }
}

mod cache;
mod cache_scanner;
mod cli;
mod cleaner;
mod output;
mod project;
mod scanner;
mod script;
mod web;

use anyhow::Result;
use cli::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.serve {
        // Launch web UI server
        web::start_server(args).await?;
    } else {
        // Run CLI cleaner
        cleaner::run(args)?;
    }

    Ok(())
}

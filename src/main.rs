mod cache;
mod cache_scanner;
mod cli;
mod cleaner;
mod output;
mod project;
mod scanner;
mod script;

use anyhow::Result;
use cli::Args;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    // Run the cleaner with parsed arguments
    cleaner::run(args)?;

    Ok(())
}

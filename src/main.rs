// src/main.rs
/*
 * Main executable for TradingBot
 */

use clap::Parser;
use tradingbot::{Result, run};

/// CLI arguments for TradingBot
#[derive(Parser)]
#[command(version, about = "TradingBot - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to input file
    #[arg(short, long)]
    input: Option<String>,
    
    /// Path to output file
    #[arg(short, long)]
    output: Option<String>,
}

/// Main entry point of the program
fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();
    
    // Run the TradingBot with the parsed arguments
    run(args.verbose, args.input, args.output)
}
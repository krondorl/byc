// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::Result;
use byc::cli::commands::{calculate_enriched_bonds, print_info, read_bonds, write_enriched_bonds};
use clap::{CommandFactory, Parser};

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " — ",
    env!("CARGO_PKG_DESCRIPTION"),
    "\nBatch processing tool for bond yield calculations.",
);

/// Calculate bond yields from given file
#[derive(Parser)]
#[command(version = VERSION_INFO)]
#[command(
    long_about = "This tool calculates bond yields from a given input file.",
    after_help = "Example usage:\n  byc input.csv output.csv"
)]
struct Cli {
    /// Input file path to process
    input_file: std::path::PathBuf,
    /// Path to save the processed output file
    output_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let cmd = Cli::command();

    print_info(cmd);

    println!("Reading bonds from: {:?}", args.input_file);
    let bonds = read_bonds(args.input_file.to_str().unwrap())?;

    let enriched_bonds = calculate_enriched_bonds(&bonds)?;

    println!("Saving results to: {:?}", args.output_file);
    write_enriched_bonds(args.output_file.to_str().unwrap(), &enriched_bonds)?;

    Ok(())
}

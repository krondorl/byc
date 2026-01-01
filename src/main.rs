// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};

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

fn main() {
    println!("byc");

    // parse CLI args
    // load inputs
    // call pure math
    // print results
}

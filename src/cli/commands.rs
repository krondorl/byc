// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use crate::{
    domain::{bond::Bond, enriched_bond::EnrichedBond},
    math::yielder::{current_yield, simple_yield},
    time::maturity::years_to_maturity_at,
};
use anyhow::Context;
use std::fs::File;

/// Displays the application's name, version, and description to the standard output.
///
/// This is typically used at the start of the program or when the `--version`
/// flag is triggered to provide a consistent branding header.
///
/// ### Example Output
/// `byc 0.1.0 — Batch processing tool for bond yield calculations.`
pub fn print_info(cmd: clap::Command) {
    let about = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();
    println!(
        "{} {} — {}",
        cmd.get_name(),
        cmd.get_version().unwrap_or(""),
        about
    );
    println!();
}

/// Reads bonds from a CSV file.
/// Read bonds don't contain yield information.
/// Those will be calculated later.
pub fn read_bonds(path: &str) -> anyhow::Result<Vec<Bond>> {
    let file = File::open(path).context(format!("Failed to open file at {}", path))?;

    let mut reader = csv::Reader::from_reader(file);
    let mut bonds = Vec::new();

    for result in reader.deserialize() {
        let record: Bond = result.context("Failed to deserialize CSV row")?;
        bonds.push(record);
    }

    Ok(bonds)
}

/// Process bonds and calculate enriched bonds information.
/// clone() is used for simplicity to avoid ownership issues.
/// If performance becomes a concern, using references or smarter ownership handling can help.
pub fn calculate_enriched_bonds(bonds: &[Bond]) -> anyhow::Result<Vec<EnrichedBond>> {
    bonds
        .iter()
        .map(|bond| {
            let years = years_to_maturity_at(&bond.maturity_date, chrono::Utc::now())
                .with_context(|| format!("failed for ISIN {}", bond.isin))?;

            Ok(EnrichedBond {
                isin: bond.isin.clone(),
                ticker: bond.ticker.clone(),
                coupon_rate: bond.coupon_rate,
                maturity_date: bond.maturity_date.clone(),
                clean_price: bond.clean_price,
                current_yield: current_yield(bond),
                simple_yield: simple_yield(bond, years),
            })
        })
        .collect()
}

/// Writes enriched bonds to a CSV file.
/// Enriched bonds contain calculated yield information.
pub fn write_enriched_bonds(path: &str, data: &[EnrichedBond]) -> anyhow::Result<()> {
    let file = File::create(path).with_context(|| format!("Failed to create file at {}", path))?;

    let mut writer = csv::Writer::from_writer(file);

    for enriched_bond in data {
        writer
            .serialize(enriched_bond)
            .context("Failed to serialize record")?;
    }

    writer.flush()?;
    Ok(())
}

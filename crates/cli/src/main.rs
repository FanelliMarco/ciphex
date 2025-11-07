use anyhow::Result;
use ciphex_detectors::UniversalDetector;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "ciphex")]
#[command(about = "A cryptographic cipher and hash identifier", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Identify the type of cipher, hash, or encoding
    Identify {
        /// The text to identify
        input: String,

        /// Show all possible matches
        #[arg(short, long)]
        all: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Show detailed evidence
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Identify {
            input,
            all,
            json,
            verbose,
        } => {
            if all {
                let results = UniversalDetector::detect_all(&input);

                if json {
                    println!("{}", serde_json::to_string_pretty(&results)?);
                } else {
                    print_all_results(&results, verbose);
                }
            } else {
                let result = UniversalDetector::detect(&input);

                if json {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                } else {
                    print_single_result(&result, verbose);
                }
            }
        }
    }

    Ok(())
}

fn print_single_result(result: &ciphex_core::DetectionResult, verbose: bool) {
    let confidence_color = if result.confidence > 0.85 {
        "green"
    } else if result.confidence > 0.70 {
        "yellow"
    } else {
        "red"
    };

    println!(
        "{}: {} (confidence: {})",
        "Detected".bold(),
        format!("{}", result.cipher_type)
            .color(confidence_color)
            .bold(),
        format!("{:.0}%", result.confidence * 100.0).color(confidence_color)
    );

    if verbose && !result.evidence.is_empty() {
        println!("\n{}", "Evidence:".bold());
        for evidence in &result.evidence {
            println!("  • {}", evidence);
        }
    }
}

fn print_all_results(results: &[ciphex_core::DetectionResult], verbose: bool) {
    println!("{}", "Possible matches (sorted by confidence):".bold());
    println!();

    for (i, result) in results.iter().enumerate() {
        let confidence_color = if result.confidence > 0.85 {
            "green"
        } else if result.confidence > 0.70 {
            "yellow"
        } else {
            "red"
        };

        println!(
            "{}. {} - {}",
            i + 1,
            format!("{}", result.cipher_type)
                .color(confidence_color)
                .bold(),
            format!("{:.0}%", result.confidence * 100.0).color(confidence_color)
        );

        if verbose && !result.evidence.is_empty() {
            for evidence in &result.evidence {
                println!("   • {}", evidence.dimmed());
            }
        }
        println!();
    }
}

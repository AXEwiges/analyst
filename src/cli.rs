use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Analyze missing values in the CSV file
    MissingValues {
        /// Path to the CSV file
        file_path: String,
        /// Optional column name to analyze
        #[arg(short, long)]
        column: Option<String>,
    },
    /// Find frequent patterns in the CSV data
    FrequentPatterns {
        /// Path to the CSV file
        file_path: String,
        /// Minimum support threshold for frequent patterns
        #[arg(short, long, default_value = "0.1")]
        min_support: f64,
    },
    /// Calculate statistics for columns in the CSV file
    ColumnStats {
        /// Path to the CSV file
        file_path: String,
        /// Optional column name to analyze
        #[arg(short, long)]
        column: Option<String>,
    },
    /// Find extrema (min/max) values for a specific column
    Extrema {
        /// Path to the CSV file
        file_path: String,
        /// Column name to analyze
        #[arg(short, long)]
        column: String,
    },
    /// Show a range of rows from the CSV file
    Show {
        /// Path to the CSV file
        file_path: String,
        /// Start row number (inclusive)
        #[arg(short, long, default_value = "1")]
        start: usize,
        /// End row number (inclusive)
        #[arg(short, long, default_value = "10")]
        end: usize,
    },
}
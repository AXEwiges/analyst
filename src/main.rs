use analyst::cli::{Cli, Command};
use analyst::run;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::MissingValues { file_path, column } => {
            run::missing_values(&file_path, column.as_deref())?;
        }
        Command::FrequentPatterns { file_path, min_support } => {
            run::frequent_patterns(&file_path, min_support)?;
        }
        Command::ColumnStats { file_path, column } => {
            run::column_stats(&file_path, column.as_deref())?;
        }
        Command::Extrema { file_path, column } => {
            run::extrema(&file_path, &column)?;
        }
        Command::Show { file_path, start, end } => {
            run::show(&file_path, start, end)?;
        }
    }

    Ok(())
}
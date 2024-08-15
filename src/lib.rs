pub mod cli;
pub mod reader;
pub mod analyzer;
pub mod utils;

pub mod run {
    use anyhow::Result;
    use crate::reader::StreamingCsvReader;
    use crate::analyzer::{missing_values, frequent_patterns, column_statistics, extrema, show};

    pub fn missing_values(file_path: &str, column: Option<&str>) -> Result<()> {
        let reader = StreamingCsvReader::new(file_path)?;
        missing_values::analyze(reader, column)
    }

    pub fn frequent_patterns(file_path: &str, min_support: f64) -> Result<()> {
        let mut reader = StreamingCsvReader::new(file_path)?;
        frequent_patterns::analyze(&mut reader, min_support)
    }

    pub fn column_stats(file_path: &str, column: Option<&str>) -> Result<()> {
        let mut reader = StreamingCsvReader::new(file_path)?;
        column_statistics::analyze(&mut reader, column)
    }

    pub fn extrema(file_path: &str, column: &str) -> Result<()> {
        let reader = StreamingCsvReader::new(file_path)?;
        extrema::analyze(reader, column)
    }

    pub fn show(file_path: &str, start: usize, end: usize) -> Result<()> {
        let reader = StreamingCsvReader::new(file_path)?;
        show::analyze(reader, start, end)
    }
}
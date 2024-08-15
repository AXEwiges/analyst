use crate::reader::StreamingCsvReader;
use anyhow::{Result, anyhow};
use std::cmp::Ordering;

pub fn analyze(reader: StreamingCsvReader, target_column: &str) -> Result<()> {
    let headers = reader.headers();
    let column_index = headers
        .iter()
        .position(|h| h == target_column)
        .ok_or_else(|| anyhow!("Column '{}' not found", target_column))?;

    let mut min_value: Option<String> = None;
    let mut max_value: Option<String> = None;

    for result in reader {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            if !value.is_empty() {
                update_extrema(value, &mut min_value, &mut max_value);
            }
        }
    }

    match (min_value, max_value) {
        (Some(min), Some(max)) => {
            println!("Extrema for column '{}':", target_column);
            println!("  Minimum value: {}", min);
            println!("  Maximum value: {}", max);
        }
        _ => println!("No valid values found in column '{}'", target_column),
    }

    Ok(())
}

fn update_extrema(value: &str, min_value: &mut Option<String>, max_value: &mut Option<String>) {
    let update = |current: &mut Option<String>, cmp: Ordering| {
        if let Some(current_value) = current {
            if value.cmp(current_value) == cmp {
                *current = Some(value.to_string());
            }
        } else {
            *current = Some(value.to_string());
        }
    };

    update(min_value, Ordering::Less);
    update(max_value, Ordering::Greater);
}
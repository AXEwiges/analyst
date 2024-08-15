use crate::reader::StreamingCsvReader;
use anyhow::Result;
use std::collections::HashMap;

pub fn analyze(mut reader: StreamingCsvReader, target_column: Option<&str>) -> Result<()> {
    let headers = reader.headers().clone();
    let mut missing_counts: HashMap<String, usize> = headers
        .iter()
        .map(|header| (header.to_string(), 0))
        .collect();

    let mut total_rows = 0;

    while let Some(result) = reader.next() {
        let record = result?;
        total_rows += 1;
        for (i, field) in record.iter().enumerate() {
            if field.is_empty() {
                *missing_counts.get_mut(&headers[i].to_string()).unwrap() += 1;
            }
        }
    }

    println!("Total rows analyzed: {}", total_rows);
    println!("Missing value analysis:");

    for (header, count) in missing_counts.iter() {
        if target_column.is_none() || target_column.unwrap() == header {
            let percentage = (*count as f64 / total_rows as f64) * 100.0;
            println!("{}: {} missing values ({:.2}%)", header, count, percentage);
        }
    }

    Ok(())
}
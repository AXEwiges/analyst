use crate::reader::StreamingCsvReader;
use anyhow::Result;
use std::collections::HashMap;

pub fn analyze(reader: &mut StreamingCsvReader<'_>, target_column: Option<&str>) -> Result<()> {
    let headers = reader.headers().clone();
    let mut value_counts: Vec<HashMap<String, usize>> = vec![HashMap::new(); headers.len()];
    let mut total_rows = 0;

    while let Some(result) = reader.next() {
        let record = result?;
        total_rows += 1;
        for (i, field) in record.iter().enumerate() {
            *value_counts[i].entry(field.to_string()).or_insert(0) += 1;
        }
    }

    println!("Total rows analyzed: {}", total_rows);
    println!("Column statistics:");

    for (i, header) in headers.iter().enumerate() {
        if target_column.is_none() || target_column.unwrap() == header {
            println!("\nColumn: {}", header);
            let mut sorted_counts: Vec<_> = value_counts[i].iter().collect();
            sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

            for (value, count) in sorted_counts.iter().take(10) {
                let percentage = (*(*count) as f64 / total_rows as f64) * 100.0;
                println!("  {}: {} occurrences ({:.2}%)", value, count, percentage);
            }

            if value_counts[i].len() > 10 {
                println!("  ... and {} more unique values", value_counts[i].len() - 10);
            }
        }
    }

    Ok(())
}
use crate::reader::StreamingCsvReader;
use anyhow::{Result, anyhow};
use prettytable::{Table, Row, Cell};

pub fn analyze(reader: StreamingCsvReader, start: usize, end: usize) -> Result<()> {
    if start < 1 || end < start || end - start + 1 > 100 {
        return Err(anyhow!("Invalid range. Start should be >= 1, end should be >= start, and range should not exceed 100 rows."));
    }

    let headers = reader.headers();
    let mut table = Table::new();
    table.add_row(Row::from_iter(headers.iter().map(|h| Cell::new(h))));

    for (i, result) in reader.enumerate() {
        if i + 1 > end {
            break;
        }
        if i + 1 >= start {
            let record = result?;
            table.add_row(Row::from_iter(record.iter().map(|field| Cell::new(field))));
        }
    }

    if table.len() == 1 {
        println!("No data found in the specified range.");
    } else {
        table.printstd();
    }

    Ok(())
}
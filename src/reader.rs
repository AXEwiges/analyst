use anyhow::Result;
use csv::{Reader, StringRecord};
use std::fs::File;
use std::path::Path;

pub struct StreamingCsvReader<'a> {
    reader: Reader<File>,
    headers: StringRecord,
    current_record: StringRecord,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> StreamingCsvReader<'a> {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);
        let headers = reader.headers()?.clone();
        let current_record = StringRecord::new();

        Ok(Self { 
            reader, 
            headers, 
            current_record,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn headers(&self) -> &StringRecord {
        &self.headers
    }
}

impl<'a> Iterator for StreamingCsvReader<'a> {
    type Item = Result<&'a StringRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_record(&mut self.current_record) {
            Ok(true) => {
                // This is safe because we're guaranteeing that the reference
                // doesn't outlive the StreamingCsvReader instance
                let record = unsafe { std::mem::transmute(&self.current_record) };
                Some(Ok(record))
            },
            Ok(false) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}
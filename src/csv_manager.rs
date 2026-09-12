use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool)
}
pub enum Column {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Text(Vec<String>),
    Bool(Vec<bool>),
}
pub struct CsvFrame { columns: HashMap<String, Column>, }
impl CsvFrame {
    pub fn new() -> Self { Self {columns: HashMap::new() } }
    fn get_column(&self, key: &str) -> Result<&Column, String> {
        return self.columns.get(key).ok_or(format!("Column '{}' not found.", key))
    }
    pub fn column_i64(&self, key: &str) {}
    pub fn column_f64(&self, key: &str) {}
    pub fn column_string(&self, key: &str) {}
    pub fn column_bool(&self, key: &str) {}
    pub fn row(&self, index: i32) -> Result<HashMap<String, Value>, String> {}
}

fn fetch_keys(reader: &BufReader) -> Result<Vec<str>, Error> {
    return reader.lines();
}

pub fn read_csv(filename: &str) -> Result<CsvFrame, Error> {
    let file = File::open(filename)?;    
    let reader = BufReader::new(file);
    let csv_frame: CsvFrame = CsvFrame::new();

    let keys: Vec<str> = fetch_keys(reader)?;
    for line in reader.lines() {
        let values: Vec<&str> = line?.split(',').collect();
        for num in 0..keys.len() {
            csv_frame.columns.insert(keys[num], values[num]);
        }
    }

    return Ok(csv_frame);
}



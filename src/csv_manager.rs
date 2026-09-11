use std::collections::HashMap;

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
    fn get_column(&self, key: &str) -> Result<&Column, String> {
        return self.columns.get(key).ok_or(format!("Column '{}' not found.", key))
    }
    pub fn column_i64(&self, key: &str) {}
    pub fn column_f64(&self, key: &str) {}
    pub fn column_string(&self, key: &str) {}
    pub fn column_bool(&self, key: &str) {}
    pub fn row(&self, index: i32) -> Result<HashMap<String, Value>, String> {}
}

pub fn read(filename: &str) -> CsvFrame {}



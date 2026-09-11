use std::collections::HashMap;

pub enum Column {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Text(Vec<String>),
    Bool(Vec<bool>),
}
pub struct CsvData { columns: HashMap<String, Column>, }
impl CsvData {
    fn get_column(&self, key: &str) -> Result<&Column, String> {
        return self.columns.get(key).ok_or(format!("Column '{}' not found.", key))
    }
}



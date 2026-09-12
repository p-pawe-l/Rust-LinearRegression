use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum Column {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Text(Vec<String>),
    Bool(Vec<bool>),
}

impl Column {
    fn len(&self) -> usize {
        match self {
            Column::Int(v) => v.len(),
            Column::Float(v) => v.len(),
            Column::Text(v) => v.len(),
            Column::Bool(v) => v.len(),
        }
    }

    fn value_at(&self, index: usize) -> Value {
        match self {
            Column::Int(v) => Value::Int(v[index]),
            Column::Float(v) => Value::Float(v[index]),
            Column::Text(v) => Value::Text(v[index].clone()),
            Column::Bool(v) => Value::Bool(v[index]),
        }
    }
}

#[derive(Debug)]
pub enum CsvError {
    Io(io::Error),
    Empty(String),
    ColumnNotFound(String),
    IndexOutOfBounds(i32),
    RowWidthMismatch { row: usize, expected: usize, found: usize },
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::Io(e) => write!(f, "I/O error: {}", e),
            CsvError::Empty(msg) => write!(f, "{}", msg),
            CsvError::ColumnNotFound(key) => write!(f, "Column '{}' not found.", key),
            CsvError::IndexOutOfBounds(idx) => write!(f, "Row index {} out of bounds.", idx),
            CsvError::RowWidthMismatch { row, expected, found } => write!(
                f,
                "Row {} has {} values, expected {}.",
                row, found, expected
            ),
        }
    }
}

impl std::error::Error for CsvError {}

impl From<io::Error> for CsvError {
    fn from(e: io::Error) -> Self {
        CsvError::Io(e)
    }
}

pub struct CsvFrame {
    columns: HashMap<String, Column>,
    // Keeps the original header order, since HashMap iteration order is arbitrary.
    column_order: Vec<String>,
}

#[allow(dead_code)]
impl CsvFrame {
    pub fn new() -> Self {
        Self { columns: HashMap::new(), column_order: Vec::new() }
    }

    pub fn column_names(&self) -> &[String] {
        &self.column_order
    }

    pub fn num_rows(&self) -> usize {
        self.column_order
            .first()
            .and_then(|name| self.columns.get(name))
            .map(|c| c.len())
            .unwrap_or(0)
    }

    fn get_column(&self, key: &str) -> Result<&Column, CsvError> {
        self.columns
            .get(key)
            .ok_or_else(|| CsvError::ColumnNotFound(key.to_string()))
    }

    pub fn column_i64(&self, key: &str) -> Result<&Vec<i64>, CsvError> {
        match self.get_column(key)? {
            Column::Int(v) => Ok(v),
            _ => Err(CsvError::ColumnNotFound(format!("{} (not an i64 column)", key))),
        }
    }

    pub fn column_f64(&self, key: &str) -> Result<&Vec<f64>, CsvError> {
        match self.get_column(key)? {
            Column::Float(v) => Ok(v),
            _ => Err(CsvError::ColumnNotFound(format!("{} (not an f64 column)", key))),
        }
    }

    pub fn column_string(&self, key: &str) -> Result<&Vec<String>, CsvError> {
        match self.get_column(key)? {
            Column::Text(v) => Ok(v),
            _ => Err(CsvError::ColumnNotFound(format!("{} (not a text column)", key))),
        }
    }

    pub fn column_bool(&self, key: &str) -> Result<&Vec<bool>, CsvError> {
        match self.get_column(key)? {
            Column::Bool(v) => Ok(v),
            _ => Err(CsvError::ColumnNotFound(format!("{} (not a bool column)", key))),
        }
    }

    pub fn row(&self, index: i32) -> Result<HashMap<String, Value>, CsvError> {
        if index < 0 || index as usize >= self.num_rows() {
            return Err(CsvError::IndexOutOfBounds(index));
        }
        let idx = index as usize;
        let mut row = HashMap::new();
        for name in &self.column_order {
            let column = self.columns.get(name).expect("column_order is out of sync with columns");
            row.insert(name.clone(), column.value_at(idx));
        }
        Ok(row)
    }
}

/// Tries to parse every value in a column as increasingly permissive types,
/// falling back to plain text if none of them fit every row.
fn infer_column(values: Vec<String>) -> Column {
    if values.iter().all(|v| v.parse::<i64>().is_ok()) {
        return Column::Int(values.iter().map(|v| v.parse::<i64>().unwrap()).collect());
    }
    if values.iter().all(|v| v.parse::<f64>().is_ok()) {
        return Column::Float(values.iter().map(|v| v.parse::<f64>().unwrap()).collect());
    }
    if values.iter().all(|v| v.parse::<bool>().is_ok()) {
        return Column::Bool(values.iter().map(|v| v.parse::<bool>().unwrap()).collect());
    }
    Column::Text(values)
}

#[allow(dead_code)]
pub fn read_csv(filename: &str) -> Result<CsvFrame, CsvError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut header_line = String::new();
    let header_len = reader.read_line(&mut header_line)?;
    if header_len == 0 {
        return Err(CsvError::Empty(format!("'{}' has no header row.", filename)));
    }
    let keys: Vec<String> = header_line.trim().split(',').map(|s| s.to_string()).collect();

    // Collect raw string values per column first, then infer each column's
    // type once all rows are known.
    let mut raw_columns: Vec<Vec<String>> = vec![Vec::new(); keys.len()];

    let mut buf = String::new();
    let mut row_num = 0usize;
    loop {
        buf.clear();
        let len = reader.read_line(&mut buf)?;
        if len == 0 {
            break;
        }
        let line = buf.trim();
        if line.is_empty() {
            continue;
        }
        let values: Vec<&str> = line.split(',').collect();
        if values.len() != keys.len() {
            return Err(CsvError::RowWidthMismatch {
                row: row_num,
                expected: keys.len(),
                found: values.len(),
            });
        }
        for (col, value) in values.iter().enumerate() {
            raw_columns[col].push(value.to_string());
        }
        row_num += 1;
    }

    let mut csv_frame = CsvFrame::new();
    for (key, values) in keys.into_iter().zip(raw_columns.into_iter()) {
        csv_frame.columns.insert(key.clone(), infer_column(values));
        csv_frame.column_order.push(key);
    }

    Ok(csv_frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_csv(name: &str, contents: &str) -> String {
        let path = std::env::temp_dir().join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn infers_int_float_text_bool_columns() {
        let path = write_temp_csv(
            "csv_manager_test_basic.csv",
            "id,price,name,active\n1,9.99,apple,true\n2,4.5,pear,false\n",
        );
        let frame = read_csv(&path).unwrap();

        assert_eq!(frame.column_i64("id").unwrap(), &vec![1, 2]);
        assert_eq!(frame.column_f64("price").unwrap(), &vec![9.99, 4.5]);
        assert_eq!(
            frame.column_string("name").unwrap(),
            &vec!["apple".to_string(), "pear".to_string()]
        );
        assert_eq!(frame.column_bool("active").unwrap(), &vec![true, false]);
    }

    #[test]
    fn mixed_int_and_float_promotes_to_float() {
        let path = write_temp_csv("csv_manager_test_promote.csv", "n\n1\n2\n3.5\n");
        let frame = read_csv(&path).unwrap();
        assert_eq!(frame.column_f64("n").unwrap(), &vec![1.0, 2.0, 3.5]);
    }

    #[test]
    fn row_returns_all_columns_by_name() {
        let path = write_temp_csv("csv_manager_test_row.csv", "id,name\n1,apple\n2,pear\n");
        let frame = read_csv(&path).unwrap();
        let row0 = frame.row(0).unwrap();
        assert_eq!(row0.get("id"), Some(&Value::Int(1)));
        assert_eq!(row0.get("name"), Some(&Value::Text("apple".to_string())));
        assert!(frame.row(2).is_err());
    }

    #[test]
    fn missing_column_is_an_error() {
        let path = write_temp_csv("csv_manager_test_missing.csv", "id\n1\n");
        let frame = read_csv(&path).unwrap();
        assert!(frame.column_string("nope").is_err());
    }
}

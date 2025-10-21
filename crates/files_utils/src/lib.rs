/// files_utils: helpers for JSON, YAML, and CSV
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io;

/// Read JSON from a file and deserialize to the given type
pub fn read_json_file<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let s = fs::read_to_string(path)?;
    let v = serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(v)
}

/// Read YAML from a file and deserialize to the given type
pub fn read_yaml_file<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let s = fs::read_to_string(path)?;
    let v = serde_yaml::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(v)
}

/// Simple CSV reader that returns vector of records (as Vec<String>)
pub fn read_csv<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<String>>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut out = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        out.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;

    #[derive(Debug, Deserialize)]
    struct Person { name: String, age: u8 }

    #[test]
    fn test_json_roundtrip() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();
        fs::write(path, r#"{ "name": "Alice", "age": 30 }"#).unwrap();
        let p: Person = read_json_file(path).unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_csv_read() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path();
        fs::write(path, "one,two,three\n1,2,3\n").unwrap();
        let rows = read_csv(path).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1][2], "3");
    }
}

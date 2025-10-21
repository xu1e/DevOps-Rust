/// files_utils: helpers for JSON, YAML, and CSV
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

/// Read JSON from a file and deserialize to the given type
pub fn read_json_file<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let s = fs::read_to_string(path)?;
    let v = serde_json::from_str(&s)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(v)
}

/// Read YAML from a file and deserialize to the given type
pub fn read_yaml_file<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let s = fs::read_to_string(path)?;
    let v = serde_yaml::from_str(&s)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(v)
}

/// Simple CSV reader that returns vector of records (Vec<String>)
/// If `has_headers` is true, the header row will be skipped.
pub fn read_csv<P: AsRef<Path>>(path: P, has_headers: bool) -> io::Result<Vec<Vec<String>>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(path)?;
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
    use tempfile::NamedTempFile;

    #[derive(Debug, Deserialize, Serialize)]
    struct Person {
        name: String,
        age: u8,
    }

    #[test]
    fn test_json_roundtrip() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "{{ \"name\": \"Alice\", \"age\": 30 }}").unwrap();
        tmp.flush().unwrap();

        let p: Person = read_json_file(tmp.path()).unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_csv_read_with_headers() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "one,two,three").unwrap();
        writeln!(tmp, "1,2,3").unwrap();
        tmp.flush().unwrap();

        let rows = read_csv(tmp.path(), true).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][2], "3");
    }

    #[test]
    fn test_csv_read_without_headers() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "one,two,three").unwrap();
        writeln!(tmp, "1,2,3").unwrap();
        tmp.flush().unwrap();

        let rows = read_csv(tmp.path(), false).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1][2], "3");
    }
}

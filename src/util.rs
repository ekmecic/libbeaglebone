//! general utils
use std::fs::File;
use std::io::Write;

/// Write some data to a file
pub fn write_to_file(data: &str, path: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

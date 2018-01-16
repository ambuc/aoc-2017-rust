use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get(path_str: &'static str) -> String {
    let path = Path::new(&path_str);
    let mut file_str: String = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut file_str)
        .unwrap();
    return file_str;
}

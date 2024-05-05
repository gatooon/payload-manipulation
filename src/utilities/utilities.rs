use std::fs::File;
use std::io::Read;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Vec<u8>{
    let mut file = File::open(path).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    return buf;
}

pub fn write_file(payload: Vec<u8>, path: &str) {
    let mut buffer = File::create(path).unwrap();
    buffer.write_all(&payload).unwrap();
}
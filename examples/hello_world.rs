// use std::io::prelude::*;
// use std::fs::File;

// extern crate clay;

// use clay::primitive_readers::{ DataOps };
// use clay::ShapeReaderFactory;

use basixT4::decode::Base64Decoder;

fn main() {
    let hello_world = 
        Base64Decoder::new()
            .decode("aGVsbG8gd29ybGQhISE=")
            .iter()
            .map(|it| char::from(*it))
            .collect::<String>();

    println!("{}", hello_world);
}
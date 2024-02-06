extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
//use std::process::Output;
use std::time::Instant; //time to get compressed

fn main() {
    if args().len() !=3 { // it expects 3 arguments
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input= BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default()); // here's where the magic starts
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Ellasped time: {:?}", start.elapsed());
}

// cd to the project, cargo init, then cargo run (ctrl + l to clear the screen) 
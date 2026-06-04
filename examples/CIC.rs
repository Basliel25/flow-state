///! A complete smoke test for a real conn.log data
///! Extracted with zeek from the CIC2017 dataset

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use flow_state::{FlowFeatures, FlowStateBuilder};

const k: usize = 20;

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("File not found");
            process::exit(2);
        }
    };

    let file = File::open(&path).expect("open conn.log");
    let batch: Vec<FlowFeatures> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| FlowFeatures::from_tsv_row(&line))
        .collect();

    eprintln!("Parsed {} from {}", batch.len(), path);

    if batch.len() < k {
        eprintln!("Insufficent data");
        process::exit(2);
    }
}


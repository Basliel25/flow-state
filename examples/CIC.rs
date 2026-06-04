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

    // Fitting
    let state = FlowStateBuilder::new(k).fit(&batch);
    eprintln!("Fitted {} clusters", state.k());

    // Predicting
    // Construct a histogram with clusters
    let mut hist: BTreeMap<u64, usize> = BTreeMap::new();

    for f in &batch {
        let id = state.predict(f);
        *hist.entry(id).or_insert(0) += 1;
    }

    // Print sorted with %
    let total = batch.len() as f64;
    let mut rows: Vec<(u64, usize)> = hist.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));

    eprintln!("\ncluster histogram (id | count | percentage):",);
    for (id, count) in rows {
        eprintln!(" {:3} | {:6} | {:5.1}%", id, count, 100.0 * count as f64 / total);
    }
}


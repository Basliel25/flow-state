# Flow state 
# flow-state

[![Crates.io](https://img.shields.io/crates/v/flow-state.svg)](https://crates.io/crates/flow-state)
[![License](https://img.shields.io/crates/l/flow-state.svg)](https://github.com/Basliel25/flow-state#license)

Clustering-based state abstraction for Zeek `conn.log` flows.

- Parse Zeek TSV rows into 42-dim feature vectors (categorical one-hots + continuous fields).
- Fit a K-means model over scaled flows; assign each flow to a discrete state ID.
- Pair with [`markov-rs`](https://crates.io/crates/markov-rs) for behavioral anomaly detection over flow sequences.

## Usage

```rust
use flow_state::{FlowFeatures, FlowStateBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Parse a Zeek conn.log (TSV). Header lines and malformed rows are skipped.
let file = File::open("conn.log")?;
let batch: Vec<FlowFeatures> = BufReader::new(file)
    .lines()
    .filter_map(Result::ok)
    .filter_map(|line| FlowFeatures::from_tsv_row(&line))
    .collect();

// Fit K=20 clusters. Builder consumes itself and returns a fitted FlowState;
// calling predict before fit is a type error.
let state = FlowStateBuilder::new(20).fit(&batch);

// Assign each flow to a state ID in [0, k).
for flow in &batch {
    let id = state.predict(flow);
    // feed `id` into markov-rs, log it, whatever.
}

// Inspect a cluster centroid (in scaled feature space).
let centroid = state.resolve(0);
```

## Pipeline

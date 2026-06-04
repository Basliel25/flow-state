///! Integration test for full fit/predict contract

use flow_state::{FlowFeatures, FlowStateBuilder, FEATURE_DIM};

/// Build a synthetic conn.log TSV row 
///   0 ts | 1 uid | 2 orig_h | 3 orig_p | 4 resp_h | 5 resp_p
///   6 proto | 7 service | 8 duration | 9 orig_bytes | 10 resp_bytes | 11 conn_state
fn row(proto: &str, service: &str, dur: &str, ob: &str, rb: &str, cs: &str) -> String {
    format!("1.0\tCxxxx\t1.2.3.4\t1111\t5.6.7.8\t80\t{proto}\t{service}\t{dur}\t{ob}\t{rb}\t{cs}")
}

fn synthetic_batch() -> Vec<FlowFeatures> {
    let rows = [
        // short tcp and http combo
        ("tcp", "http", "0.1",  "100",   "200",   "SF"),
        ("tcp", "http", "0.15", "120",   "180",   "SF"),
        ("tcp", "http", "0.08", "90",    "210",   "SF"),
        // long tcp and ssl combo
        ("tcp", "ssl",  "30.0", "50000", "80000", "SF"),
        ("tcp", "ssl",  "45.0", "60000", "90000", "SF"),
        ("tcp", "ssl",  "28.0", "48000", "85000", "SF"),
        // udp and dns with intential missing entries
        ("udp", "dns",  "0.05", "60",    "120",   "SF"),
        ("udp", "dns",  "-",    "55",    "-",     "SF"),
        ("udp", "dns",  "0.03", "70",    "110",   "SF"),
    ];

    rows.iter()
        .map(|(p, s, d, ob, rb, cs)| {
            FlowFeatures::from_tsv_row(&row(p, s, d, ob, rb, cs))
                .expect("synthetic row should parse")
        }).collect()
}

#[test]
fn fit_then_predict_reutrns_id_in_range() {}

#[test]
fn reslove_returns_41_dim_centroid() {}

#[test]
fn same_flow_predicts_same_cluster() {}


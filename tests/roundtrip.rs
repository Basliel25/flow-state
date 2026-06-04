///! Integration test for full fit/predict contract

use flow_state::{FlowFeatures, FlowStateBuilder, FEATURE_DIM};

/// Build a synthetic conn.log TSV row 
///   0 ts | 1 uid | 2 orig_h | 3 orig_p | 4 resp_h | 5 resp_p
///   6 proto | 7 service | 8 duration | 9 orig_bytes | 10 resp_bytes | 11 conn_state
fn row(proto: &str, service: &str, dur: &str, ob: &str, rb: &str, cs: &str) -> String {
    format!("1.0\tCxxxx\t1.2.3.4\t1111\t5.6.7.8\t80\t{proto}\t{service}\t{dur}\t{ob}\t{rb}\t{cs}")
}

fn synthetic_batch() -> Vec<FlowFeatures> {}

#[test]
fn fit_then_predict_reutrns_id_in_range() {}

#[test]
fn reslove_returns_41_dim_centroid() {}

#[test]
fn same_flow_predicts_same_cluster() {}


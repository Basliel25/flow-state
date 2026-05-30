///! FlowKey: the catagorical projection of a zeek conn.log
///! file.
///
///! Packages (proto, sevice, conn_state) as a state

use std::collections::HashMap;

// Potential borrow check problem with String
// But the states should outlive the conn.log line
pub struct FlowKey {
    pub proto: String,
    pub service: String,
    pub conn_state: String,
}

impl FlowKey {
    pub fn from_tsv_row(row: &str) -> Option<Self> {
        if(row.starts_with("#")) {return None;}

        let fields: Vec<&str> = row.split("\t").collect();

        Some(FlowKey{
            proto: fields.get(7)?.to_string(),
            service: fields.get(8)?.to_string(),
            conn_state: fields.get(12)?.to_string(),
        })
    }
}




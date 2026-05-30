///! FlowKey: the catagorical projection of a zeek conn.log
///! file.
///
///! Packages (proto, sevice, conn_state) as a state

use std::collections::HashMap;
use std::fs;

// Potential borrow check problem with String
// But the states should outlive the conn.log line
pub struct FlowKey {
    pub proto: String,
    pub service: String,
    pub conn_state: String,
}

impl FlowKey {
    pub fn from_tsv_row(row: &str) -> Option<Self> {
        if row.starts_with("#") {return None;}

        let fields: Vec<&str> = row.split("\t").collect();

        Some(FlowKey{
            proto: fields.get(7)?.to_string(),
            service: fields.get(8)?.to_string(),
            conn_state: fields.get(12)?.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample_tsv_sanity() {
        // Parse valid rows
        // columns 1..=12, with
        // proto@7
        // service@8
        // conn_state@12
        let content = fs::read_to_string("tests/fixtures/sample_conn_log.tsv").expect("fixture missing");

        let keys: Vec<FlowKey> = content
            .lines()
            .filter_map(FlowKey::from_tsv_row)
            .collect();

        eprintln!("content length: {}", content.len());
        eprintln!("first 200 bytes: {:?}", &content[..content.len().min(200)]);
        assert!(keys.is_empty(), "no rows");

        // Simple sanity check to see if proto is parsed 
        // correctly
        for k in &keys {
            assert!(["tcp", "udp", "icmp"].contains(&k.proto.as_str()), 
            "unexpected protocol {}", k.proto
            );
        }
    }

}





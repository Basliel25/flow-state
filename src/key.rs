use std::collections::HashMap;
use std::fs;
///! FlowKey: the catagorical projection of a zeek conn.log
///! file.
///
///! Packages (proto, sevice, conn_state) as a state
pub struct FlowKey {
    // Potential borrow check problem with String
    // But the states should outlive the conn.log line
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
        let content = fs::read_to_string("tests/fixtures/sample_conn_log.tsv").expect("fixture missing");

        let keys: Vec<FlowKey> = content
            .lines()
            .filter_map(FlowKey::from_tsv_row)
            .collect();

        assert!(keys.is_empty(), "no rows");

        // Simple sanity check to see if proto is parsed 
        // correctly
        for k in &keys {
            assert!(["tcp", "udp", "icmp"].contains(&k.proto.as_str()), 
            "unexpected protocol {}", k.proto
            );
        }
    }

    #[test]
    fn parses_valid_row() {
        // Parse valid rows
        // columns 1..=12, with
        // proto@7
        // service@8
        // conn_state@12
        let row = "ts\tuid\torig_h\torig_p\tresp_h\tresp_p\tlocal\ttcp\thttp\t1.5\t100\t200\tSF";
        let key = FlowKey::from_tsv_row(row).unwrap();
        assert_eq!(key.proto, "tcp");
        assert_eq!(key.service, "http");
        assert_eq!(key.conn_state, "SF");
    }

    #[test]
    fn rejects_header_lines() {
        assert!(FlowKey::from_tsv_row("#separator \\x09").is_none());
        assert!(FlowKey::from_tsv_row("#fields\tts\tuid").is_none());
    }

    #[test]
    fn rejects_short_rows() {
        assert!(FlowKey::from_tsv_row("ts\tuid\tonly\tthree").is_none());
    }

    #[test]
    fn preserves_zeek_dash() {
        // Zeek writes '-' to mark empty placeholders
        let row = "ts\tuid\torig_h\torig_p\tresp_h\tresp_p\tlocal\ttcp\t-\t1.5\t100\t200\tS0";
        let key = FlowKey::from_tsv_row(row).unwrap();
        assert_eq!(key.service, "-");
        assert_eq!(key.conn_state, "S0");
    }
}





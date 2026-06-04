use std::num::FpCategory::Nan;

///! Extract features form zeek conn.log line
///! and returns a pure high-dimensonla feauture vector 
///! draft vector Dimensions:
///! [0] duration in seconds (NaN if empty)
///! [1] original bytes (NaN if empty)
///! [2] resp_bytes
///! [3..8] protocol one-hot (5 separate states): tcp, udp, icmp, unkown_transport and potentially other
///! [8..34] service one-hot(26 separate states): refer to features/service
///! [34..41] connection state one hot(7 separate states): SF, S0, S1, S2, S3, OTH, RSTO potentaily other 
///! Continous values newds to be log transformed, done by scaler?

use ndarray::Array1;

pub const FEATURE_DIM: usize = 41;

const PROTOS: &[&str] = &[
    "tcp", "udp", "icmp", "unknown_transport",
];

const SERVICES: &[&str] = &["-", "dce_rpc", "dhcp", "dns", "enum", "ftp", "ftp-data",
"geneve", "gssapi", "gssapi,ntlm,smb", "gssapi,ntlm,smb,dce_rpc",
"gssapi,smb", "http", "imap", "irc", "ldap_tcp", "modbus",
"ntlm", "ntlm,smb,dce_rpc", "pop3", "radius", "smb",
"smtp", "ssh", "ssl", "syslog",];

const CONN_STATE: &[&str] = &["OTH", "RSTO", "S0", "S1", "S3", "SF",];

struct ParsedRow {
    proto_idx: usize,       // 0..=4 
    service_idx: usize,     // 0..=26 
    conn_state_idx: usize,  // 0..=6 
    duration: f64,  // NaN if missing
    orig_bytes: f64, // NaN if missing
    resp_bytes: f64, // NaN if missing
}
#[derive(Clone, Debug)]
pub struct FlowFeatures {
    pub vector: Array1<f64>,
}

impl FlowFeatures {
    pub fn from_tsv_row(row: &str)->Option<Self> {
        let parsed = parse_row(row)?;
        Some(FlowFeatures {vector: to_vector(&parsed)})
    }
}

fn parse_row(row: &str) -> Option<ParsedRow> {
    if row.starts_with('#') {return None}

    let fields: Vec<&str> = row.split('\t').collect();

    Some(ParsedRow{
        proto_idx: lookup(&PROTOS, fields.get(6)?),
        service_idx: lookup(&SERVICES, fields.get(7)?),
        conn_state_idx: lookup(&CONN_STATE, fields.get(11)?),
        duration: parse_continous(fields.get(8)?),
        orig_bytes: parse_continous(fields.get(9)?),
        resp_bytes: parse_continous(fields.get(10)?),
    })
}

fn lookup(vocab: &[&str], value: &str) -> usize {
    vocab.iter().position(|&v| v == value).unwrap_or(vocab.len())
}

/// Parse contionus data sets as array is Array1<f64>
fn parse_continous(s: &str) -> f64 {
    if s == "-" {f64::NAN} 
    else {s.parse().unwrap_or(f64::NAN)}
}

fn to_vector(parsed: &ParsedRow) -> Array1<f64> {
    let mut feature_vector = Array1::zeros(FEATURE_DIM);

    feature_vector[0] = parsed.duration;
    feature_vector[1] = parsed.orig_bytes;
    feature_vector[2] = parsed.resp_bytes;

    feature_vector[3 + parsed.proto_idx] = 1.0;          
    feature_vector[8 + parsed.service_idx] = 1.0;        
    feature_vector[34 + parsed.conn_state_idx] = 1.0;    

    feature_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONT: std::ops::Range<usize> = 0..3;
    const PROTO: std::ops::Range<usize> = 3..8;
    const SERVICE: std::ops::Range<usize> = 8..34;
    const CONN_STATE: std::ops::Range<usize> = 34..41;

    fn row(proto: &str
        , service: &str
        , dur: &str
        , ob: &str
        , rb: &str
        , cs: &str) -> String {
        format!("1.0\tCxxxx\t1.2.3.4\t1111\t5.6.7.8\t80\t{proto}\t{service}\t{dur}\t{ob}\t{rb}\t{cs}")
    }
}

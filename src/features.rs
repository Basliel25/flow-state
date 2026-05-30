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
    pub fn from_tsv_row(row: &str)->Option<Self> {todo!()}
}

fn parse_row(row: &str) -> Option<ParsedRow> {
    if row.starts_with('#') {return None}

    let fields: Vec<&str> = row.split('\t').collect();

    Some(ParsedRow{
        proto_idx: lookup(&PROTOS, fields.get(7)?),
        service_idx: lookup(&SERVICES, fields.get(8)?),
        conn_state_idx: lookup(&CONN_STATE, fields.get(12)?),
        duration: parse_continous(fields.get(9)?),
        orig_bytes: parse_continous(fields.get(10)?),
        resp_bytes: parse_continous(fields.get(11)?),
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

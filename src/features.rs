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

#[derive(Clone, Debug)]
pub struct FlowFeatures {
    pub vector: Array1<f64>,
}

impl FlowFeatures {
    pub fn from_tsv_row(row: &str)->Option<self> {todo!()}
}

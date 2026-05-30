use ndarray::Array1;

///! Extract features form zeek conn.log line
///! and returns a pure high-dimensonla feauture vector 
///! draft vector Dimensions:
///! [0] duration in seconds (NaN if empty)
///! [1] original bytes (NaN if empty)
///! [2] resp_bytes
///! [3..7] protocol one-hot: tcp, udp and potentially other
///! [7..16] service one-hot: http, ssnl, dns, ftp, ssh, smtcp, irc, dhcp or potentialy others
///! [16..25] connection state one hot: SF, S0, S1, REJ, RSTO, RSTR, SH, OTH potentaily other 
///! Continous values newds to be log transformed, done by scaler?
#[derive(Clone, Debug)]
pub struct FlowFeatures {
    pub vector: Array1<f64>,
}

impl FlowFeatures {
    pub fn from_tsv_row(row: &str)->Option<self> {todo!()}
}

use ndarray::Array1;

///! Extract features form zeek conn.log line
///! and returns a pure high-dimensonla feauture vector 
pub struct FlowFeatures {
    pub vector: Array1<f64>,
}

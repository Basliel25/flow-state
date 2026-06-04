///! Scaler
///! log1p and z scoring for the continous dimensions, 
///! And pass through the one-hots
///! This module principally does two things:
///! Fitting: consumes a batch and freezes per columns params mean and std diviation
///! Transformation: Applies and scales the frozen params and scales

use ndarray::Array1;
use crate::features::{FlowFeatures, FEATURE_DIM};


/// Index of the contious dimensions
const CONT_DIMS: [usize;3] = [0, 1, 2];

pub struct scaler {
    /// Mean of log1p(x) for continous, NaN ignored
    means: [f64;3],
    /// Std of log1p(x) for continous, NaN ignored
    std: [f64;3],
}

impl scaler {
    pub fn fit(batch: &[FlowFeatures]) -> Self {todo!()}
    pub fn transform(&self, f: &FlowFeatures) -> Array1<f64> {todo!()}
}

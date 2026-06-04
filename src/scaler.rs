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

#[derive(Clone, Debug)]
pub struct Scaler {
    /// Mean of log1p(x) for continous, NaN ignored
    means: [f64;3],
    /// Std of log1p(x) for continous, NaN ignored
    std: [f64;3],
}

impl Scaler {
    /// Fit per-column mean/std on log1p'd continuous columns, ignoring NaNs.
    pub fn fit(batch: &[FlowFeatures]) -> Self {
        // Implement Welfords algorithm to calculate mean

        if batch.is_empty() {panic!("Empty batch");}

        let mut means = [0.0f64;3];
        let mut stds = [0.0f64;3];

        for (i, &cols) in CONT_DIMS.iter().enumerate() {

            let mut count: u64 = 0;
            let mut mean: f64 = 0.0;
            let mut mean_2: f64 = 0.0;

            for b in batch {
                let x = b.vector[cols].ln_1p();
                if x.is_nan() {continue;}

                count += 1;

                let delta = x - mean;
                mean += delta / count as f64;

                let delta_2 = x - mean;
                mean_2 += delta * delta_2;
            }

            if count == 0 {panic!("continous col: {} is entirley empty", cols)}

            // Calculate final mean
            let variance = mean_2 / count as f64;
            let std = variance.sqrt();
            let std = if std < 1e-12 { 1.0 } else { std };

            // Add to the scaler struct
            means[i] = mean;
            stds[i] = std;

    }

        Scaler {means, std: stds}
}


    pub fn transform(&self, f: &FlowFeatures) -> Array1<f64> {todo!()}
}



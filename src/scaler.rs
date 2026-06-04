///! Scaler
///! log1p and z scoring for the continous dimensions, 
///! And pass through the one-hots
///! This module principally does two things:
///! Fitting: consumes a batch and freezes per columns params mean and std diviation
///! Transformation: Applies and scales the frozen params and scales

use ndarray::{Array1, s};
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


    pub fn transform(&self, f: &FlowFeatures) -> Array1<f64> {
        let mut scaled: Array1<f64> = Array1::zeros(FEATURE_DIM);

        for (i, &col) in CONT_DIMS.iter().enumerate() {
            let x = f.vector[col];
            if x.is_nan() {continue;}

            scaled[col] = (x.ln_1p() - self.means[i]) / self.std[i];
        }

        scaled.slice_mut(s![3..])
            .assign(&f.vector.slice(s![3..]));
        scaled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;
    
    /// Build a flow features vector from given parameters
    fn ff(dur: f64, ob: f64, rb: f64) -> FlowFeatures {
        let mut v = Array1::zeros(FEATURE_DIM);
        v[0] = dur;
        v[1] = ob;
        v[2] = rb;
        v[3]  = 1.0; // proto
        v[8]  = 1.0; // service
        v[34] = 1.0; // conn_state 
        FlowFeatures { vector: v }
    }

    #[test]
    fn fit_ignores_nan_crosscheck_handcomputed_log_mean() {
        // mean(log1p) = (0 + 1) / 2 = 0.5
        let batch = vec![
            ff(0.0,0.0, 0.0),
            ff(std::f64::consts::E - 1.0, 0.0, 0.0),
            ff(f64::NAN,0.0, 0.0),
        ];

        let s = Scaler::fit(&batch);

        assert!((s.means[0] - 0.5).abs() < 1e-10,
                "expected mean(log1p) = 0.5, got {}", s.means[0]);
        assert!((s.std[0] - 0.5).abs() < 1e-10,
                "expected std = 0.5, got {}", s.std[0]);
    }

    #[test]
    fn transform_z_scores_continous_and_zero_for_nan() {
        let batch = vec![
            ff(0.0,0.0, 0.0),
            ff(std::f64::consts::E - 1.0, 0.0, 0.0),
            ff(f64::NAN,0.0, 0.0),
        ];

        let s = Scaler::fit(&batch);

        let out = s.transform(&ff(std::f64::consts::E - 1.0, 0.0, 0.0));
        assert!((out[0] - 1.0).abs() < 1e-10, "got {}", out[0]);

        // NaN input imputes to 0.0 
        let out_nan = s.transform(&ff(f64::NAN, 0.0, 0.0));
        assert_eq!(out_nan[0], 0.0);
    }




}



///! FlowState: orchestrates scaler and clusterModel as a single fit and predict model.
///
///! Two phases: new(k) constructs an unfit shell and 
///! fit(batch) freezes both scalar and cluster modle, predict if unfit.

use ndarray::Array1;

use crate::features::FlowFeatures;
use crate::scaler::Scaler;
use crate::cluster::ClusterModel;

pub struct FlowState {
    k: usize, 
    scaler: Option<Scaler>,
    cluster: Option<ClusterModel>,
}

impl FlowState {
    pub fn new(k: usize, cluster: ClusterModel) -> Self {
        Self {k, 
            scaler: None,
            cluster: None}
    }

    /// Fit scaler on raw batch, then fit cluster on scaled batch
    pub fn fit(&mut self, batch: &[FlowFeatures]) {todo!()}

    /// Map a flow to its cluster id
    pub fn predict(&self, f: &FlowFeatures) -> u64 {todo!()}

    ///Look up a centroid by cluster id
    /// Look up is in the scaled space
    pub fn resolve(&self, id: u64) -> Array1<f64> {
        todo!()
    }

    pub fn k(&self)->usize {self.k}

}



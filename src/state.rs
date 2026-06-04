///! FlowState: orchestrates scaler and clusterModel as a single fit and predict model.
///
///! Two phases: new(k) constructs an unfit shell and 
///! fit(batch) freezes both scalar and cluster modle, predict if unfit.

use ndarray::Array1;

use crate::features::FlowFeatures;
use crate::scaler::Scaler;
use crate::cluster::ClusterModel;

pub struct FlowStateBuilder {
    k: usize, 
}


impl FlowStateBuilder {
    pub fn new(k: usize) -> Self {
        Self { k }
    }

    /// Fit scaler on raw batch, then fit cluster on scaled batch
    pub fn fit(&mut self, batch: &[FlowFeatures]) -> FlowState {todo!()}
    pub fn k(&self)->usize {self.k}
}

/// Fitted modle
pub struct FlowState {
    k: usize,
    scaler: Scaler,
    cluster: ClusterModel,
}

impl FlowState {
    // Construction happens through FlowStateBuilder::fit
    
    /// Map a flow to its cluster id
    pub fn predict(&self, f: &FlowFeatures) -> u64 {todo!()}

    /// Look up a centroid by cluster id
    /// Look up is in the scaled space
    pub fn resolve(&self, id: u64) -> Array1<f64> {
        todo!()
    }

    pub fn k(&self)->usize {self.k}
}


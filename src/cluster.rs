///! K-means clustering over scaled feature vectors
///
///! Utilizes the linfa clustering's K-means. 
///
///! Outputs a u64 vecor with dim (0,K]

use ndarray::Array1;
use linfa::DatasetBase;
use linfa::traits::{Fit, Predict};
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;

#[derive(Debug, Clone)]
pub struct ClusterModel {
    k: usize, // Number of clusters
    /// Fitted linfa model
    inner: KMeans<f64, linfa_nn::distance::L2Dist>
}

impl ClusterModel {
    /// Fit k-means on a batch of scaled vectors
    pub fn fit(batch: &[Array1<f64>], k: usize) -> Self {
        todo!()
    }

    /// Assign a single scaled vector to its nearest centriod
    /// Reutrns the index of the cluster in (0,k]
    pub fn predict(&self, vector: &[Array1<f64>]) -> u64 {todo!()}

    /// The resolve module, fetches a cluster and returns a human
    /// readable state explanation.
    pub fn resolve(&self , id: u64) -> Array1<f64> {todo!()}

    pub fn k(&self) ->usize {self.k}
}

///! K-means clustering over scaled feature vectors
///
///! Utilizes the linfa clustering's K-means. 
///
///! Outputs a u64 vecor with dim (0,K]

use crate::features::FEATURE_DIM;
use ndarray::{Array1, Array2, Axis};
use linfa::{Dataset, DatasetBase};
use linfa::traits::{Fit, Predict};
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;

use rand_xoshiro::Xoshiro256Plus;
use rand_xoshiro::rand_core::SeedableRng;

#[derive(Debug, Clone)]
pub struct ClusterModel {
    k: usize, // Number of clusters
    /// Fitted linfa model
    inner: KMeans<f64, linfa_nn::distance::L2Dist>
}

impl ClusterModel {
    /// Fit k-means on a batch of scaled vectors
    pub fn fit(batch: &[Array1<f64>], k: usize) -> Self {
        // K clustering requires k samples to seat across centriods
        assert!(
            batch.len() >= k,
            "Batch has {} samples needs >= {}",
            batch.len(),
            k
        );

        // Linfa clustering expects a 2d array with shape (n_samples, n_features)
        let n = batch.len();
        let flat: Vec<f64> = batch
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect();

        let data = Array2::from_shape_vec((n, FEATURE_DIM), flat)
            .expect("Shape mismatch");

        // DatasetBase is used to wrap inputs for linfa
        let dataset = DatasetBase::from(data);

        // Seeded rng so the clustering is reproducible
        let rng = Xoshiro256Plus::seed_from_u64(42);
        
        // Construct fitted clusters
        let fitted: KMeans<f64, _> = KMeans::params_with_rng(k, rng)
            .fit(&dataset)
            .expect("K Means fit failed");

        ClusterModel {k, inner: fitted}
    }

    /// Assign a single scaled vector to its nearest centriod
    /// Reutrns the index of the cluster in (0,k]
    pub fn predict(&self, vector: &Array1<f64>) -> u64 {
        // Real chalenge here linfa wants 2darray
        let as_matrix = vector.view().insert_axis(Axis(0)).to_owned();

        let preds = self.inner.predict(&as_matrix);

        preds[0] as u64
    }

    /// The resolve module, fetches a cluster and returns a human
    /// readable state explanation.
    pub fn resolve(&self, id: u64) -> Array1<f64> {
        assert!((id as usize) < self.k, "cluster id {} out of range (k={})", id, self.k);
        self.inner.centroids().row(id as usize).to_owned()
    }

    pub fn k(&self) ->usize {self.k}
}



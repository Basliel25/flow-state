///! Flow key: The intern table - Maps flowkey state - u64
///! Data held in an IndexMap 

use indexMap::IndexMap;
use crate::key::FlowKey;

#[derive(Debug, Default)]
pub struct FlowState {
    table: IndexMap<FlowKey, ()>,
}

impl FlowState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern as a key, returns its stable id
    /// indexing to the same key returns the same id
    pub fn intern(&mut self, key: FlowKey) -> u64 {
        let (idx, _) = self.table.insert_full(key, ());
        idx as u64
    }

    /// Resolve function for fetching from table
    pub fn resolve(&self, id: u64) -> Option<FlowKey> {
        self.table.get_index(id as usize).map(|k, _| k.clone())
    }


}

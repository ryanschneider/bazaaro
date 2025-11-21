use super::provider::RngProvider;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

/// Client-side RNG that looks up pre-determined results from the server's log
pub struct ClientRng<K: Hash + Eq> {
    results: HashMap<K, u32>,
}

impl<K: Hash + Eq> ClientRng<K> {
    /// Create a ClientRng from the server's RNG log
    pub fn from_log(results: HashMap<K, u32>) -> Self {
        Self { results }
    }

    /// Get a reference to the results map
    pub fn results(&self) -> &HashMap<K, u32> {
        &self.results
    }
}

impl<K: Hash + Eq> RngProvider<K> for ClientRng<K> {
    fn result(&mut self, key: K) -> u32 {
        *self
            .results
            .get(&key)
            .expect("Missing RNG result from server - client/server desync!")
    }

    fn result_range(&mut self, key: K, _range: Range<u32>) -> u32 {
        // Range is ignored on client, just lookup the logged value
        self.result(key)
    }
}

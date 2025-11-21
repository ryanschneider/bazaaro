use super::provider::RngProvider;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

/// Test RNG that allows injecting predetermined outcomes for unit tests
pub struct TestRng<K: Hash + Eq> {
    values: HashMap<K, u32>,
}

impl<K: Hash + Eq> TestRng<K> {
    /// Create a new empty TestRng
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Add a predetermined value for a specific key (builder pattern)
    pub fn with_value(mut self, key: K, value: u32) -> Self {
        self.values.insert(key, value);
        self
    }

    /// Create a TestRng with a complete set of predetermined values
    pub fn with_values(values: HashMap<K, u32>) -> Self {
        Self { values }
    }

    /// Get a reference to the values map
    pub fn values(&self) -> &HashMap<K, u32> {
        &self.values
    }
}

impl<K: Hash + Eq> Default for TestRng<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Hash + Eq> RngProvider<K> for TestRng<K> {
    fn result(&mut self, key: K) -> u32 {
        *self.values.get(&key).unwrap_or(&0) // Default to 0 for unspecified keys
    }

    fn result_range(&mut self, key: K, _range: Range<u32>) -> u32 {
        self.result(key)
    }
}

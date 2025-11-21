use std::ops::Range;

/// Core trait for RNG providers that can be used interchangeably
/// for server (generates random values), client (looks up values), and tests (predetermined values).
pub trait RngProvider<K> {
    /// Get a random u32 for the given key
    fn result(&mut self, key: K) -> u32;

    /// Get a random value in the specified range
    fn result_range(&mut self, key: K, range: Range<u32>) -> u32;

    /// Get a random boolean with the given probability (0.0 to 1.0)
    fn result_bool(&mut self, key: K, probability: f32) -> bool {
        let value = self.result_range(key, 0..100);
        value < (probability * 100.0) as u32
    }
}

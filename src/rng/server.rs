use super::provider::RngProvider;
use bevy::prelude::Resource;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

/// Server-side RNG that generates actual random values and logs them for transmission to client
#[derive(Resource)]
pub struct ServerRng<K: Hash + Eq + Clone> {
    generator: StdRng,
    log: HashMap<K, u32>,
}

impl<K: Hash + Eq + Clone> ServerRng<K> {
    /// Create a new ServerRng with the given seed
    pub fn new(seed: u64) -> Self {
        Self {
            generator: StdRng::seed_from_u64(seed),
            log: HashMap::new(),
        }
    }

    /// Create a ServerRng with an existing RNG generator
    pub fn with_rng(generator: StdRng) -> Self {
        Self {
            generator,
            log: HashMap::new(),
        }
    }

    /// Get the logged results for transmission to client
    /// This takes ownership of the log, leaving an empty HashMap in its place
    pub fn take_log(&mut self) -> HashMap<K, u32> {
        std::mem::take(&mut self.log)
    }

    /// Get a reference to the current log without clearing it
    pub fn log(&self) -> &HashMap<K, u32> {
        &self.log
    }
}

impl<K: Hash + Eq + Clone> RngProvider<K> for ServerRng<K> {
    fn result(&mut self, key: K) -> u32 {
        let value = self.generator.random();
        self.log.insert(key, value);
        value
    }

    fn result_range(&mut self, key: K, range: Range<u32>) -> u32 {
        let value = self.generator.random_range(range);
        self.log.insert(key, value);
        value
    }
}

//! Deterministic Client/Server RNG System for Bevy
//!
//! This module provides a deterministic RNG system designed for client/server game architectures
//! where both sides run the same simulation code.
//!
//! ## Key Concepts
//!
//! - **System order independence**: Systems can run in any order as long as they use the correct key
//! - **Named RNG lookups**: Instead of consuming RNG values sequentially, each random decision
//!   is identified by a structured key
//! - **Server generates, client looks up**: The server generates random values and logs them.
//!   The client looks up results by the same key.
//!
//! ## Usage
//!
//! ### Server Setup
//! ```rust,ignore
//! use bazaaro::rng::prelude::*;
//!
//! app.insert_resource(ServerRng::<RngKey>::new(seed));
//! app.add_systems(Update, my_system::<ServerRng<RngKey>>);
//! ```
//!
//! ### Client Setup
//! ```rust,ignore
//! use bazaaro::rng::prelude::*;
//!
//! // After receiving RNG log from server
//! let client_rng = ClientRng::from_log(server_rng_log);
//! app.insert_resource(client_rng);
//! app.add_systems(Update, my_system::<ClientRng<RngKey>>);
//! ```
//!
//! ### Test Setup
//! ```rust,ignore
//! use bazaaro::rng::prelude::*;
//!
//! let test_rng = TestRng::new()
//!     .with_value(my_key, 42);
//! app.insert_resource(test_rng);
//! app.add_systems(Update, my_system::<TestRng<RngKey>>);
//! ```
//!
//! ### Writing Generic Systems
//! ```rust,ignore
//! fn my_system<R: RngProvider<RngKey>>(
//!     mut rng: ResMut<R>,
//!     // ... other parameters
//! ) {
//!     let key = RngKey {
//!         frame: current_frame,
//!         actor: ActorId::TeamSlot { team: Team::A, slot: 0 },
//!         action: RngAction::PickTarget,
//!     };
//!     let random_value = rng.result_range(key, 0..10);
//! }
//! ```

pub mod client;
pub mod key;
pub mod prelude;
pub mod provider;
pub mod server;
pub mod test;

// Re-export main types at module level
pub use client::ClientRng;
pub use key::{AbilityId, ActorId, RngAction, RngKey, Team};
pub use provider::RngProvider;
pub use server::ServerRng;
pub use test::TestRng;

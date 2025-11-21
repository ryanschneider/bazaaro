use serde::{Deserialize, Serialize};

/// Example RNG key structure that uniquely identifies each random decision.
/// Uses stable identifiers that are the same on both client and server.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RngKey {
    /// Game frame/tick when the decision is made
    pub frame: u64,

    /// Stable identifier for the entity (NOT Bevy's Entity ID)
    /// Could be team slot position, spawn order, etc.
    pub actor: ActorId,

    /// What kind of random decision is being made
    pub action: RngAction,
}

/// Stable identifier for game actors.
/// IMPORTANT: Do NOT use Bevy's Entity IDs directly - they may differ between
/// client and server due to spawn order variations.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActorId {
    /// Identifies an entity by its team and slot position
    TeamSlot { team: Team, slot: usize },

    /// Identifies an entity by a stable spawn ID assigned at creation
    SpawnId(u64),
}

/// Types of random actions that can occur in the game
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RngAction {
    /// Randomly picking a target for an action
    PickTarget,

    /// Checking if a critical hit occurs
    CheckCrit,

    /// Triggering a specific ability
    TriggerAbility { ability_id: AbilityId },

    // Add more action types as needed for your game
}

/// Team identifier
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Team {
    A,
    B,
}

/// Type alias for ability identifiers
/// Using String for serde compatibility, but could be an enum or other type
pub type AbilityId = String;

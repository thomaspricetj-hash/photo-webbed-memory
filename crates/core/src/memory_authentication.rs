// src/memory_authentication.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::node::NodeId;
use crate::engine::NodeState;

/// Authentication result for a memory state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MemoryAuthResult {
    Valid,
    Adjusted,
    Weak,
}

/// MAX‑tier configuration for the authentication loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAuthConfig {
    pub min_stability: f32,
    pub min_importance: f32,
    pub max_idle_ticks: u64,

    pub stability_reinforce_factor: f32,
    pub importance_reinforce_factor: f32,

    pub stability_decay_factor: f32,
    pub importance_decay_factor: f32,

    /// MAX‑tier: semantic drift correction multiplier
    pub drift_correction_factor: f32,

    /// MAX‑tier: long‑term heat reinforcement
    pub long_term_heat_factor: f32,
}

impl Default for MemoryAuthConfig {
    fn default() -> Self {
        Self {
            min_stability: 0.25,
            min_importance: 1.0,
            max_idle_ticks: 10_000,

            stability_reinforce_factor: 1.03,
            importance_reinforce_factor: 1.05,

            stability_decay_factor: 0.97,
            importance_decay_factor: 0.95,

            drift_correction_factor: 0.015,
            long_term_heat_factor: 0.02,
        }
    }
}

/// MAX‑tier memory authentication engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAuthEngine {
    pub config: MemoryAuthConfig,
}

impl MemoryAuthEngine {
    pub fn new(config: MemoryAuthConfig) -> Self {
        Self { config }
    }

    /// Authenticate a single memory node state.
    pub fn authenticate_state(
        &self,
        _id: NodeId,
        state: &mut NodeState,
        now: u64,
    ) -> MemoryAuthResult {
        let idle = now.saturating_sub(state.last_access);

        // ------------------------------------------------------------
        // 1. Idle penalty (MAX‑tier: soft decay)
        // ------------------------------------------------------------
        if idle > self.config.max_idle_ticks {
            state.stability *= self.config.stability_decay_factor;
            state.importance *= self.config.importance_decay_factor;
        }

        // ------------------------------------------------------------
        // 2. Stability / importance reinforcement or decay
        // ------------------------------------------------------------
        let mut reinforced = false;
        let mut weakened = false;

        if state.stability >= self.config.min_stability {
            state.stability *= self.config.stability_reinforce_factor;
            reinforced = true;
        } else {
            state.stability *= self.config.stability_decay_factor;
            weakened = true;
        }

        if state.importance >= self.config.min_importance {
            state.importance *= self.config.importance_reinforce_factor;
            reinforced = true;
        } else {
            state.importance *= self.config.importance_decay_factor;
            weakened = true;
        }

        // ------------------------------------------------------------
        // 3. MAX‑tier drift correction
        // ------------------------------------------------------------
        let drift_correction = self.config.drift_correction_factor
            * (1.0 - state.stability).max(0.0);

        state.stability = (state.stability + drift_correction).min(1.0);

        // ------------------------------------------------------------
        // 4. MAX‑tier long‑term heat reinforcement
        // ------------------------------------------------------------
        state.heat.long_term += self.config.long_term_heat_factor * state.stability;

        // ------------------------------------------------------------
        // 5. Clamp values
        // ------------------------------------------------------------
        state.stability = state.stability.clamp(0.0, 1.0);
        state.importance = state.importance.clamp(0.0, 10.0);

        // ------------------------------------------------------------
        // 6. Return authentication result
        // ------------------------------------------------------------
        match (reinforced, weakened) {
            (true, false) => MemoryAuthResult::Valid,
            (true, true) => MemoryAuthResult::Adjusted,
            (false, true) => MemoryAuthResult::Weak,
            (false, false) => MemoryAuthResult::Adjusted,
        }
    }

    /// Authenticate all memory states (pure additive, no removals).
    pub fn authenticate_all(
        &self,
        states: &mut HashMap<NodeId, NodeState>,
        now: u64,
    ) {
        for (id, state) in states.iter_mut() {
            let _ = self.authenticate_state(*id, state, now);
        }
    }
}


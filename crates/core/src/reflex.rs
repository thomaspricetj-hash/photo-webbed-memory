use crate::node::NodeId;
use crate::MemoryEngine;

use std::collections::HashMap;

/// Reflex entry: stimulus → target node
#[derive(Debug, Clone)]
pub struct ReflexEntry {
    pub stimulus: String,
    pub target: NodeId,

    /// Reflex intensity (0–1). Higher = stronger activation bias.
    pub strength: f32,

    /// Stability added to the target node when reflex fires.
    pub stability_gain: f32,

    /// Heat added to the target node when reflex fires.
    pub heat_gain: f32,

    /// Reflex usage counter (for learning/decay).
    pub usage_count: u64,

    /// Last time this reflex fired.
    pub last_fired: u64,
}

/// Reflex subsystem: fast stimulus → response memory
#[derive(Debug, Clone)]
pub struct ReflexSystem {
    /// stimulus label (lowercase) → reflex entry
    pub table: HashMap<String, ReflexEntry>,

    /// Global decay rate for reflex strength.
    pub decay_rate: f32,

    /// Global learning rate for reflex strengthening.
    pub learning_rate: f32,
}

impl ReflexSystem {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            decay_rate: 0.0005,
            learning_rate: 0.02,
        }
    }

    // ------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------

    pub fn add_reflex(
        &mut self,
        stimulus: &str,
        target: NodeId,
        strength: f32,
        stability_gain: f32,
        heat_gain: f32,
    ) {
        let entry = ReflexEntry {
            stimulus: stimulus.to_lowercase(),
            target,
            strength: strength.clamp(0.0, 1.0),
            stability_gain,
            heat_gain,
            usage_count: 0,
            last_fired: 0,
        };

        self.table.insert(stimulus.to_lowercase(), entry);
    }

    pub fn remove_reflex(&mut self, stimulus: &str) {
        self.table.remove(&stimulus.to_lowercase());
    }

    pub fn has_reflex(&self, stimulus: &str) -> bool {
        self.table.contains_key(&stimulus.to_lowercase())
    }

    pub fn get_reflex(&self, stimulus: &str) -> Option<&ReflexEntry> {
        self.table.get(&stimulus.to_lowercase())
    }

    // ------------------------------------------------------------
    // TRIGGERING + LEARNING
    // ------------------------------------------------------------

    /// Trigger reflex if stimulus matches, with learning + decay hooks.
    pub fn trigger(&mut self, engine: &mut MemoryEngine, stimulus: &str, now: u64) {
        let key = stimulus.to_lowercase();

        if let Some(entry) = self.table.get_mut(&key) {
            let target = entry.target;

            // Reflex activation (safe: engine guards reflex lane)
            engine.activate(target, now, "reflex");

            // Reinforce heat + stability scaled by strength
            if let Some(state) = engine.states.get_mut(&target) {
                state.heat.short_term += entry.heat_gain * entry.strength;
                state.stability = (state.stability + entry.stability_gain * entry.strength).min(1.0);
            }

            // Hebbian-style learning: repeated firing strengthens reflex
            entry.strength = (entry.strength + self.learning_rate).clamp(0.0, 1.0);

            // Usage tracking
            entry.usage_count += 1;
            entry.last_fired = now;

            // Hook: episodic linking (placeholder)
            self.link_to_episodic(engine, target, now);

            // Hook: photonic bias (placeholder)
            self.bias_photonic(engine, target);
        }
    }

    // ------------------------------------------------------------
    // DECAY / MAINTENANCE
    // ------------------------------------------------------------

    /// Time-based decay of reflex strength.
    /// Call this periodically from MemoryEngine (e.g., in decay_tick).
    pub fn decay_tick(&mut self, now: u64) {
        for entry in self.table.values_mut() {
            let dt = (now.saturating_sub(entry.last_fired)) as f32;

            // Exponential-like decay based on time since last fire
            let decay_amount = self.decay_rate * dt;
            entry.strength = (entry.strength - decay_amount).clamp(0.0, 1.0);
        }
    }

    // ------------------------------------------------------------
    // HOOKS: EPISODIC / PHOTONIC / SEMANTIC
    // ------------------------------------------------------------

    /// Placeholder: link reflex target into episodic memory.
    fn link_to_episodic(&self, _engine: &mut MemoryEngine, _target: NodeId, _now: u64) {
        // Future: create a lightweight episodic trace for reflex events.
    }

    /// Placeholder: bias photonic propagation from reflex targets.
    fn bias_photonic(&self, _engine: &mut MemoryEngine, _target: NodeId) {
        // Future: increase initial amplitude or resonance for reflex nodes.
    }

    /// Placeholder: semantic generalization (e.g., "dog" → "animal").
    pub fn generalize_reflex(&self, _stimulus: &str) -> Option<String> {
        None
    }

    // ------------------------------------------------------------
    // DEBUG / EXPORT
    // ------------------------------------------------------------

    pub fn export(&self) -> Vec<(String, NodeId, f32, u64, u64)> {
        self.table
            .values()
            .map(|r| {
                (
                    r.stimulus.clone(),
                    r.target,
                    r.strength,
                    r.usage_count,
                    r.last_fired,
                )
            })
            .collect()
    }
}

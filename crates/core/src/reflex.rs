use crate::{
    engine::MemoryEngine,
    node::NodeId,
};

use std::collections::HashMap;

/// Tier‑3 MAX Reflex entry: stimulus → target node
#[derive(Debug, Clone)]
pub struct ReflexEntry {
    pub stimulus: String,
    pub target: NodeId,

    /// Reflex intensity (0–1)
    pub strength: f32,

    /// Stability added when reflex fires
    pub stability_gain: f32,

    /// Heat added when reflex fires
    pub heat_gain: f32,

    /// Reflex usage counter
    pub usage_count: u64,

    /// Last time reflex fired
    pub last_fired: u64,

    /// Reflex resonance alignment (0–1)
    pub resonance: f32,

    /// Reflex importance (0–10)
    pub importance: f32,
}

/// Tier‑3 MAX Reflex subsystem
#[derive(Debug, Clone)]
pub struct ReflexSystem {
    pub table: HashMap<String, ReflexEntry>,

    /// Reflex decay rate
    pub decay_rate: f32,

    /// Reflex learning rate
    pub learning_rate: f32,

    /// Resonance learning rate
    pub resonance_rate: f32,

    /// Importance learning rate
    pub importance_rate: f32,
}

impl ReflexSystem {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            decay_rate: 0.0005,
            learning_rate: 0.02,
            resonance_rate: 0.015,
            importance_rate: 0.03,
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
            resonance: 0.0,
            importance: 0.0,
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
    // TRIGGERING + LEARNING (Tier‑3 MAX)
    // ------------------------------------------------------------

    pub fn trigger(&mut self, engine: &mut MemoryEngine, stimulus: &str, now: u64) {
        let key = stimulus.to_lowercase();

        if let Some(entry) = self.table.get_mut(&key) {
            let target = entry.target;

            // Reflex activation
            engine.activate(target, now, "reflex");

            // Reinforce heat + stability scaled by strength
            if let Some(state) = engine.states.get_mut(&target) {
                state.heat.short_term += entry.heat_gain * entry.strength;
                state.stability = (state.stability + entry.stability_gain * entry.strength).min(1.0);

                // Reflex resonance coupling
                entry.resonance = (entry.resonance + state.heat.resonance * self.resonance_rate).min(1.0);

                // Reflex importance grows with node importance
                entry.importance = (entry.importance + state.importance * self.importance_rate).min(10.0);
            }

            // Nonlinear reflex learning curve
            entry.strength = (entry.strength + self.learning_rate * (1.0 - entry.strength)).clamp(0.0, 1.0);

            // Usage tracking
            entry.usage_count += 1;
            entry.last_fired = now;

            // Episodic imprinting
            self.link_to_episodic(engine, target, now);

            // Photonic reflex amplification
            self.bias_photonic(engine, target);

            // Hive generalization reflex mapping
            self.bias_hive(engine, target);
        }
    }

    // ------------------------------------------------------------
    // DECAY / MAINTENANCE (Tier‑3 MAX)
    // ------------------------------------------------------------

    pub fn decay_tick(&mut self, now: u64) {
        for entry in self.table.values_mut() {
            let dt = (now.saturating_sub(entry.last_fired)) as f32;

            // Reflex strength decay
            let decay_amount = self.decay_rate * dt;
            entry.strength = (entry.strength - decay_amount).clamp(0.0, 1.0);

            // Reflex resonance decay
            entry.resonance *= f32::exp(-0.001 * dt);

            // Reflex importance decay
            entry.importance *= f32::exp(-0.002 * dt);
        }
    }

    // ------------------------------------------------------------
    // HOOKS: EPISODIC / PHOTONIC / HIVE (Tier‑3 MAX)
    // ------------------------------------------------------------

    fn link_to_episodic(&self, engine: &mut MemoryEngine, target: NodeId, now: u64) {
        // Reflex events create lightweight episodic traces
        let label = engine.graph.nodes.get(&target).map(|n| n.label.clone()).unwrap_or_default();
        engine.semantic.store_reflex_event(label, now);
    }

    fn bias_photonic(&self, engine: &mut MemoryEngine, target: NodeId) {
        // Reflex nodes get photonic amplitude boost
        if let Some(state) = engine.states.get_mut(&target) {
            state.heat.resonance = (state.heat.resonance + 0.05).min(1.0);
        }
    }

    fn bias_hive(&self, engine: &mut MemoryEngine, target: NodeId) {
        // Reflex nodes reinforce hive generalization
        if let Some(node) = engine.graph.nodes.get(&target) {
            engine.word_hive.observe_word(&node.label, &[]);
        }
    }

    // ------------------------------------------------------------
    // DEBUG / EXPORT
    // ------------------------------------------------------------

    pub fn export(&self) -> Vec<(String, NodeId, f32, f32, f32, u64, u64)> {
        self.table
            .values()
            .map(|r| {
                (
                    r.stimulus.clone(),
                    r.target,
                    r.strength,
                    r.resonance,
                    r.importance,
                    r.usage_count,
                    r.last_fired,
                )
            })
            .collect()
    }
}

use crate::{
    engine::MemoryEngine,
    node::NodeId,
    memory_linking::MemoryLinker,
    memory_reuse::MemoryReuseEngine,
};

/// Photonic propagation event
#[derive(Debug, Clone)]
pub struct PhotonicEvent {
    pub origin: NodeId,
    pub affected: NodeId,
    pub intensity: f32,
}

/// Hybrid photonic engine:
/// - propagation (light-like pulses)
/// - interference (constructive/destructive)
/// - resonance (loops → long-term memory)
/// - MAX‑tier resonance boosting (North Star physics)
#[derive(Debug)]
pub struct PhotonicPropagationEngine {
    /// Base propagation speed (conceptual; used to scale intensity).
    pub speed: f32,
    /// Loss per hop.
    pub attenuation: f32,
    /// Constructive interference multiplier.
    pub interference_gain: f32,
    /// Destructive interference multiplier.
    pub destructive_factor: f32,
    /// Loop activation threshold for resonance.
    pub resonance_threshold: f32,
    /// Minimum short-term heat to be considered “active”.
    pub activation_threshold: f32,
}

impl PhotonicPropagationEngine {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            attenuation: 0.15,
            interference_gain: 1.25,
            destructive_factor: 0.65,
            resonance_threshold: 0.75,
            activation_threshold: 0.05,
        }
    }

    /// MAX‑tier resonance booster:
    /// Called by North Star physics.
    /// Increases:
    /// - short-term heat
    /// - long-term heat
    /// - stability
    /// - importance
    /// Then runs normal resonance.
    pub fn boost_resonance(&self, engine: &mut MemoryEngine, id: NodeId, bias: f32) {
        if let Some(state) = engine.states.get_mut(&id) {
            // Increase photonic amplitude
            state.heat.short_term += bias * 0.25;
            state.heat.long_term += bias * 0.35;

            // Increase stability
            state.stability = (state.stability + bias * 0.15).min(1.0);

            // Increase importance
            state.importance = (state.importance + bias * 0.20).min(10.0);
        }

        // Run normal resonance afterward
        self.resonance(engine);
    }

    /// Propagate activation like photons across edges.
    /// Uses origin short-term heat as base intensity.
    pub fn propagate(&self, engine: &mut MemoryEngine, origin: NodeId) -> Vec<PhotonicEvent> {
        let mut events = Vec::new();

        // Base intensity from origin state.
        let base_intensity = engine
            .states
            .get(&origin)
            .map(|s| s.heat.short_term * self.speed)
            .unwrap_or(0.0);

        if base_intensity < self.activation_threshold {
            return events;
        }

        // Clone edges to avoid borrow conflicts.
        let edges: Vec<_> = engine.graph.edges.values().cloned().collect();

        for edge in edges {
            if edge.from == origin {
                // Single-hop attenuation.
                let intensity = base_intensity * edge.weight * (1.0 - self.attenuation);

                if intensity > self.activation_threshold {
                    // Use origin's last_access as time base for activation.
                    let now = engine
                        .states
                        .get(&origin)
                        .map(|s| s.last_access)
                        .unwrap_or(0);

                    engine.activate(edge.to, now, "photonic");

                    events.push(PhotonicEvent {
                        origin,
                        affected: edge.to,
                        intensity,
                    });
                }
            }
        }

        events
    }

    /// Interference: waves meeting create constructive or destructive effects.
    /// Uses short-term heat overlap to decide link strengthening/weakening.
    pub fn interference(&self, engine: &mut MemoryEngine) {
        let linker = MemoryLinker::new();

        let ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = ids[i];
                let b = ids[j];

                let sa = match engine.states.get(&a) {
                    Some(s) => s,
                    None => continue,
                };
                let sb = match engine.states.get(&b) {
                    Some(s) => s,
                    None => continue,
                };

                let overlap = sa.heat.short_term * sb.heat.short_term;

                if overlap > 0.5 {
                    // Constructive interference → stronger link via linker.
                    linker.try_link(engine, a, b);
                } else if overlap < 0.05 {
                    // Destructive interference → weaken existing links.
                    let edges: Vec<_> = engine.graph.edges.values_mut().collect();
                    for edge in edges {
                        if (edge.from == a && edge.to == b)
                            || (edge.from == b && edge.to == a)
                        {
                            edge.weight *= self.destructive_factor;
                        }
                    }
                }
            }
        }
    }

    /// Resonance: loops become long-term memory.
    /// Uses long-term heat + stability as resonance score.
    pub fn resonance(&self, engine: &mut MemoryEngine) {
        let reuse = MemoryReuseEngine::new();

        let ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for id in ids {
            let state = match engine.states.get(&id) {
                Some(s) => s,
                None => continue,
            };

            let resonance_score = state.heat.long_term * 0.5 + state.stability * 0.5;

            if resonance_score >= self.resonance_threshold {
                reuse.boost(engine, id);
            }
        }
    }

    /// Full hybrid photonic tick.
    pub fn photonic_tick(&self, engine: &mut MemoryEngine, origin: NodeId) {
        // 1. Propagate activation like photons.
        let _events = self.propagate(engine, origin);

        // 2. Interference patterns create or weaken links.
        self.interference(engine);

        // 3. Resonance strengthens stable nodes.
        self.resonance(engine);
    }
}



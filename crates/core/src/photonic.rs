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
#[derive(Debug)]
pub struct PhotonicPropagationEngine {
    pub speed: f32,               // propagation speed
    pub attenuation: f32,         // loss per hop
    pub interference_gain: f32,   // constructive interference multiplier
    pub destructive_factor: f32,  // destructive interference multiplier
    pub resonance_threshold: f32, // loop activation threshold
}

impl PhotonicPropagationEngine {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            attenuation: 0.15,
            interference_gain: 1.25,
            destructive_factor: 0.65,
            resonance_threshold: 0.75,
        }
    }

    /// Propagate activation like photons across edges
    pub fn propagate(&self, engine: &mut MemoryEngine, origin: NodeId) -> Vec<PhotonicEvent> {
        let mut events = Vec::new();

        let edges: Vec<_> = engine.graph.edges.values().cloned().collect();

        for edge in edges {
            if edge.from == origin {
                let intensity = edge.weight * (1.0 - self.attenuation);

                if intensity > 0.01 {
                    engine.activate_main(edge.to, intensity as u64);

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

    /// Interference: waves meeting create constructive or destructive effects
    pub fn interference(&self, engine: &mut MemoryEngine) {
        let linker = MemoryLinker::new();

        let ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = ids[i];
                let b = ids[j];

                let sa = &engine.states[&a];
                let sb = &engine.states[&b];

                let overlap = sa.heat.short_term * sb.heat.short_term;

                if overlap > 0.5 {
                    // Constructive interference → stronger link
                    linker.try_link(engine, a, b);
                } else if overlap < 0.05 {
                    // Destructive interference → weaken link
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

    /// Resonance: loops become long-term memory
    pub fn resonance(&self, engine: &mut MemoryEngine) {
        let reuse = MemoryReuseEngine::new();

        let ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for id in ids {
            let state = &engine.states[&id];

            // Resonance = repeated activation + stability
            let resonance_score =
                state.heat.long_term * 0.5 + state.stability * 0.5;

            if resonance_score >= self.resonance_threshold {
                reuse.boost(engine, id);
            }
        }
    }

    /// Full hybrid photonic tick
    pub fn photonic_tick(&self, engine: &mut MemoryEngine, origin: NodeId) {
        // 1. Propagate activation like photons
        let _events = self.propagate(engine, origin);

        // 2. Interference patterns create or weaken links
        self.interference(engine);

        // 3. Resonance strengthens stable nodes
        self.resonance(engine);
    }
}

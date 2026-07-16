use crate::{
    engine::MemoryEngine,
    node::NodeId,
    memory_reuse::MemoryReuseEngine,
};

/// Consolidation event: long-term anchor formation
#[derive(Debug, Clone)]
pub struct ConsolidationEvent {
    pub id: NodeId,
    pub anchor: bool,
}

/// Drift event: memory weakening or transformation
#[derive(Debug, Clone)]
pub struct DriftEvent {
    pub id: NodeId,
    pub drift_amount: f32,
}

/// Clustering event: node assigned to a semantic cluster
#[derive(Debug, Clone)]
pub struct ClusterEvent {
    pub id: NodeId,
    pub cluster_id: usize,
}

/// Fractal Echo event: pattern reinforcement
#[derive(Debug, Clone)]
pub struct FractalEchoEvent {
    pub id: NodeId,
    pub echo_strength: f32,
}

/// Memory Consolidation Engine
#[derive(Debug, Clone)]
pub struct MemoryConsolidationEngine {
    pub stability_threshold: f32,
    pub long_term_threshold: f32,
    pub anchor_boost: f32,
}

impl MemoryConsolidationEngine {
    pub fn new() -> Self {
        Self {
            stability_threshold: 0.8,
            long_term_threshold: 0.7,
            anchor_boost: 0.2,
        }
    }

    pub fn consolidate(&self, engine: &mut MemoryEngine) -> Vec<ConsolidationEvent> {
        let mut events = Vec::new();
        let reuse = MemoryReuseEngine::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();

        for id in ids {
            let state = &engine.states[&id];

            let score = state.stability * 0.6 + state.heat.long_term * 0.4;

            if score >= self.stability_threshold && state.heat.long_term >= self.long_term_threshold {
                reuse.boost(engine, id);

                events.push(ConsolidationEvent {
                    id,
                    anchor: true,
                });
            }
        }

        events
    }
}

/// Memory Drift Engine
#[derive(Debug, Clone)]
pub struct MemoryDriftEngine {
    pub drift_rate: f32,
    pub volatility_factor: f32,
}

impl MemoryDriftEngine {
    pub fn new() -> Self {
        Self {
            drift_rate: 0.01,
            volatility_factor: 0.2,
        }
    }

    pub fn drift(&self, engine: &mut MemoryEngine) -> Vec<DriftEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();

        for id in ids {
            let state = engine.states.get_mut(&id).unwrap();

            let volatility = 1.0 - state.stability;
            let drift_amount = self.drift_rate * (1.0 + self.volatility_factor * volatility);

            state.heat.long_term *= f32::exp(-drift_amount);
            state.stability *= f32::exp(-drift_amount * 0.5);

            events.push(DriftEvent {
                id,
                drift_amount,
            });
        }

        events
    }
}

/// Memory Clustering Engine
#[derive(Debug, Clone)]
pub struct MemoryClusteringEngine {
    pub cluster_count: usize,
}

impl MemoryClusteringEngine {
    pub fn new(cluster_count: usize) -> Self {
        Self { cluster_count }
    }

    pub fn cluster(&self, engine: &mut MemoryEngine) -> Vec<ClusterEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();
        if ids.is_empty() || self.cluster_count == 0 {
            return events;
        }

        let mut scored: Vec<(NodeId, f32)> = ids
            .iter()
            .map(|id| {
                let state = &engine.states[id];
                let score = state.stability * 0.6 + state.heat.long_term * 0.4;
                (*id, score)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let bucket_size = (scored.len() as f32 / self.cluster_count as f32).ceil() as usize;

        for (idx, (id, _)) in scored.into_iter().enumerate() {
            let cluster_id = idx / bucket_size;
            events.push(ClusterEvent { id, cluster_id });
        }

        events
    }
}

/// Fractal Echo Engine
/// - reinforces repeating activation patterns
/// - strengthens nodes that match system-wide activation rhythms
#[derive(Debug, Clone)]
pub struct FractalEchoEngine {
    pub echo_gain: f32,
}

impl FractalEchoEngine {
    pub fn new() -> Self {
        Self {
            echo_gain: 0.12,
        }
    }

    pub fn echo(&self, engine: &mut MemoryEngine) -> Vec<FractalEchoEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();

        let global_rhythm: f32 = ids
            .iter()
            .map(|id| engine.states[id].heat.short_term)
            .sum::<f32>()
            / ids.len().max(1) as f32;

        for id in ids {
            let state = engine.states.get_mut(&id).unwrap();

            let local_rhythm = state.heat.short_term;

            let similarity = 1.0 - (global_rhythm - local_rhythm).abs();

            if similarity > 0.6 {
                let echo_strength = similarity * self.echo_gain;

                state.stability = (state.stability + echo_strength).min(1.0);
                state.heat.long_term += echo_strength * 0.5;

                events.push(FractalEchoEvent {
                    id,
                    echo_strength,
                });
            }
        }

        events
    }
}

/// High-level cognition pass: consolidation + drift + clustering + fractal echo
#[derive(Debug, Clone)]
pub struct MemoryCognitionEngine {
    pub consolidation: MemoryConsolidationEngine,
    pub drift: MemoryDriftEngine,
    pub clustering: MemoryClusteringEngine,
    pub fractal_echo: FractalEchoEngine,
}

impl MemoryCognitionEngine {
    pub fn new(cluster_count: usize) -> Self {
        Self {
            consolidation: MemoryConsolidationEngine::new(),
            drift: MemoryDriftEngine::new(),
            clustering: MemoryClusteringEngine::new(cluster_count),
            fractal_echo: FractalEchoEngine::new(),
        }
    }

    pub fn cognition_tick(&self, engine: &mut MemoryEngine) {
        let _ = self.consolidate(engine);
        let _ = self.drift(engine);
        let _ = self.cluster(engine);
        let _ = self.fractal_echo.echo(engine);
    }

    pub fn consolidate(&self, engine: &mut MemoryEngine) -> Vec<ConsolidationEvent> {
        self.consolidation.consolidate(engine)
    }

    pub fn drift(&self, engine: &mut MemoryEngine) -> Vec<DriftEvent> {
        self.drift.drift(engine)
    }

    pub fn cluster(&self, engine: &mut MemoryEngine) -> Vec<ClusterEvent> {
        self.clustering.cluster(engine)
    }
}



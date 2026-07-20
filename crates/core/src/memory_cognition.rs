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
    pub score: f32,
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
    pub score: f32,
}

/// Fractal Echo event: pattern reinforcement
#[derive(Debug, Clone)]
pub struct FractalEchoEvent {
    pub id: NodeId,
    pub echo_strength: f32,
}

/// Diverging branch kinds for visual memory
#[derive(Debug, Clone, Copy)]
pub enum BranchKind {
    Shape,
    Color,
    Context,
    Motion,
    Semantic,
}

/// Diverging branch event: node splits into a new branch interpretation
#[derive(Debug, Clone)]
pub struct DivergingBranchEvent {
    pub id: NodeId,
    pub branch_kind: BranchKind,
    pub divergence_score: f32,
}

/// Memory Consolidation Engine
#[derive(Debug, Clone)]
pub struct MemoryConsolidationEngine {
    pub stability_threshold: f32,
    pub long_term_threshold: f32,
    pub importance_threshold: f32,
    pub anchor_boost: f32,
}

impl MemoryConsolidationEngine {
    pub fn new() -> Self {
        Self {
            stability_threshold: 0.8,
            long_term_threshold: 0.7,
            importance_threshold: 2.0,
            anchor_boost: 0.2,
        }
    }

    pub fn consolidate(&self, engine: &mut MemoryEngine) -> Vec<ConsolidationEvent> {
        let mut events = Vec::new();
        let reuse = MemoryReuseEngine::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();

        for id in ids {
            let state = &engine.states[&id];

            let score = state.stability * 0.5
                + state.heat.long_term * 0.3
                + state.importance * 0.2;

            if score >= self.stability_threshold
                && state.heat.long_term >= self.long_term_threshold
                && state.importance >= self.importance_threshold
            {
                reuse.boost(engine, id);

                if let Some(s) = engine.states.get_mut(&id) {
                    s.stability = (s.stability + self.anchor_boost).min(1.0);
                    s.heat.long_term += self.anchor_boost * 0.5;
                    s.importance = (s.importance + self.anchor_boost).min(10.0);
                }

                events.push(ConsolidationEvent {
                    id,
                    anchor: true,
                    score,
                });
            }
        }

        events
    }
}

/// Memory Drift Engine
#[derive(Debug, Clone)]
pub struct MemoryDriftEngine {
    pub base_drift_rate: f32,
    pub volatility_factor: f32,
    pub importance_protection: f32,
}

impl MemoryDriftEngine {
    pub fn new() -> Self {
        Self {
            base_drift_rate: 0.01,
            volatility_factor: 0.2,
            importance_protection: 0.5,
        }
    }

    pub fn drift(&self, engine: &mut MemoryEngine) -> Vec<DriftEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();

        for id in ids {
            let state = match engine.states.get_mut(&id) {
                Some(s) => s,
                None => continue,
            };

            let volatility = 1.0 - state.stability;
            let importance_factor = (10.0 - state.importance).max(0.0) / 10.0;

            let drift_amount = self.base_drift_rate
                * (1.0 + self.volatility_factor * volatility)
                * (self.importance_protection * importance_factor + (1.0 - self.importance_protection));

            state.heat.long_term *= f32::exp(-drift_amount);
            state.stability *= f32::exp(-drift_amount * 0.5);
            state.importance *= f32::exp(-drift_amount * 0.3);

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
                let score = state.stability * 0.4
                    + state.heat.long_term * 0.3
                    + state.importance * 0.3;
                (*id, score)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let bucket_size = (scored.len() as f32 / self.cluster_count as f32).ceil() as usize;

        for (idx, (id, score)) in scored.into_iter().enumerate() {
            let cluster_id = idx / bucket_size;
            events.push(ClusterEvent { id, cluster_id, score });
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
    pub importance_gain: f32,
}

impl FractalEchoEngine {
    pub fn new() -> Self {
        Self {
            echo_gain: 0.12,
            importance_gain: 0.08,
        }
    }

    pub fn echo(&self, engine: &mut MemoryEngine) -> Vec<FractalEchoEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();
        if ids.is_empty() {
            return events;
        }

        let global_rhythm: f32 = ids
            .iter()
            .map(|id| engine.states[id].heat.short_term)
            .sum::<f32>()
            / ids.len().max(1) as f32;

        for id in ids {
            let state = match engine.states.get_mut(&id) {
                Some(s) => s,
                None => continue,
            };

            let local_rhythm = state.heat.short_term;
            let similarity = 1.0 - (global_rhythm - local_rhythm).abs();

            if similarity > 0.6 {
                let echo_strength = similarity * self.echo_gain;

                state.stability = (state.stability + echo_strength).min(1.0);
                state.heat.long_term += echo_strength * 0.5;
                state.importance = (state.importance + similarity * self.importance_gain).min(10.0);

                events.push(FractalEchoEvent {
                    id,
                    echo_strength,
                });
            }
        }

        events
    }
}

/// Diverging Branch Engine
/// - identifies nodes that should split into multiple visual/cognitive interpretations
#[derive(Debug, Clone)]
pub struct DivergingBranchEngine {
    pub divergence_threshold: f32,
    pub volatility_threshold: f32,
    pub max_branches_per_tick: usize,
}

impl DivergingBranchEngine {
    pub fn new() -> Self {
        Self {
            divergence_threshold: 0.65,
            volatility_threshold: 0.35,
            max_branches_per_tick: 32,
        }
    }

    pub fn diverge(&self, engine: &mut MemoryEngine) -> Vec<DivergingBranchEvent> {
        let mut events = Vec::new();

        let ids: Vec<NodeId> = engine.states.keys().copied().collect();
        if ids.is_empty() {
            return events;
        }

        // Sort by "tension" between importance and stability:
        // high importance + mid/low stability → good divergence candidates.
        let mut scored: Vec<(NodeId, f32)> = ids
            .iter()
            .map(|id| {
                let state = &engine.states[id];
                let volatility = 1.0 - state.stability;
                let tension = (state.importance / 10.0) * volatility;
                (*id, tension)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut branch_count = 0;

        for (id, tension) in scored {
            if branch_count >= self.max_branches_per_tick {
                break;
            }

            if tension < self.divergence_threshold {
                continue;
            }

            let state = match engine.states.get_mut(&id) {
                Some(s) => s,
                None => continue,
            };

            let volatility = 1.0 - state.stability;
            if volatility < self.volatility_threshold {
                continue;
            }

            // Decide branch kind based on relative weights of heat + importance.
            let short = state.heat.short_term;
            let long = state.heat.long_term;
            let importance = state.importance;

            let branch_kind = if short > long && importance > 5.0 {
                BranchKind::Motion
            } else if long > short && importance > 5.0 {
                BranchKind::Context
            } else if importance >= 7.5 {
                BranchKind::Semantic
            } else if short >= long {
                BranchKind::Shape
            } else {
                BranchKind::Color
            };

            let divergence_score = tension;

            // Slightly adjust state to reflect branching pressure without breaking existing behavior.
            state.stability *= f32::exp(-0.03 * divergence_score);
            state.heat.short_term *= 1.0 + 0.05 * divergence_score;
            state.heat.long_term *= 1.0 + 0.03 * divergence_score;

            events.push(DivergingBranchEvent {
                id,
                branch_kind,
                divergence_score,
            });

            branch_count += 1;
        }

        events
    }
}

/// High-level cognition pass: consolidation + drift + clustering + fractal echo + divergence
#[derive(Debug, Clone)]
pub struct MemoryCognitionEngine {
    pub consolidation: MemoryConsolidationEngine,
    pub drift: MemoryDriftEngine,
    pub clustering: MemoryClusteringEngine,
    pub fractal_echo: FractalEchoEngine,
    pub divergence: DivergingBranchEngine,
}

impl MemoryCognitionEngine {
    pub fn new(cluster_count: usize) -> Self {
        Self {
            consolidation: MemoryConsolidationEngine::new(),
            drift: MemoryDriftEngine::new(),
            clustering: MemoryClusteringEngine::new(cluster_count),
            fractal_echo: FractalEchoEngine::new(),
            divergence: DivergingBranchEngine::new(),
        }
    }

    pub fn cognition_tick(&self, engine: &mut MemoryEngine) {
        let _ = self.consolidate(engine);
        let _ = self.drift(engine);
        let _ = self.cluster(engine);
        let _ = self.fractal_echo.echo(engine);
        let _ = self.divergence.diverge(engine);
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



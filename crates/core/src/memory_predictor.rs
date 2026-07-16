use crate::{
    engine::MemoryEngine,
    node::NodeId,
    edge::EdgeId,
};

/// Single prediction for next likely activation
#[derive(Debug, Clone)]
pub struct Prediction {
    pub node: Option<NodeId>,
    pub edge: Option<EdgeId>,
    pub lane: Option<String>,
    pub score: f32,
}

/// Multi-step prediction sequence
#[derive(Debug, Clone)]
pub struct PredictionSequence {
    pub steps: Vec<Prediction>,
}

/// Memory predictor: forecasts next activation based on current graph state
#[derive(Debug)]
pub struct MemoryPredictor {
    /// Weighting factors for scoring
    pub w_short: f32,
    pub w_long: f32,
    pub w_stability: f32,
    pub w_semantic: f32,
    pub w_confidence: f32,
    pub w_edge_weight: f32,
    pub w_edge_confidence: f32,
    pub w_edge_activation: f32,
}

impl MemoryPredictor {
    pub fn new() -> Self {
        Self {
            w_short: 1.5,
            w_long: 2.0,
            w_stability: 1.2,
            w_semantic: 1.3,
            w_confidence: 1.0,
            w_edge_weight: 2.0,
            w_edge_confidence: 1.5,
            w_edge_activation: 0.1,
        }
    }

    /// Compute node score
    fn score_node(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let state = match engine.states.get(id) {
            Some(s) => s,
            None => return -1.0,
        };

        let node = match engine.graph.nodes.get(id) {
            Some(n) => n,
            None => return -1.0,
        };

        self.w_short * state.heat.short_term +
        self.w_long * state.heat.long_term +
        self.w_stability * state.stability +
        self.w_semantic * node.semantic_weight +
        self.w_confidence * node.confidence
    }

    /// Compute edge score
    fn score_edge(&self, engine: &MemoryEngine, eid: &EdgeId) -> f32 {
        let edge = match engine.graph.edges.get(eid) {
            Some(e) => e,
            None => return -1.0,
        };

        self.w_edge_weight * edge.weight +
        self.w_edge_confidence * edge.confidence +
        self.w_edge_activation * (edge.activation_count as f32)
    }

    /// Compute lane score
    fn score_lane(&self, engine: &MemoryEngine, label: &str) -> f32 {
        let lane = match engine.scratchpad.lanes.get(label) {
            Some(l) => l,
            None => return -1.0,
        };

        // Simple heuristic: pinned nodes matter more than recent
        (lane.recent.len() as f32) * 1.0 + (lane.pinned.len() as f32) * 2.0
    }

    /// Predict a single next activation
    pub fn predict_next(&self, engine: &MemoryEngine) -> Prediction {
        // Node
        let mut best_node: Option<NodeId> = None;
        let mut best_node_score: f32 = -1.0;

        for id in engine.states.keys() {
            let score = self.score_node(engine, id);
            if score > best_node_score {
                best_node_score = score;
                best_node = Some(*id);
            }
        }

        // Edge
        let mut best_edge: Option<EdgeId> = None;
        let mut best_edge_score: f32 = -1.0;

        for eid in engine.graph.edges.keys() {
            let score = self.score_edge(engine, eid);
            if score > best_edge_score {
                best_edge_score = score;
                best_edge = Some(*eid);
            }
        }

        // Lane
        let mut best_lane: Option<String> = None;
        let mut best_lane_score: f32 = -1.0;

        for label in engine.scratchpad.lanes.keys() {
            let score = self.score_lane(engine, label);
            if score > best_lane_score {
                best_lane_score = score;
                best_lane = Some(label.clone());
            }
        }

        // Combined score (simple sum of bests)
        let total_score = best_node_score.max(0.0)
            + best_edge_score.max(0.0)
            + best_lane_score.max(0.0);

        Prediction {
            node: best_node,
            edge: best_edge,
            lane: best_lane,
            score: total_score,
        }
    }

    /// Predict a multi-step sequence by repeatedly sampling without modifying engine
    pub fn predict_sequence(&self, engine: &MemoryEngine, steps: usize) -> PredictionSequence {
        let mut out = Vec::new();

        // We’ll just recompute each step from the same state for now
        // (you can later add simulated activation/decay for more realism)
        for _ in 0..steps {
            let p = self.predict_next(engine);
            out.push(p);
        }

        PredictionSequence { steps: out }
    }
}

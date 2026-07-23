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

/// Tier‑3 MAX + Tier‑7 Roundabout + Tier‑8 Geometry + Tier‑9 Temporal Forecasting
#[derive(Debug)]
pub struct MemoryPredictor {
    pub w_short: f32,
    pub w_long: f32,
    pub w_stability: f32,
    pub w_importance: f32,
    pub w_resonance: f32,
    pub w_inertia: f32,
    pub w_semantic: f32,
    pub w_confidence: f32,
    pub w_edge_weight: f32,
    pub w_edge_confidence: f32,
    pub w_edge_activation: f32,
    pub w_edge_age: f32,
    pub w_hive: f32,
    pub w_cluster: f32,
    pub w_lane_resonance: f32,
    pub w_ep_mem: f32,

    /// Roundabout / geometry weights
    pub w_gravity: f32,
    pub w_zone_match: f32,
    pub w_drift_penalty: f32,

    /// Tier‑9 temporal weights
    pub w_recency: f32,
    pub w_temporal_stability: f32,
    pub w_temporal_wave: f32,
}

impl MemoryPredictor {
    pub fn new() -> Self {
        Self {
            w_short: 1.5,
            w_long: 2.0,
            w_stability: 1.2,
            w_importance: 1.8,
            w_resonance: 1.6,
            w_inertia: 0.8,
            w_semantic: 1.3,
            w_confidence: 1.0,
            w_edge_weight: 2.0,
            w_edge_confidence: 1.5,
            w_edge_activation: 0.1,
            w_edge_age: -0.02,
            w_hive: 1.4,
            w_cluster: 1.2,
            w_lane_resonance: 1.1,
            w_ep_mem: 1.0,

            w_gravity: 1.3,
            w_zone_match: 1.1,
            w_drift_penalty: 0.4,

            w_recency: 1.4,
            w_temporal_stability: 1.3,
            w_temporal_wave: 1.2,
        }
    }

    /// Hive + cluster semantic gravity
    fn hive_cluster_bonus(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let node = match engine.graph.nodes.get(id) {
            Some(n) => n,
            None => return 0.0,
        };

        let hive = engine.word_hive.generalize_word(&node.label);

        let mut hive_bonus = if hive == node.label { 1.0 } else { 1.3 };

        if let Some(cid) = engine.word_hive.word_to_cluster.get(&node.label.to_lowercase()) {
            if let Some(cluster) = engine.word_hive.clusters.get(cid) {
                hive_bonus += cluster.strength * self.w_cluster;
            }
        }

        hive_bonus * self.w_hive
    }

    /// Episodic memory influence
    fn episodic_bonus(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let node = match engine.graph.nodes.get(id) {
            Some(n) => n,
            None => return 0.0,
        };

        let label_lc = node.label.to_lowercase();
        let mut score = 0.0;

        for ep in engine.semantic.episodes.values() {
            let summary = ep.compressed_summary.to_lowercase();
            if summary.contains(&label_lc) {
                let sim = Self::similarity(&summary, &label_lc);
                score += sim * self.w_ep_mem;
            }
        }

        score
    }

    /// Simple semantic similarity
    fn similarity(a: &str, b: &str) -> f32 {
        let a = a.to_lowercase();
        let b = b.to_lowercase();
        if a == b {
            return 1.0;
        }
        let overlap = a.chars().filter(|c| b.contains(*c)).count();
        let total = a.len().max(b.len());
        overlap as f32 / total as f32
    }

    /// Roundabout / geometry bonus for a node
    fn roundabout_bonus(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let node = match engine.graph.nodes.get(id) {
            Some(n) => n,
            None => return 0.0,
        };

        let gravity_term = node.gravity * self.w_gravity;

        let mut zone_term = 0.0;
        if let Some(main_lane) = engine.scratchpad.lanes.get("main") {
            if let Some(last_id) = main_lane.recent.back() {
                if let Some(last_node) = engine.graph.nodes.get(last_id) {
                    if last_node.zone == node.zone {
                        zone_term = self.w_zone_match;
                    }
                }
            }
        }

        gravity_term + zone_term
    }

    /// Drift penalty based on heat drift vector
    fn drift_penalty(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let state = match engine.states.get(id) {
            Some(s) => s,
            None => return 1.0,
        };

        let drift_mag = (state.heat.drift_dx.abs() + state.heat.drift_dy.abs()).min(1.0);
        1.0 / (1.0 + drift_mag * self.w_drift_penalty)
    }

    /// Tier‑9: temporal recency + wave alignment
    fn temporal_bonus(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let state = match engine.states.get(id) {
            Some(s) => s,
            None => return 0.0,
        };

        // Recency: use node's own last_access as time reference
        let now = state.last_access as f32;

        // Compute global wave center from all nodes
        let ids: Vec<NodeId> = engine.states.keys().copied().collect();
        let mut wave_center = 0.0;
        if !ids.is_empty() {
            let sum: f32 = ids
                .iter()
                .map(|nid| engine.states[nid].last_access as f32)
                .sum();
            wave_center = sum / ids.len() as f32;
        }

        let dt = (wave_center - now).abs();
        let recency = (1.0 / (1.0 + dt * 0.02)) * self.w_recency;

        let temporal_stability = state.heat.temporal_stability * self.w_temporal_stability;

        let wave_diff = (wave_center - now).abs().min(200.0);
        let wave_alignment = (1.0 - (wave_diff / 200.0)) * self.w_temporal_wave;

        recency + temporal_stability + wave_alignment
    }

    /// Compute node score (Tier‑3 + Tier‑7 + Tier‑8 + Tier‑9)
    fn score_node(&self, engine: &MemoryEngine, id: &NodeId) -> f32 {
        let state = match engine.states.get(id) {
            Some(s) => s,
            None => return -1.0,
        };

        let node = match engine.graph.nodes.get(id) {
            Some(n) => n,
            None => return -1.0,
        };

        let hive_cluster = self.hive_cluster_bonus(engine, id);
        let ep_bonus = self.episodic_bonus(engine, id);
        let roundabout = self.roundabout_bonus(engine, id);
        let drift_penalty = self.drift_penalty(engine, id);
        let temporal = self.temporal_bonus(engine, id);

        let base =
            self.w_short * state.heat.short_term +
            self.w_long * state.heat.long_term +
            self.w_stability * state.stability +
            self.w_importance * state.importance +
            self.w_resonance * state.heat.resonance +
            self.w_inertia * state.heat.inertia +
            self.w_semantic * node.semantic_weight +
            self.w_confidence * node.confidence +
            hive_cluster +
            ep_bonus +
            roundabout +
            temporal;

        base * drift_penalty
    }

    /// Compute edge score
    fn score_edge(&self, engine: &MemoryEngine, eid: &EdgeId) -> f32 {
        let edge = match engine.graph.edges.get(eid) {
            Some(e) => e,
            None => return -1.0,
        };

        self.w_edge_weight * edge.weight +
        self.w_edge_confidence * edge.confidence +
        self.w_edge_activation * (edge.activation_count as f32) +
        self.w_edge_age * (edge.age as f32)
    }

    /// Compute lane score
    fn score_lane(&self, engine: &MemoryEngine, label: &str) -> f32 {
        let lane = match engine.scratchpad.lanes.get(label) {
            Some(l) => l,
            None => return -1.0,
        };

        let pinned_score = lane.pinned.len() as f32 * 2.5;
        let recent_score = lane.recent.len() as f32 * 1.2;

        let mut resonance_sum = 0.0;
        let mut count = 0;

        for id in lane.recent.iter() {
            if let Some(state) = engine.states.get(id) {
                resonance_sum += state.heat.resonance;
                count += 1;
            }
        }

        let lane_resonance = if count > 0 {
            (resonance_sum / count as f32) * self.w_lane_resonance
        } else {
            0.0
        };

        pinned_score + recent_score + lane_resonance
    }

    /// Predict a single next activation (Tier‑9)
    pub fn predict_next(&self, engine: &MemoryEngine) -> Prediction {
        let mut best_node: Option<NodeId> = None;
        let mut best_node_score: f32 = -1.0;

        for id in engine.states.keys() {
            let score = self.score_node(engine, id);
            if score > best_node_score {
                best_node_score = score;
                best_node = Some(*id);
            }
        }

        let mut best_edge: Option<EdgeId> = None;
        let mut best_edge_score: f32 = -1.0;

        for eid in engine.graph.edges.keys() {
            let score = self.score_edge(engine, eid);
            if score > best_edge_score {
                best_edge_score = score;
                best_edge = Some(*eid);
            }
        }

        let mut best_lane: Option<String> = None;
        let mut best_lane_score: f32 = -1.0;

        for label in engine.scratchpad.lanes.keys() {
            let score = self.score_lane(engine, label);
            if score > best_lane_score {
                best_lane_score = score;
                best_lane = Some(label.clone());
            }
        }

        let total_score =
            best_node_score.max(0.0) +
            best_edge_score.max(0.0) +
            best_lane_score.max(0.0);

        Prediction {
            node: best_node,
            edge: best_edge,
            lane: best_lane,
            score: total_score,
        }
    }

    /// Predict a multi-step sequence (Tier‑9)
    pub fn predict_sequence(&self, engine: &MemoryEngine, steps: usize) -> PredictionSequence {
        let mut out = Vec::new();
        for _ in 0..steps {
            out.push(self.predict_next(engine));
        }
        PredictionSequence { steps: out }
    }
}



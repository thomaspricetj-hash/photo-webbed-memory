use crate::{
    engine::MemoryEngine,
    node::NodeId,
    edge::{EdgeId, EdgeKind},
};

/// Result of a link formation event
#[derive(Debug, Clone)]
pub struct LinkEvent {
    pub from: NodeId,
    pub to: NodeId,
    pub kind: EdgeKind,
    pub weight: f32,
}

/// Tier‑3 Memory Linker: synapse formation + semantic gravity + resonance coupling
#[derive(Debug)]
pub struct MemoryLinker {
    /// Weight multipliers
    pub w_coactivate: f32,
    pub w_temporal: f32,
    pub w_semantic: f32,
    pub w_stability: f32,
    pub w_importance: f32,
    pub w_resonance: f32,

    /// Minimum score required to form a link
    pub threshold: f32,

    /// Decay rate for weak links
    pub link_decay_rate: f32,

    /// Minimum weight before pruning
    pub prune_threshold: f32,
}

impl MemoryLinker {
    pub fn new() -> Self {
        Self {
            w_coactivate: 1.5,
            w_temporal: 1.2,
            w_semantic: 1.3,
            w_stability: 1.0,
            w_importance: 1.4,
            w_resonance: 1.25,
            threshold: 2.0,
            link_decay_rate: 0.01,
            prune_threshold: 0.05,
        }
    }

    /// Co-activation score between two nodes
    fn score_coactivation(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];
        (sa.heat.short_term * sb.heat.short_term) * self.w_coactivate
    }

    /// Temporal proximity score
    fn score_temporal(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];
        let dt = (sa.last_access as i64 - sb.last_access as i64).abs() as f32;
        if dt > 50.0 {
            return 0.0;
        }
        (50.0 - dt) / 50.0 * self.w_temporal
    }

    /// Semantic similarity score (Tier‑3: hive + cluster aware)
    fn score_semantic(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let na = &engine.graph.nodes[&a];
        let nb = &engine.graph.nodes[&b];

        let kind_bonus = if na.kind == nb.kind { 1.0 } else { 0.3 };

        let hive_a = engine.word_hive.generalize_word(&na.label);
        let hive_b = engine.word_hive.generalize_word(&nb.label);

        let hive_bonus = if hive_a == hive_b { 1.0 } else { 0.4 };

        (na.semantic_weight.min(nb.semantic_weight)) * kind_bonus * hive_bonus * self.w_semantic
    }

    /// Stability score
    fn score_stability(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];
        (sa.stability + sb.stability) * 0.5 * self.w_stability
    }

    /// Importance score (Tier‑3)
    fn score_importance(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let ia = engine.states[&a].importance;
        let ib = engine.states[&b].importance;
        ((ia + ib) * 0.5) * self.w_importance
    }

    /// Resonance score (Tier‑3)
    fn score_resonance(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let ra = engine.states[&a].heat.resonance;
        let rb = engine.states[&b].heat.resonance;
        ((ra + rb) * 0.5) * self.w_resonance
    }

    /// Total link score (Tier‑3)
    fn score_link(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        self.score_coactivation(engine, a, b)
            + self.score_temporal(engine, a, b)
            + self.score_semantic(engine, a, b)
            + self.score_stability(engine, a, b)
            + self.score_importance(engine, a, b)
            + self.score_resonance(engine, a, b)
    }

    // ---------------------------------------------------------
    // Tier‑7 Roundabout Routing: stability‑first link selection
    // ---------------------------------------------------------
    fn roundabout_route(
        &self,
        engine: &MemoryEngine,
        a: NodeId,
        b: NodeId,
        raw_score: f32,
    ) -> f32 {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];

        // 1. Drift penalty (Tier‑7) — use heat drift vector magnitude
        let drift_a = (sa.heat.drift_dx.abs() + sa.heat.drift_dy.abs()).min(1.0);
        let drift_b = (sb.heat.drift_dx.abs() + sb.heat.drift_dy.abs()).min(1.0);
        let drift = (drift_a - drift_b).abs();
        let drift_penalty = 1.0 / (1.0 + drift * 0.15);

        // 2. Stability gate (Tier‑7)
        let stability_gate = ((sa.stability + sb.stability) * 0.5).min(1.0);

        // 3. Heatmap bias (Tier‑7)
        let ha = sa.heat.long_term;
        let hb = sb.heat.long_term;
        let heat_bias = 1.0 + ((ha + hb) * 0.12);

        // 4. Circulation logic (Tier‑7)
        let mut circulation_boost = 1.0;
        if raw_score < self.threshold {
            circulation_boost = 1.0 + (engine.scratchpad.circulations as f32 * 0.05);
        }

        // 5. Polygon / semantic zone bias (Tier‑7)
        let na = &engine.graph.nodes[&a];
        let nb = &engine.graph.nodes[&b];
        let zone_bias = if na.zone == nb.zone { 1.25 } else { 0.85 };

        // Final roundabout score
        raw_score * drift_penalty * stability_gate * heat_bias * circulation_boost * zone_bias
    }

    /// Try to form or reinforce a link between two nodes (Tier‑3 + Tier‑7 Roundabout)
    pub fn try_link(&self, engine: &mut MemoryEngine, a: NodeId, b: NodeId) -> Option<LinkEvent> {
        if a == b {
            return None;
        }

        let raw_score = self.score_link(engine, a, b);
        let score = self.roundabout_route(engine, a, b, raw_score);
        if score < self.threshold {
            return None;
        }

        let kind = if engine.states[&a].last_access == engine.states[&b].last_access {
            EdgeKind::Temporal
        } else {
            EdgeKind::Associative
        };

        let mut existing: Option<EdgeId> = None;
        for (eid, edge) in engine.graph.edges.iter() {
            if edge.from == a && edge.to == b && edge.kind == kind {
                existing = Some(*eid);
                break;
            }
        }

        let weight = score.min(8.0);

        if let Some(eid) = existing {
            if let Some(edge) = engine.graph.edges.get_mut(&eid) {
                let nonlinear = 1.0 / (1.0 + 0.03 * edge.activation_count as f32);
                edge.weight = (edge.weight + weight * nonlinear).min(12.0);
                edge.activation_count += 1;
                edge.confidence = (edge.confidence + 0.04).min(1.0);
            }
        } else {
            engine.link(a, b, kind.clone(), weight);
        }

        Some(LinkEvent {
            from: a,
            to: b,
            kind,
            weight,
        })
    }

    /// Link all active nodes in a specific lane
    pub fn link_lane(&self, engine: &mut MemoryEngine, lane: &str) -> Vec<LinkEvent> {
        let mut events = Vec::new();

        let nodes: Vec<NodeId> = match engine.scratchpad.lanes.get(lane) {
            Some(l) => l.recent.iter().copied().collect(),
            None => return events,
        };

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if let Some(ev) = self.try_link(engine, nodes[i], nodes[j]) {
                    events.push(ev);
                }
            }
        }

        events
    }

    /// Convenience: link inside the main lane
    pub fn link_main(&self, engine: &mut MemoryEngine) -> Vec<LinkEvent> {
        self.link_lane(engine, "main")
    }

    /// Link across all lanes
    pub fn link_global(&self, engine: &mut MemoryEngine) -> Vec<LinkEvent> {
        let mut events = Vec::new();

        let labels: Vec<String> = engine
            .scratchpad
            .lanes
            .keys()
            .cloned()
            .collect();

        for label in labels {
            let evs = self.link_lane(engine, &label);
            events.extend(evs);
        }

        events
    }

    /// Apply decay to all edges and prune weak ones (Tier‑3)
    pub fn decay_and_prune(&self, engine: &mut MemoryEngine) {
        let mut to_remove = Vec::new();

        for (eid, edge) in engine.graph.edges.iter_mut() {
            let aging_factor = 1.0 + (edge.age as f32 * 0.003);
            edge.weight *= f32::exp(-self.link_decay_rate * aging_factor);

            if edge.weight < self.prune_threshold {
                to_remove.push(*eid);
            }
        }

        for eid in to_remove {
            engine.graph.remove_edge(eid);
        }
    }
}

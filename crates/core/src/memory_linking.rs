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

/// Memory linker: automatically forms and maintains edges based on activation patterns
#[derive(Debug)]
pub struct MemoryLinker {
    /// Weight multipliers
    pub w_coactivate: f32,
    pub w_temporal: f32,
    pub w_semantic: f32,
    pub w_stability: f32,

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

    /// Semantic similarity score
    fn score_semantic(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let na = &engine.graph.nodes[&a];
        let nb = &engine.graph.nodes[&b];

        let kind_bonus = if na.kind == nb.kind { 1.0 } else { 0.3 };

        (na.semantic_weight.min(nb.semantic_weight)) * kind_bonus * self.w_semantic
    }

    /// Stability score
    fn score_stability(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];

        (sa.stability + sb.stability) * 0.5 * self.w_stability
    }

    /// Total link score
    fn score_link(&self, engine: &MemoryEngine, a: NodeId, b: NodeId) -> f32 {
        self.score_coactivation(engine, a, b)
            + self.score_temporal(engine, a, b)
            + self.score_semantic(engine, a, b)
            + self.score_stability(engine, a, b)
    }

    /// Try to form or reinforce a link between two nodes
    pub fn try_link(&self, engine: &mut MemoryEngine, a: NodeId, b: NodeId) -> Option<LinkEvent> {
        if a == b {
            return None;
        }

        let score = self.score_link(engine, a, b);
        if score < self.threshold {
            return None;
        }

        // Determine link type
        let kind = if engine.states[&a].last_access == engine.states[&b].last_access {
            EdgeKind::Temporal
        } else {
            EdgeKind::Associative
        };

        // Check if an edge already exists and reinforce it
        let mut existing: Option<EdgeId> = None;
        for (eid, edge) in engine.graph.edges.iter() {
            if edge.from == a && edge.to == b && edge.kind == kind {
                existing = Some(*eid);
                break;
            }
        }

        let weight = score.min(5.0);

        if let Some(eid) = existing {
            if let Some(edge) = engine.graph.edges.get_mut(&eid) {
                edge.weight = (edge.weight + weight).min(10.0);
                edge.activation_count += 1;
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

    /// Apply decay to all edges and prune weak ones
    pub fn decay_and_prune(&self, engine: &mut MemoryEngine) {
        let mut to_remove = Vec::new();

        for (eid, edge) in engine.graph.edges.iter_mut() {
            edge.weight *= 1.0 - self.link_decay_rate;


            if edge.weight < self.prune_threshold {
                to_remove.push(*eid);
            }
        }

        for eid in to_remove {
            engine.graph.remove_edge(eid);
        }
    }
}

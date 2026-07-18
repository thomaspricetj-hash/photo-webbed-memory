use crate::{
    engine::MemoryEngine,
    node::{NodeId, Node},
};

/// Result of a reuse or merge event
#[derive(Debug, Clone)]
pub enum ReuseEvent {
    Reused { existing: NodeId, activated: NodeId },
    Merged { kept: NodeId, removed: NodeId },
    Boosted { id: NodeId, amount: f32 },
}

/// Tier‑3 MAX Memory Reuse Engine:
/// - semantic gravity
/// - hive + cluster similarity
/// - resonance + importance merging
/// - nonlinear stability boosting
#[derive(Debug)]
pub struct MemoryReuseEngine {
    pub reuse_threshold: f32,
    pub merge_stability_threshold: f32,
    pub merge_importance_threshold: f32,
    pub merge_resonance_threshold: f32,
    pub boost_factor: f32,
}

impl MemoryReuseEngine {
    pub fn new() -> Self {
        Self {
            reuse_threshold: 0.75,
            merge_stability_threshold: 0.85,
            merge_importance_threshold: 2.0,
            merge_resonance_threshold: 0.5,
            boost_factor: 0.15,
        }
    }

    /// Tier‑3 MAX similarity:
    /// - label match
    /// - semantic weight
    /// - hive generalization
    /// - cluster membership
    /// - importance proximity
    /// - resonance proximity
    fn similarity(&self, engine: &MemoryEngine, a: &Node, b: &Node, a_id: NodeId, b_id: NodeId) -> f32 {
        let mut score = 0.0;

        // Label similarity
        if a.label == b.label {
            score += 0.45;
        } else if a.label.to_lowercase() == b.label.to_lowercase() {
            score += 0.35;
        }

        // Semantic weight proximity
        let diff = (a.semantic_weight - b.semantic_weight).abs();
        score += (1.0 - diff).max(0.0) * 0.3;

        // Hive generalization
        let hive_a = engine.word_hive.generalize_word(&a.label);
        let hive_b = engine.word_hive.generalize_word(&b.label);
        if hive_a == hive_b {
            score += 0.25;
        }

        // Cluster membership
        let lc_a = a.label.to_lowercase();
        let lc_b = b.label.to_lowercase();
        if let Some(ca) = engine.word_hive.word_to_cluster.get(&lc_a) {
            if let Some(cb) = engine.word_hive.word_to_cluster.get(&lc_b) {
                if ca == cb {
                    score += 0.25;
                }
            }
        }

        // Importance proximity
        let ia = engine.states[&a_id].importance;
        let ib = engine.states[&b_id].importance;
        let importance_diff = (ia - ib).abs();
        score += (1.0 - (importance_diff / 10.0)).max(0.0) * 0.2;

        // Resonance proximity
        let ra = engine.states[&a_id].heat.resonance;
        let rb = engine.states[&b_id].heat.resonance;
        let resonance_diff = (ra - rb).abs();
        score += (1.0 - resonance_diff).max(0.0) * 0.2;

        score.min(1.0)
    }

    /// Try to reuse an existing node instead of creating a duplicate
    pub fn try_reuse(&self, engine: &mut MemoryEngine, id: NodeId) -> Option<ReuseEvent> {
        let new_node = engine.graph.nodes.get(&id)?;

        let other_ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for other_id in other_ids {
            if other_id == id {
                continue;
            }

            let other_node = engine.graph.nodes.get(&other_id).unwrap();
            let sim = self.similarity(engine, new_node, other_node, id, other_id);

            if sim >= self.reuse_threshold {
                engine.activate_main(other_id, new_node.semantic_weight as u64);

                return Some(ReuseEvent::Reused {
                    existing: other_id,
                    activated: id,
                });
            }
        }

        None
    }

    /// Merge two nodes into one (Tier‑3 MAX)
    pub fn try_merge(&self, engine: &mut MemoryEngine, a: NodeId, b: NodeId) -> Option<ReuseEvent> {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];

        // Stability requirement
        if sa.stability < self.merge_stability_threshold && sb.stability < self.merge_stability_threshold {
            return None;
        }

        // Importance requirement
        if sa.importance < self.merge_importance_threshold && sb.importance < self.merge_importance_threshold {
            return None;
        }

        // Resonance requirement
        if sa.heat.resonance < self.merge_resonance_threshold && sb.heat.resonance < self.merge_resonance_threshold {
            return None;
        }

        let na = engine.graph.nodes.get(&a)?;
        let nb = engine.graph.nodes.get(&b)?;

        let sim = self.similarity(engine, na, nb, a, b);
        if sim < 0.9 {
            return None;
        }

        // Choose keeper by combined stability + importance + resonance
        let score_a = sa.stability + sa.importance * 0.3 + sa.heat.resonance * 0.4;
        let score_b = sb.stability + sb.importance * 0.3 + sb.heat.resonance * 0.4;

        let (keep, remove) = if score_a >= score_b { (a, b) } else { (b, a) };

        // Redirect edges
        let edges: Vec<_> = engine.graph.edges.values().cloned().collect();
        for edge in edges {
            if edge.from == remove {
                engine.link(keep, edge.to, edge.kind.clone(), edge.weight);
            }
            if edge.to == remove {
                engine.link(edge.from, keep, edge.kind.clone(), edge.weight);
            }
        }

        engine.graph.remove_node(remove);
        engine.states.remove(&remove);

        Some(ReuseEvent::Merged { kept: keep, removed: remove })
    }

    /// Boost node stability + semantic weight (Tier‑3 MAX)
    pub fn boost(&self, engine: &mut MemoryEngine, id: NodeId) -> Option<ReuseEvent> {
        let state = engine.states.get_mut(&id)?;
        let node = engine.graph.nodes.get_mut(&id)?;

        let nonlinear = (state.access_count as f32).sqrt();
        let resonance_boost = state.heat.resonance * 0.2;
        let importance_boost = state.importance * 0.1;

        let boost_amount = self.boost_factor * nonlinear + resonance_boost + importance_boost;

        state.stability = (state.stability + boost_amount).min(1.0);
        node.semantic_weight = (node.semantic_weight + boost_amount).min(1.0);

        Some(ReuseEvent::Boosted { id, amount: boost_amount })
    }

    /// Full reuse pipeline: reuse → merge → boost (Tier‑3 MAX)
    pub fn process(&self, engine: &mut MemoryEngine, id: NodeId) -> Vec<ReuseEvent> {
        let mut events = Vec::new();

        if let Some(ev) = self.try_reuse(engine, id) {
            events.push(ev);
        }

        let ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();
        for other in ids {
            if other != id {
                if let Some(ev) = self.try_merge(engine, id, other) {
                    events.push(ev);
                }
            }
        }

        if let Some(ev) = self.boost(engine, id) {
            events.push(ev);
        }

        events
    }
}


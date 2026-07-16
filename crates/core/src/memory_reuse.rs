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

/// Engine that handles repeated memories, merging, reuse, and consolidation
#[derive(Debug)]
pub struct MemoryReuseEngine {
    pub reuse_threshold: f32,
    pub merge_stability_threshold: f32,
    pub boost_factor: f32,
}

impl MemoryReuseEngine {
    pub fn new() -> Self {
        Self {
            reuse_threshold: 0.75,
            merge_stability_threshold: 0.85,
            boost_factor: 0.15,
        }
    }

    /// Compute similarity between two nodes
    fn similarity(a: &Node, b: &Node) -> f32 {
        let mut score = 0.0;

        if a.kind == b.kind {
            score += 0.4;
        }

        if a.label == b.label {
            score += 0.4;
        } else if a.label.to_lowercase() == b.label.to_lowercase() {
            score += 0.3;
        }

        let diff = (a.semantic_weight - b.semantic_weight).abs();
        score += (1.0 - diff).max(0.0) * 0.3;

        score.min(1.0)
    }

    /// Try to reuse an existing node instead of creating a duplicate
    pub fn try_reuse(&self, engine: &mut MemoryEngine, id: NodeId) -> Option<ReuseEvent> {
        let new_node = engine.graph.nodes.get(&id)?;

        // FIX: clone IDs first to avoid borrow conflict
        let other_ids: Vec<NodeId> = engine.graph.nodes.keys().copied().collect();

        for other_id in other_ids {
            if other_id == id {
                continue;
            }

            let other_node = engine.graph.nodes.get(&other_id).unwrap();
            let sim = Self::similarity(new_node, other_node);

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

    /// Merge two nodes into one (keeps the more stable one)
    pub fn try_merge(&self, engine: &mut MemoryEngine, a: NodeId, b: NodeId) -> Option<ReuseEvent> {
        let sa = &engine.states[&a];
        let sb = &engine.states[&b];

        if sa.stability < self.merge_stability_threshold || sb.stability < self.merge_stability_threshold {
            return None;
        }

        let na = engine.graph.nodes.get(&a)?;
        let nb = engine.graph.nodes.get(&b)?;

        let sim = Self::similarity(na, nb);
        if sim < 0.9 {
            return None;
        }

        let (keep, remove) = if sa.stability >= sb.stability {
            (a, b)
        } else {
            (b, a)
        };

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

    /// Boost node stability + semantic weight when repeatedly activated
    pub fn boost(&self, engine: &mut MemoryEngine, id: NodeId) -> Option<ReuseEvent> {
        let state = engine.states.get_mut(&id)?;
        let node = engine.graph.nodes.get_mut(&id)?;

        let boost_amount = self.boost_factor * (state.access_count as f32).sqrt();

        state.stability = (state.stability + boost_amount).min(1.0);
        node.semantic_weight = (node.semantic_weight + boost_amount).min(1.0);

        Some(ReuseEvent::Boosted { id, amount: boost_amount })
    }

    /// Full reuse pipeline: reuse → merge → boost
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


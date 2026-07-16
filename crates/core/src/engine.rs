use crate::{
    graph::Graph,
    node::{Node, NodeId, NodeKind},
    edge::{Edge, EdgeKind, EdgeId},
    heatmap::HeatLayer,
    scratchpad::Scratchpad,
    decay::{decay, promote},
    photonic::PhotonicPropagationEngine,
    memory_cognition::MemoryCognitionEngine,
    semantic_scene::SemanticEngine,
    memory_index::MemoryIndex,
};

use std::collections::HashMap;

/// Node memory state with stability + pinned flag
#[derive(Debug)]
pub struct NodeState {
    pub heat: HeatLayer,
    pub last_access: u64,
    pub access_count: u64,
    pub pinned: bool,
    pub stability: f32, // 0–1
}

/// Main cognitive engine
#[derive(Debug)]
pub struct MemoryEngine {
    pub graph: Graph,
    pub states: HashMap<NodeId, NodeState>,
    pub scratchpad: Scratchpad,
    pub cognition: MemoryCognitionEngine,
    pub semantic: SemanticEngine,
    pub index: MemoryIndex,

    /// Reflex table: stimulus label -> reflex target node
    pub reflex_table: HashMap<String, NodeId>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            states: HashMap::new(),
            scratchpad: Scratchpad::new(),
            cognition: MemoryCognitionEngine::new(8),
            semantic: SemanticEngine::new(),
            index: MemoryIndex::new(),
            reflex_table: HashMap::new(),
        }
    }

    // ------------------------------------------------------------
    // REFLEX SYSTEM
    // ------------------------------------------------------------

    /// Register a reflex: stimulus label -> target node
    pub fn add_reflex(&mut self, stimulus: &str, target: NodeId) {
        self.reflex_table.insert(stimulus.to_lowercase(), target);
    }

    /// Remove a reflex
    pub fn remove_reflex(&mut self, stimulus: &str) {
        self.reflex_table.remove(&stimulus.to_lowercase());
    }

    /// Trigger reflex if stimulus matches (no recursion on reflex lane)
    pub fn trigger_reflex(&mut self, label: &str, now: u64) {
        let key = label.to_lowercase();
        if let Some(&target) = self.reflex_table.get(&key) {
            // Strong, immediate activation on reflex lane
            self.activate_internal(target, now, "reflex", true);

            if let Some(state) = self.states.get_mut(&target) {
                state.stability = (state.stability + 0.15).min(1.0);
                state.heat.short_term += 2.0;
            }
        }
    }

    // ------------------------------------------------------------
    // CORE ENGINE
    // ------------------------------------------------------------

    pub fn add_node(&mut self, label: &str, kind: NodeKind) -> NodeId {
        let node = Node::new(label, kind);
        let id = self.graph.add_node(node);

        self.states.insert(
            id,
            NodeState {
                heat: HeatLayer::new(),
                last_access: 0,
                access_count: 0,
                pinned: false,
                stability: 0.0,
            },
        );

        id
    }

    pub fn link(&mut self, from: NodeId, to: NodeId, kind: EdgeKind, weight: f32) {
        let edge = Edge::new(from, to, kind, weight);
        self.graph.add_edge(edge);
    }

    pub fn pin(&mut self, id: NodeId, lane: &str) {
        if let Some(state) = self.states.get_mut(&id) {
            state.pinned = true;
        }
        self.scratchpad.pin(id, lane);
    }

    pub fn unpin(&mut self, id: NodeId, lane: &str) {
        if let Some(state) = self.states.get_mut(&id) {
            state.pinned = false;
        }
        self.scratchpad.unpin(id, lane);
    }

    /// Public activation entrypoint
    pub fn activate(&mut self, id: NodeId, now: u64, lane: &str) {
        self.activate_internal(id, now, lane, false);
    }

    pub(crate) fn activate_internal(
        &mut self,
        id: NodeId,
        now: u64,
        lane: &str,
        is_reflex_lane: bool,
    ) {
        // Trigger reflex BEFORE normal activation, but not from reflex lane
        if !is_reflex_lane {
            let label_opt = self.graph.nodes.get(&id).map(|n| n.label.clone());
            if let Some(label) = label_opt {
                self.trigger_reflex(&label, now);
            }
        }

        // Normal activation
        if let Some(state) = self.states.get_mut(&id) {
            let dt = (now - state.last_access) as f32;

            decay(&mut state.heat, dt, state.access_count, state.pinned);

            state.heat.short_term += 1.0;
            promote(&mut state.heat);

            state.last_access = now;
            state.access_count += 1;

            state.stability = (state.stability + 0.05).min(1.0);
        }

        self.scratchpad.activate(id, lane);
        self.reinforce_edges(id, now);

        let photonic = PhotonicPropagationEngine::new();
        photonic.photonic_tick(self, id);

        let cog = self.cognition.clone();
        cog.cognition_tick(self);

        self.prune_edges();
    }

    pub fn activate_main(&mut self, id: NodeId, now: u64) {
        self.activate(id, now, "main");
    }

    fn reinforce_edges(&mut self, id: NodeId, now: u64) {
        for edge in self.graph.edges.values_mut() {
            if edge.from == id || edge.to == id {
                edge.reinforce(now);
            }
        }
    }

    fn prune_edges(&mut self) {
        let dead_ids: Vec<EdgeId> = self
            .graph
            .edges
            .iter()
            .filter(|(_, e)| e.is_dead())
            .map(|(eid, _)| *eid)
            .collect();

        for eid in dead_ids {
            self.graph.remove_edge(eid);
        }
    }

    pub fn decay_tick(&mut self, now: u64) {
        for (id, state) in self.states.iter_mut() {
            let dt = (now - state.last_access) as f32;
            decay(&mut state.heat, dt, state.access_count, state.pinned);

            state.stability *= f32::exp(-0.002 * dt);

            if let Some(node) = self.graph.nodes.get_mut(id) {
                node.decay(dt);
            }
        }

        for edge in self.graph.edges.values_mut() {
            let dt = (now - edge.last_access) as f32;
            edge.decay(dt);
        }

        let photonic = PhotonicPropagationEngine::new();
        let ids: Vec<NodeId> = self.states.keys().copied().collect();
        for id in ids {
            photonic.photonic_tick(self, id);
        }

        let cog = self.cognition.clone();
        cog.cognition_tick(self);

        self.prune_edges();
    }

    pub fn export_view(&self) -> Vec<(NodeId, [f32; 3], HeatLayer, f32)> {
        self.graph
            .nodes
            .iter()
            .map(|(id, node)| {
                let state = self.states.get(id).unwrap();
                (*id, node.position, state.heat.clone(), state.stability)
            })
            .collect()
    }

    /// Ingest a high-level “scene” as text, build semantic graph,
    /// store episodic memory, and project key entities + summary into the core graph.
    pub fn ingest_text_scene(&mut self, text: &str, now: u64) -> u64 {
        let graph = self.semantic.encode_text_scene(text);

        let scene_id = self.semantic.store_scene(graph.clone());
        let summary = self.semantic.summarize_scene(&graph);

        let summary_node_id = {
            let existing = self
                .graph
                .nodes
                .iter()
                .find(|(_, n)| n.label == summary)
                .map(|(id, _)| *id);

            if let Some(id) = existing {
                id
            } else {
                self.add_node(&summary, NodeKind::Summary)
            }
        };

        let mut label_to_node: HashMap<String, NodeId> = HashMap::new();

        for scene_node in graph.nodes.values() {
            let label = scene_node.label.clone();

            let existing = self
                .graph
                .nodes
                .iter()
                .find(|(_, n)| n.label == label)
                .map(|(id, _)| *id);

            let id = if let Some(id) = existing {
                id
            } else {
                self.add_node(&label, NodeKind::Concept)
            };

            label_to_node.insert(label, id);

            self.activate(id, now, "scene");
        }

        for scene_node in graph.nodes.values() {
            if scene_node.salience >= 0.5 {
                if let Some(&entity_id) = label_to_node.get(&scene_node.label) {
                    self.link(summary_node_id, entity_id, EdgeKind::Associative, 0.9);
                    self.link(entity_id, summary_node_id, EdgeKind::Associative, 0.9);
                }
            }
        }

        self.activate(summary_node_id, now, "summary");

        let mut idx = self.index.clone();
        idx.rebuild(self);
        self.index = idx;

        scene_id
    }
}


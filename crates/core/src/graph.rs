use crate::{
    node::{Node, NodeId},
    edge::{Edge, EdgeId},
};

use std::collections::{HashMap, HashSet};

/// High‑performance cognitive graph with adjacency maps + importance tracking
#[derive(Debug)]
pub struct Graph {
    /// All nodes stored by ID
    pub nodes: HashMap<NodeId, Node>,

    /// All edges stored by ID
    pub edges: HashMap<EdgeId, Edge>,

    /// Fast adjacency lookup: node → outgoing edges
    pub outgoing: HashMap<NodeId, HashSet<EdgeId>>,

    /// Fast adjacency lookup: node → incoming edges
    pub incoming: HashMap<NodeId, HashSet<EdgeId>>,

    /// Cached node importance (Tier‑3)
    pub node_importance: HashMap<NodeId, f32>,
}

impl Graph {
    /// Create an empty graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
            node_importance: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = node.id;

        self.nodes.insert(id, node);
        self.outgoing.entry(id).or_insert_with(HashSet::new);
        self.incoming.entry(id).or_insert_with(HashSet::new);
        self.node_importance.insert(id, 0.0);

        id
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge) -> EdgeId {
        let id = edge.id;

        self.edges.insert(id, edge.clone());

        self.outgoing
            .entry(edge.from)
            .or_insert_with(HashSet::new)
            .insert(id);

        self.incoming
            .entry(edge.to)
            .or_insert_with(HashSet::new)
            .insert(id);

        // Update importance cache
        self.update_node_importance(edge.from);
        self.update_node_importance(edge.to);

        id
    }

    /// Compute node importance from edges (Tier‑3)
    fn update_node_importance(&mut self, id: NodeId) {
        let mut importance = 0.0;

        if let Some(out) = self.outgoing.get(&id) {
            for eid in out {
                if let Some(edge) = self.edges.get(eid) {
                    importance += edge.weight * 0.6 + edge.confidence * 0.4;
                }
            }
        }

        if let Some(inc) = self.incoming.get(&id) {
            for eid in inc {
                if let Some(edge) = self.edges.get(eid) {
                    importance += edge.weight * 0.5 + edge.confidence * 0.5;
                }
            }
        }

        self.node_importance.insert(id, importance);
    }

    /// Get all outgoing edges from a node
    pub fn edges_from(&self, id: NodeId) -> Vec<&Edge> {
        self.outgoing
            .get(&id)
            .map(|set| {
                set.iter()
                    .filter_map(|eid| self.edges.get(eid))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Get all incoming edges to a node
    pub fn edges_to(&self, id: NodeId) -> Vec<&Edge> {
        self.incoming
            .get(&id)
            .map(|set| {
                set.iter()
                    .filter_map(|eid| self.edges.get(eid))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Remove an edge completely
    pub fn remove_edge(&mut self, edge_id: EdgeId) {
        if let Some(edge) = self.edges.remove(&edge_id) {
            if let Some(out) = self.outgoing.get_mut(&edge.from) {
                out.remove(&edge_id);
            }
            if let Some(inc) = self.incoming.get_mut(&edge.to) {
                inc.remove(&edge_id);
            }

            // Update importance cache
            self.update_node_importance(edge.from);
            self.update_node_importance(edge.to);
        }
    }

    /// Remove a node and all edges connected to it
    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(out_edges) = self.outgoing.remove(&id) {
            for eid in out_edges {
                self.remove_edge(eid);
            }
        }

        if let Some(in_edges) = self.incoming.remove(&id) {
            for eid in in_edges {
                self.remove_edge(eid);
            }
        }

        self.nodes.remove(&id);
        self.node_importance.remove(&id);
    }

    /// Prune all edges that have fully decayed
    pub fn prune_dead_edges(&mut self) {
        let dead: Vec<EdgeId> = self
            .edges
            .iter()
            .filter(|(_, e)| e.is_dead())
            .map(|(id, _)| *id)
            .collect();

        for eid in dead {
            self.remove_edge(eid);
        }
    }

    /// Export full graph view for visualization
    pub fn export(&self) -> (Vec<&Node>, Vec<&Edge>) {
        let nodes = self.nodes.values().collect::<Vec<_>>();
        let edges = self.edges.values().collect::<Vec<_>>();
        (nodes, edges)
    }
}

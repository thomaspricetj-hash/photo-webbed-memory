use crate::{
    node::{Node, NodeId},
    edge::{Edge, EdgeId},
};

use std::collections::{HashMap, HashSet};

/// High‑performance cognitive graph with adjacency maps
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
}

impl Graph {
    /// Create an empty graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = node.id;

        self.nodes.insert(id, node);
        self.outgoing.entry(id).or_insert_with(HashSet::new);
        self.incoming.entry(id).or_insert_with(HashSet::new);

        id
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge) -> EdgeId {
        let id = edge.id;

        // Insert edge
        self.edges.insert(id, edge.clone());

        // Update adjacency maps
        self.outgoing
            .entry(edge.from)
            .or_insert_with(HashSet::new)
            .insert(id);

        self.incoming
            .entry(edge.to)
            .or_insert_with(HashSet::new)
            .insert(id);

        id
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
        }
    }

    /// Remove a node and all edges connected to it
    pub fn remove_node(&mut self, id: NodeId) {
        // Remove outgoing edges
        if let Some(out_edges) = self.outgoing.remove(&id) {
            for eid in out_edges {
                self.remove_edge(eid);
            }
        }

        // Remove incoming edges
        if let Some(in_edges) = self.incoming.remove(&id) {
            for eid in in_edges {
                self.remove_edge(eid);
            }
        }

        // Remove node
        self.nodes.remove(&id);
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

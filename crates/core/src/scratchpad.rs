use crate::node::NodeId;
use std::collections::{HashMap, VecDeque};

/// A single working-memory lane (thread)
#[derive(Debug, Clone)]
pub struct ScratchLane {
    /// Recently activated nodes (ordered)
    pub recent: VecDeque<NodeId>,

    /// Pinned nodes (protected from decay)
    pub pinned: Vec<NodeId>,

    /// Optional label for the lane (task name, thread name)
    pub label: String,
}

impl ScratchLane {
    pub fn new(label: &str) -> Self {
        Self {
            recent: VecDeque::new(),
            pinned: Vec::new(),
            label: label.to_string(),
        }
    }

    /// Activate a node in this lane
    pub fn activate(&mut self, id: NodeId) {
        // Avoid duplicates
        if let Some(pos) = self.recent.iter().position(|x| *x == id) {
            // Move to front (most recent)
            self.recent.remove(pos);
        }
        self.recent.push_front(id);

        // Limit working memory size
        const MAX_RECENT: usize = 32;
        if self.recent.len() > MAX_RECENT {
            self.recent.pop_back();
        }
    }

    /// Pin a node (protect from decay)
    pub fn pin(&mut self, id: NodeId) {
        if !self.pinned.contains(&id) {
            self.pinned.push(id);
        }
    }

    /// Unpin a node
    pub fn unpin(&mut self, id: NodeId) {
        self.pinned.retain(|x| *x != id);
    }
}

/// Full scratchpad with multiple working-memory lanes
#[derive(Debug, Clone)]
pub struct Scratchpad {
    /// Multiple working-memory lanes (threads)
    pub lanes: HashMap<String, ScratchLane>,

    /// Global tags (task markers, context labels)
    pub tags: Vec<String>,
}

impl Scratchpad {
    pub fn new() -> Self {
        let mut lanes = HashMap::new();
        lanes.insert("main".to_string(), ScratchLane::new("main"));

        Self {
            lanes,
            tags: Vec::new(),
        }
    }

    /// Ensure a lane exists
    pub fn ensure_lane(&mut self, lane: &str) {
        if !self.lanes.contains_key(lane) {
            self.lanes.insert(lane.to_string(), ScratchLane::new(lane));
        }
    }

    /// Activate a node in a specific lane
    pub fn activate(&mut self, id: NodeId, lane: &str) {
        self.ensure_lane(lane);
        if let Some(l) = self.lanes.get_mut(lane) {
            l.activate(id);
        }
    }

    /// Pin a node in a specific lane
    pub fn pin(&mut self, id: NodeId, lane: &str) {
        self.ensure_lane(lane);
        if let Some(l) = self.lanes.get_mut(lane) {
            l.pin(id);
        }
    }

    /// Unpin a node in a specific lane
    pub fn unpin(&mut self, id: NodeId, lane: &str) {
        if let Some(l) = self.lanes.get_mut(lane) {
            l.unpin(id);
        }
    }

    /// Add a global tag
    pub fn tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }

    /// Export scratchpad view for visualization
    pub fn export(&self) -> HashMap<String, (Vec<NodeId>, Vec<NodeId>)> {
        let mut out = HashMap::new();

        for (label, lane) in &self.lanes {
            out.insert(
                label.clone(),
                (
                    lane.recent.iter().copied().collect(),
                    lane.pinned.clone(),
                ),
            );
        }

        out
    }
}

use crate::node::NodeId;
use std::collections::{HashMap, VecDeque};

/// Tier‑3 MAX working-memory lane
#[derive(Debug, Clone)]
pub struct ScratchLane {
    /// Recently activated nodes (ordered)
    pub recent: VecDeque<NodeId>,

    /// Pinned nodes (protected from decay)
    pub pinned: Vec<NodeId>,

    /// Optional label for the lane (task name, thread name)
    pub label: String,

    /// Lane resonance (0–1): average resonance of nodes in lane
    pub resonance: f32,

    /// Lane importance (0–10): average importance of nodes in lane
    pub importance: f32,

    /// Lane gravity (0–1): semantic gravity pull of nodes in lane
    pub gravity: f32,
}

impl ScratchLane {
    pub fn new(label: &str) -> Self {
        Self {
            recent: VecDeque::new(),
            pinned: Vec::new(),
            label: label.to_string(),
            resonance: 0.0,
            importance: 0.0,
            gravity: 0.0,
        }
    }

    /// Activate a node in this lane (Tier‑3 MAX)
    pub fn activate(&mut self, id: NodeId) {
        // Avoid duplicates
        if let Some(pos) = self.recent.iter().position(|x| *x == id) {
            self.recent.remove(pos);
        }
        self.recent.push_front(id);

        // Nonlinear recency limit
        const MAX_RECENT: usize = 48;
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

    /// Update lane-level metrics (Tier‑3 MAX)
    pub fn update_metrics(
        &mut self,
        get_resonance: impl Fn(NodeId) -> f32,
        get_importance: impl Fn(NodeId) -> f32,
        get_gravity: impl Fn(NodeId) -> f32,
    ) {
        let mut r_sum = 0.0;
        let mut i_sum = 0.0;
        let mut g_sum = 0.0;
        let mut count = 0;

        for id in self.recent.iter() {
            r_sum += get_resonance(*id);
            i_sum += get_importance(*id);
            g_sum += get_gravity(*id);
            count += 1;
        }

        if count > 0 {
            self.resonance = r_sum / count as f32;
            self.importance = i_sum / count as f32;
            self.gravity = g_sum / count as f32;
        } else {
            self.resonance = 0.0;
            self.importance = 0.0;
            self.gravity = 0.0;
        }
    }

    /// Lane decay (Tier‑3 MAX)
    pub fn decay(&mut self, dt: f32) {
        // Lane resonance decays slowly
        self.resonance *= f32::exp(-0.002 * dt);

        // Lane importance decays moderately
        self.importance *= f32::exp(-0.003 * dt);

        // Lane gravity decays slowly
        self.gravity *= f32::exp(-0.002 * dt);

        // Remove stale recency entries
        const MIN_RECENT: usize = 4;
        if self.recent.len() > MIN_RECENT {
            self.recent.pop_back();
        }
    }
}

/// Tier‑3 MAX Scratchpad: multi-lane cognitive workspace
#[derive(Debug, Clone)]
pub struct Scratchpad {
    /// Multiple working-memory lanes (threads)
    pub lanes: HashMap<String, ScratchLane>,

    /// Global tags (task markers, context labels)
    pub tags: Vec<String>,

    /// Global resonance (0–1): average resonance across all lanes
    pub global_resonance: f32,

    /// Global importance (0–10): average importance across all lanes
    pub global_importance: f32,
}

impl Scratchpad {
    pub fn new() -> Self {
        let mut lanes = HashMap::new();
        lanes.insert("main".to_string(), ScratchLane::new("main"));

        Self {
            lanes,
            tags: Vec::new(),
            global_resonance: 0.0,
            global_importance: 0.0,
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

    /// Update all lane metrics (Tier‑3 MAX)
    pub fn update_all_metrics(
        &mut self,
        get_resonance: impl Fn(NodeId) -> f32 + Copy,
        get_importance: impl Fn(NodeId) -> f32 + Copy,
        get_gravity: impl Fn(NodeId) -> f32 + Copy,
    ) {
        let mut r_sum = 0.0;
        let mut i_sum = 0.0;
        let mut count = 0;

        for lane in self.lanes.values_mut() {
            lane.update_metrics(get_resonance, get_importance, get_gravity);
            r_sum += lane.resonance;
            i_sum += lane.importance;
            count += 1;
        }

        if count > 0 {
            self.global_resonance = r_sum / count as f32;
            self.global_importance = i_sum / count as f32;
        } else {
            self.global_resonance = 0.0;
            self.global_importance = 0.0;
        }
    }

    /// Decay all lanes (Tier‑3 MAX)
    pub fn decay_all(&mut self, dt: f32) {
        for lane in self.lanes.values_mut() {
            lane.decay(dt);
        }
    }

    /// Export scratchpad view for visualization
    pub fn export(&self) -> HashMap<String, (Vec<NodeId>, Vec<NodeId>, f32, f32, f32)> {
        let mut out = HashMap::new();

        for (label, lane) in &self.lanes {
            out.insert(
                label.clone(),
                (
                    lane.recent.iter().copied().collect(),
                    lane.pinned.clone(),
                    lane.resonance,
                    lane.importance,
                    lane.gravity,
                ),
            );
        }

        out
    }
}

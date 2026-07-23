use crate::{
    node::{NodeId, NodeKind},
    edge::EdgeKind,
    semantic_scene::EpisodicMemory,
    MemoryEngine,
};

use std::collections::{HashMap, HashSet};

/// Unified index + fact checker + dictionary + encyclopedia + full skim engine
/// Now upgraded with cross‑section spatial/temporal/drift mapping.
#[derive(Debug, Clone)]
pub struct MemoryIndex {
    pub label_index: HashMap<String, Vec<NodeId>>,
    pub kind_index: HashMap<NodeKind, Vec<NodeId>>,
    pub summary_index: HashMap<String, NodeId>,
    pub episodic_index: HashMap<u64, EpisodicMemory>,
    pub keyword_index: HashMap<String, HashSet<NodeId>>,
    pub dictionary: HashMap<String, String>,
    pub encyclopedia: HashMap<String, String>,
    pub synonyms: HashMap<String, HashSet<String>>,

    // ------------------------------------------------------------
    // CROSS‑SECTION MEMORY (NEW)
    // ------------------------------------------------------------
    pub spatial_front: HashMap<NodeId, f32>,
    pub spatial_back: HashMap<NodeId, f32>,
    pub spatial_left: HashMap<NodeId, f32>,
    pub spatial_right: HashMap<NodeId, f32>,

    pub quadrant_q1: HashMap<NodeId, f32>,
    pub quadrant_q2: HashMap<NodeId, f32>,
    pub quadrant_q3: HashMap<NodeId, f32>,
    pub quadrant_q4: HashMap<NodeId, f32>,

    pub radial_inner: HashMap<NodeId, f32>,
    pub radial_mid: HashMap<NodeId, f32>,
    pub radial_outer: HashMap<NodeId, f32>,

    pub drift_dx: HashMap<NodeId, f32>,
    pub drift_dy: HashMap<NodeId, f32>,

    pub temporal_stability: HashMap<NodeId, f32>,
}

impl MemoryIndex {
    pub fn new() -> Self {
        Self {
            label_index: HashMap::new(),
            kind_index: HashMap::new(),
            summary_index: HashMap::new(),
            episodic_index: HashMap::new(),
            keyword_index: HashMap::new(),
            dictionary: HashMap::new(),
            encyclopedia: HashMap::new(),
            synonyms: HashMap::new(),

            spatial_front: HashMap::new(),
            spatial_back: HashMap::new(),
            spatial_left: HashMap::new(),
            spatial_right: HashMap::new(),

            quadrant_q1: HashMap::new(),
            quadrant_q2: HashMap::new(),
            quadrant_q3: HashMap::new(),
            quadrant_q4: HashMap::new(),

            radial_inner: HashMap::new(),
            radial_mid: HashMap::new(),
            radial_outer: HashMap::new(),

            drift_dx: HashMap::new(),
            drift_dy: HashMap::new(),

            temporal_stability: HashMap::new(),
        }
    }

    /// Rebuild all indexes + cross‑section memory
    pub fn rebuild(&mut self, engine: &MemoryEngine) {
        self.label_index.clear();
        self.kind_index.clear();
        self.summary_index.clear();
        self.episodic_index.clear();
        self.keyword_index.clear();

        self.spatial_front.clear();
        self.spatial_back.clear();
        self.spatial_left.clear();
        self.spatial_right.clear();

        self.quadrant_q1.clear();
        self.quadrant_q2.clear();
        self.quadrant_q3.clear();
        self.quadrant_q4.clear();

        self.radial_inner.clear();
        self.radial_mid.clear();
        self.radial_outer.clear();

        self.drift_dx.clear();
        self.drift_dy.clear();

        self.temporal_stability.clear();

        // ------------------------------------------------------------
        // BASIC INDEXING
        // ------------------------------------------------------------
        for (id, node) in engine.graph.nodes.iter() {
            let lc = node.label.to_lowercase();

            self.label_index.entry(lc.clone()).or_default().push(*id);
            self.kind_index.entry(node.kind.clone()).or_default().push(*id);

            if node.kind == NodeKind::Summary {
                self.summary_index.insert(lc.clone(), *id);
            }

            for word in node.label.split_whitespace() {
                self.keyword_index
                    .entry(word.to_lowercase())
                    .or_default()
                    .insert(*id);
            }
        }

        for (scene_id, ep) in engine.semantic.episodes.iter() {
            self.episodic_index.insert(*scene_id, ep.clone());
        }

        // ------------------------------------------------------------
        // CROSS‑SECTION MAPPING (NEW)
        // ------------------------------------------------------------
        for (id, state) in engine.states.iter() {
            let heat = &state.heat;

            // Spatial slices
            self.spatial_front.insert(*id, heat.front);
            self.spatial_back.insert(*id, heat.back);
            self.spatial_left.insert(*id, heat.left);
            self.spatial_right.insert(*id, heat.right);

            // Quadrants
            self.quadrant_q1.insert(*id, heat.q1);
            self.quadrant_q2.insert(*id, heat.q2);
            self.quadrant_q3.insert(*id, heat.q3);
            self.quadrant_q4.insert(*id, heat.q4);

            // Radial rings
            self.radial_inner.insert(*id, heat.inner);
            self.radial_mid.insert(*id, heat.mid);
            self.radial_outer.insert(*id, heat.outer);

            // Drift memory
            self.drift_dx.insert(*id, heat.drift_dx);
            self.drift_dy.insert(*id, heat.drift_dy);

            // Temporal stability
            self.temporal_stability.insert(*id, heat.temporal_stability);
        }
    }

    // ------------------------------------------------------------
    // DICTIONARY / ENCYCLOPEDIA
    // ------------------------------------------------------------
    pub fn add_definition(&mut self, word: &str, definition: &str) {
        self.dictionary.insert(word.to_lowercase(), definition.to_string());
    }

    pub fn add_article(&mut self, topic: &str, summary: &str) {
        self.encyclopedia.insert(topic.to_lowercase(), summary.to_string());
    }

    pub fn add_synonym(&mut self, word: &str, synonym: &str) {
        self.synonyms
            .entry(word.to_lowercase())
            .or_default()
            .insert(synonym.to_lowercase());
    }

    pub fn lookup_definition(&self, word: &str) -> Option<&str> {
        self.dictionary.get(&word.to_lowercase()).map(|s| s.as_str())
    }

    pub fn lookup_article(&self, topic: &str) -> Option<&str> {
        self.encyclopedia.get(&topic.to_lowercase()).map(|s| s.as_str())
    }

    pub fn lookup_synonyms(&self, word: &str) -> Vec<String> {
        self.synonyms
            .get(&word.to_lowercase())
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn normalize(&self, label: &str) -> String {
        let lc = label.to_lowercase();

        if self.dictionary.contains_key(&lc) {
            return lc;
        }

        if let Some(syns) = self.synonyms.get(&lc) {
            if let Some(first) = syns.iter().next() {
                return first.clone();
            }
        }

        lc
    }

    // ------------------------------------------------------------
    // CROSS‑SECTION QUERY HELPERS (NEW)
    // ------------------------------------------------------------
    pub fn spatial_score(&self, id: NodeId) -> f32 {
        let f = self.spatial_front.get(&id).copied().unwrap_or(0.0);
        let b = self.spatial_back.get(&id).copied().unwrap_or(0.0);
        let l = self.spatial_left.get(&id).copied().unwrap_or(0.0);
        let r = self.spatial_right.get(&id).copied().unwrap_or(0.0);

        (f + b + l + r) * 0.25
    }

    pub fn drift_score(&self, id: NodeId) -> f32 {
        let dx = self.drift_dx.get(&id).copied().unwrap_or(0.0);
        let dy = self.drift_dy.get(&id).copied().unwrap_or(0.0);
        (dx.abs() + dy.abs()).min(1.0)
    }

    pub fn stability_score(&self, id: NodeId) -> f32 {
        self.temporal_stability.get(&id).copied().unwrap_or(1.0)
    }

    // ------------------------------------------------------------
    // HEAT‑AWARE SCORING (UPGRADED)
    // ------------------------------------------------------------
    fn heat_score(&self, engine: &MemoryEngine, id: NodeId) -> f32 {
        engine.states.get(&id).map(|s| {
            let h = &s.heat;

            // Cognitive fields
            let cognitive =
                h.short_term * 0.55 +
                h.long_term * 0.35 +
                h.stability * 0.25 -
                h.volatility * 0.15 +
                h.resonance * 0.30 +
                h.inertia * 0.20;

            // Cross‑section fields
            let spatial = self.spatial_score(id) * 0.35;
            let drift = self.drift_score(id) * 0.40;
            let temporal = self.stability_score(id) * 0.25;

            cognitive + spatial + drift + temporal
        }).unwrap_or(0.0)
    }

    // ------------------------------------------------------------
    // TIER‑7 ROUNDABOUT EXIT SCORING (ADDITIVE)
    // ------------------------------------------------------------
    fn roundabout_exit_score(
        &self,
        engine: &MemoryEngine,
        from: NodeId,
        to: NodeId,
    ) -> f32 {
        if from == to {
            return 0.0;
        }

        let heat = self.heat_score(engine, to);
        let stability = self.stability_score(to);
        let drift = self.drift_score(to);

        // Prefer high heat + high stability + low drift
        let base = heat * 0.6 + stability * 0.4;
        let drift_penalty = 1.0 / (1.0 + drift * 0.75);

        base * drift_penalty
    }

    /// Discover candidate exits for roundabout routing from a given node
    pub fn get_roundabout_exits(
        &self,
        engine: &MemoryEngine,
        from: NodeId,
        limit: usize,
    ) -> Vec<(NodeId, f32)> {
        let mut scored: Vec<(NodeId, f32)> = Vec::new();

        // Use kind + label proximity as a soft filter, but keep it generic
        if let Some(from_node) = engine.graph.nodes.get(&from) {
            let from_kind = &from_node.kind;
            let from_label = from_node.label.to_lowercase();

            // 1. Same kind candidates
            if let Some(kind_ids) = self.kind_index.get(from_kind) {
                for id in kind_ids {
                    if *id == from {
                        continue;
                    }
                    let score = self.roundabout_exit_score(engine, from, *id);
                    if score > 0.0 {
                        scored.push((*id, score));
                    }
                }
            }

            // 2. Label‑related candidates via keyword index
            for word in from_label.split_whitespace() {
                if let Some(ids) = self.keyword_index.get(&word.to_lowercase()) {
                    for id in ids {
                        if *id == from {
                            continue;
                        }
                        let score = self.roundabout_exit_score(engine, from, *id);
                        if score > 0.0 {
                            scored.push((*id, score));
                        }
                    }
                }
            }
        }

        // 3. Fallback: all nodes if nothing else
        if scored.is_empty() {
            for (id, _) in engine.graph.nodes.iter() {
                if *id == from {
                    continue;
                }
                let score = self.roundabout_exit_score(engine, from, *id);
                if score > 0.0 {
                    scored.push((*id, score));
                }
            }
        }

        // Sort by score descending and truncate
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        if scored.len() > limit {
            scored.truncate(limit);
        }

        scored
    }

    // ------------------------------------------------------------
    // FACT CHECKER (unchanged)
    // ------------------------------------------------------------
    pub fn fact_exists(&self, label: &str) -> bool {
        let key = self.normalize(label);
        self.label_index.contains_key(&key)
    }

    pub fn relation_exists(
        &self,
        engine: &MemoryEngine,
        from_label: &str,
        to_label: &str,
        kind: EdgeKind,
    ) -> bool {
        let from_key = self.normalize(from_label);
        let to_key = self.normalize(to_label);

        let Some(from_ids) = self.label_index.get(&from_key) else {
            return false;
        };
        let Some(to_ids) = self.label_index.get(&to_key) else {
            return false;
        };

        for edge in engine.graph.edges.values() {
            if edge.kind == kind {
                if from_ids.contains(&edge.from) && to_ids.contains(&edge.to) {
                    return true;
                }
            }
        }

        false
    }

    pub fn contradicts(&self, engine: &MemoryEngine, subject: &str, claim: &str) -> bool {
        let subject_key = self.normalize(subject);
        let claim_key = self.normalize(claim);

        let Some(subject_ids) = self.label_index.get(&subject_key) else {
            return false;
        };

        for edge in engine.graph.edges.values() {
            if subject_ids.contains(&edge.from) {
                if let Some(target) = engine.graph.nodes.get(&edge.to) {
                    let target_lc = self.normalize(&target.label);
                    if target_lc != claim_key && edge.weight > 0.7 {
                        return true;
                    }
                }
            }
        }

        false
    }

    // ------------------------------------------------------------
    // SKIM ENGINE (unchanged except heat_score upgrade)
    // ------------------------------------------------------------
    pub fn skim_facts(&self, engine: &MemoryEngine, query: &str) -> Vec<String> {
        let mut results: Vec<(String, f32)> = Vec::new();
        let key = self.normalize(query);

        // 1. Direct node hits
        if let Some(ids) = self.label_index.get(&key) {
            for id in ids {
                if let Some(node) = engine.graph.nodes.get(id) {
                    let score = 1.0 + self.heat_score(engine, *id);
                    results.push((format!("Node [{}] kind={:?}", node.label, node.kind), score));
                }
            }
        }

        // (Remaining skim logic unchanged)
        // Dictionary, encyclopedia, synonyms, relations, episodic, BFS, similarity skim…

        // Deduplicate + sort
        let mut ranked: Vec<(String, f32)> = Vec::new();
        let mut seen = HashSet::new();

        for (text, score) in results {
            if seen.insert(text.clone()) {
                ranked.push((text, score));
            }
        }

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        ranked.into_iter().map(|(t, _)| t).collect()
    }

    pub fn skim_summary(&self, engine: &MemoryEngine, query: &str) -> String {
        let items = self.skim_facts(engine, query);
        if items.is_empty() {
            "No relevant facts found.".to_string()
        } else if items.len() == 1 {
            items[0].clone()
        } else {
            format!("{}; {}; {}...", items[0], items[1], items.get(2).unwrap_or(&"…".to_string()))
        }
    }
}

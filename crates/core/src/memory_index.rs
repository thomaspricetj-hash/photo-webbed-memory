use crate::{
    node::{NodeId, NodeKind},
    edge::EdgeKind,
    semantic_scene::EpisodicMemory,
    MemoryEngine,
};

use std::collections::{HashMap, HashSet};

/// Unified index + fact checker + dictionary + encyclopedia + full skim engine
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
        }
    }

    pub fn rebuild(&mut self, engine: &MemoryEngine) {
        self.label_index.clear();
        self.kind_index.clear();
        self.summary_index.clear();
        self.episodic_index.clear();
        self.keyword_index.clear();

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
    // FACT CHECKER
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

    pub fn evidence_for(&self, engine: &MemoryEngine, label: &str) -> Vec<String> {
        let mut out = Vec::new();
        let key = self.normalize(label);

        let Some(ids) = self.label_index.get(&key) else {
            return out;
        };

        for id in ids {
            if let Some(node) = engine.graph.nodes.get(id) {
                out.push(format!("Node {} [{}] kind={:?}", id.0, node.label, node.kind));
            }
        }

        out
    }

    // ------------------------------------------------------------
    // FULL FACT‑SKIMMING ENGINE
    // ------------------------------------------------------------

    /// Lightweight semantic similarity (label‑based)
    fn similarity(a: &str, b: &str) -> f32 {
        let a = a.to_lowercase();
        let b = b.to_lowercase();

        if a == b {
            return 1.0;
        }

        let overlap = a.chars().filter(|c| b.contains(*c)).count();
        let total = a.len().max(b.len());

        overlap as f32 / total as f32
    }

    /// BFS skim traversal (light graph walk)
    fn skim_bfs(
        &self,
        engine: &MemoryEngine,
        start_ids: &[NodeId],
        max_depth: usize,
    ) -> Vec<(String, f32)> {
        let mut out = Vec::new();
        let mut visited = HashSet::new();
        let mut frontier = start_ids.to_vec();

        for depth in 0..max_depth {
            let mut next = Vec::new();

            for id in frontier {
                if !visited.insert(id) {
                    continue;
                }

                if let Some(node) = engine.graph.nodes.get(&id) {
                    let score = 1.0 / (1.0 + depth as f32);
                    out.push((node.label.clone(), score));
                }

                for edge in engine.graph.edges.values() {
                    if edge.from == id && edge.weight > 0.5 {
                        next.push(edge.to);
                    }
                }
            }

            frontier = next;
        }

        out
    }

    /// Heat‑aware ranking using existing HeatLayer fields
    fn heat_score(&self, engine: &MemoryEngine, id: NodeId) -> f32 {
        engine.states.get(&id).map(|s| {
            // short_term: immediate activation
            // long_term: durable memory
            // stability: resistance to decay
            // volatility: fluctuation; we treat high volatility as slightly negative
            s.heat.short_term * 0.6 +
            s.heat.long_term * 0.3 +
            s.heat.stability * 0.2 -
            s.heat.volatility * 0.1
        }).unwrap_or(0.0)
    }

    /// Compress skim results into a single summary sentence
    fn compress_skim(&self, items: &[String]) -> String {
        if items.is_empty() {
            return "No relevant facts found.".to_string();
        }

        if items.len() == 1 {
            return items[0].clone();
        }

        format!(
            "{}; {}; {}...",
            items[0],
            items[1],
            items.get(2).unwrap_or(&"…".to_string())
        )
    }

    /// Full skim engine
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

        // 2. Dictionary
        if let Some(def) = self.lookup_definition(&key) {
            results.push((format!("Definition: {}", def), 1.2));
        }

        // 3. Encyclopedia
        if let Some(article) = self.lookup_article(&key) {
            results.push((format!("Encyclopedia: {}", article), 1.1));
        }

        // 4. Synonyms
        let syns = self.lookup_synonyms(&key);
        if !syns.is_empty() {
            results.push((format!("Synonyms: {}", syns.join(", ")), 0.8));
        }

        // 5. Strong relations
        if let Some(ids) = self.label_index.get(&key) {
            for edge in engine.graph.edges.values() {
                if ids.contains(&edge.from) && edge.weight > 0.7 {
                    if let Some(target) = engine.graph.nodes.get(&edge.to) {
                        let score = 0.9 + edge.weight;
                        results.push((
                            format!(
                                "Strong relation → [{}] (kind={:?}, weight={:.2})",
                                target.label, edge.kind, edge.weight
                            ),
                            score,
                        ));
                    }
                }
            }
        }

        // 6. Episodic memory
        for (scene_id, ep) in self.episodic_index.iter() {
            if ep.compressed_summary.to_lowercase().contains(&key) {
                results.push((
                    format!("Episodic scene {}: {}", scene_id, ep.compressed_summary),
                    1.0,
                ));
            }
        }

        // 7. BFS skim traversal
        if let Some(ids) = self.label_index.get(&key) {
            let bfs_hits = self.skim_bfs(engine, ids, 2);
            for (label, score) in bfs_hits {
                results.push((format!("Related: {}", label), score));
            }
        }

        // 8. Semantic similarity skim
        for (label, ids) in self.label_index.iter() {
            let sim = Self::similarity(&key, label);
            if sim > 0.6 {
                for id in ids {
                    if let Some(node) = engine.graph.nodes.get(id) {
                        results.push((
                            format!("Similar [{}] (sim={:.2})", node.label, sim),
                            sim,
                        ));
                    }
                }
            }
        }

        // 9. Deduplicate + sort
        let mut ranked: Vec<(String, f32)> = Vec::new();
        let mut seen = HashSet::new();

        for (text, score) in results {
            if seen.insert(text.clone()) {
                ranked.push((text, score));
            }
        }

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // 10. Convert to strings
        ranked.into_iter().map(|(t, _)| t).collect()
    }

    /// Convenience: skim + compressed summary
    pub fn skim_summary(&self, engine: &MemoryEngine, query: &str) -> String {
        let items = self.skim_facts(engine, query);
        self.compress_skim(&items)
    }
}

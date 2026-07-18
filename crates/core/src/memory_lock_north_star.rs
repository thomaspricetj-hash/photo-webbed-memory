use std::collections::HashSet;

use crate::semantic_scene::EpisodicMemory;
use crate::engine::{MemoryEngine, MemoryHit};

/// MAX‑Tier Memory Lock:
/// Prevents decay, drift, pruning, and stability loss for protected memories.
/// Locks can be scene‑based or meaning‑based (keyword).
#[derive(Debug, Clone)]
pub struct MemoryLock {
    pub locked_scene_ids: HashSet<u64>,
    pub locked_keywords: HashSet<String>,
}

impl MemoryLock {
    pub fn new() -> Self {
        Self {
            locked_scene_ids: HashSet::new(),
            locked_keywords: HashSet::new(),
        }
    }

    pub fn lock_scene(&mut self, scene_id: u64) {
        self.locked_scene_ids.insert(scene_id);
    }

    pub fn lock_keyword(&mut self, keyword: &str) {
        self.locked_keywords.insert(keyword.to_lowercase());
    }

    pub fn is_locked_episode(&self, ep: &EpisodicMemory) -> bool {
        if self.locked_scene_ids.contains(&ep.scene_id) {
            return true;
        }
        let summary = ep.compressed_summary.to_lowercase();
        self.locked_keywords.iter().any(|kw| summary.contains(kw))
    }
}

/// MAX‑Tier North Star:
/// A global cognitive attractor that biases recall, stability, propagation,
/// and consolidation toward long‑term meaning goals.
#[derive(Debug, Clone)]
pub struct NorthStar {
    pub description: String,
    pub importance: f32,          // 0.0–1.0
    pub focus_keywords: Vec<String>,
    pub stability_bias: f32,      // increases node stability
    pub resonance_bias: f32,      // increases photonic propagation amplitude
}

impl NorthStar {
    pub fn new(
        description: &str,
        importance: f32,
        focus_keywords: Vec<String>,
        stability_bias: f32,
        resonance_bias: f32,
    ) -> Self {
        Self {
            description: description.to_string(),
            importance,
            focus_keywords: focus_keywords
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect(),
            stability_bias,
            resonance_bias,
        }
    }

    /// MAX‑tier relevance scoring:
    /// - keyword match
    /// - summary meaning match
    pub fn relevance_to_episode(&self, ep: &EpisodicMemory) -> f32 {
        let summary = ep.compressed_summary.to_lowercase();
        let mut score = 0.0;

        for kw in &self.focus_keywords {
            if summary.contains(kw) {
                score += 1.0;
            }
        }

        score * self.importance
    }
}

/// Combined MAX‑Tier Controller:
/// - Memory locking (hard protection)
/// - North Star (soft global bias)
/// - Stability physics
/// - Resonance biasing
/// - Drift resistance
/// - Consolidation hooks
#[derive(Debug, Clone)]
pub struct MemoryLockNorthStar {

    pub locks: MemoryLock,
    pub north_stars: Vec<NorthStar>,
}

impl MemoryLockNorthStar {
    pub fn new() -> Self {
        Self {
            locks: MemoryLock::new(),
            north_stars: Vec::new(),
        }
    }

    pub fn add_north_star(&mut self, ns: NorthStar) {
        self.north_stars.push(ns);
    }
}

/// MAX‑Tier Biasing (free function):
/// - Hard lock → strong score boost
/// - North Star → meaning‑based bias
/// - Stability boost → prevents drift/decay
/// - Resonance boost → improves propagation
pub fn apply_bias(
    lock_ns: &MemoryLockNorthStar,
    engine: &mut MemoryEngine,
    hits: &mut Vec<MemoryHit>,
) {
    for hit in hits.iter_mut() {
        if let Some(ep) = engine.semantic.episodes.get(&hit.scene_id).cloned() {
            // HARD LOCK: cannot decay, cannot drift, cannot be pruned.
            if lock_ns.locks.is_locked_episode(&ep) {
                hit.score += 2.0; // MAX‑tier boost

                // Strong stability physics
                engine.apply_stability_bias(ep.scene_id, 0.5);
            }

            // NORTH STAR: meaning‑anchored bias + physics
            for ns in &lock_ns.north_stars {
                let rel = ns.relevance_to_episode(&ep);
                if rel > 0.0 {
                    // Soft score bias
                    hit.score += rel * 0.25;

                    // Stability physics: increase node stability + long‑term heat
                    engine.apply_stability_bias(ep.scene_id, ns.stability_bias * rel);

                    // Resonance physics: increase photonic propagation amplitude
                    engine.apply_resonance_bias(ep.scene_id, ns.resonance_bias * rel);
                }
            }
        }
    }

    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
}

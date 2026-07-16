// src/semantic_scene.rs
//


use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// High-level semantic category for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticKind {
    Object,
    Actor,
    Action,
    Emotion,
    Environment,
    Event,
    Relation,
    Concept,
}

/// Basic semantic token extracted from input.
#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub text: String,
    pub kind: SemanticKind,
    pub salience: f32,
}

/// Relationship between two semantic entities.
#[derive(Debug, Clone)]
pub struct SemanticRelation {
    pub from_id: u64,
    pub to_id: u64,
    pub relation_type: String,
    pub weight: f32,
}

/// A node in the scene graph.
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: u64,
    pub label: String,
    pub kind: SemanticKind,
    pub salience: f32,
}

/// Contextual information about the scene.
#[derive(Debug, Clone, Default)]
pub struct SceneContext {
    pub location: Option<String>,
    pub time_of_day: Option<String>,
    pub mood: Option<String>,
    pub tags: HashSet<String>,
}

/// A scene graph representing “what is going on”.
#[derive(Debug, Clone)]
pub struct SceneGraph {
    pub nodes: HashMap<u64, SceneNode>,
    pub relations: Vec<SemanticRelation>,
    pub context: SceneContext,
    pub timestamp: u64,
}

/// Temporal binding between scenes.
#[derive(Debug, Clone)]
pub struct TemporalLink {
    pub from_scene_id: u64,
    pub to_scene_id: u64,
    pub relation: String,
    pub strength: f32,
}

/// Long-term episodic memory entry.
#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    pub scene_id: u64,
    pub graph: SceneGraph,
    pub temporal_links: Vec<TemporalLink>,
    pub compressed_summary: String,
}

/// Main semantic engine that sits on top of your MemoryEngine.
#[derive(Debug, Clone)]
pub struct SemanticEngine {
    pub episodes: HashMap<u64, EpisodicMemory>,
    pub next_id: u64,
}

impl SemanticEngine {
    pub fn new() -> Self {
        Self {
            episodes: HashMap::new(),
            next_id: 1,
        }
    }

    fn gen_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn now_ts() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// 1) Semantic Encoding Layer
    pub fn encode_text_scene(&mut self, text: &str) -> SceneGraph {
        let tokens = self.simple_tokenize(text);
        let mut nodes = HashMap::new();
        let mut relations = Vec::new();

        for token in tokens.iter() {
            let id = self.gen_id();
            nodes.insert(
                id,
                SceneNode {
                    id,
                    label: token.text.clone(),
                    kind: token.kind,
                    salience: token.salience,
                },
            );
        }

        let node_ids: Vec<u64> = nodes.keys().cloned().collect();
        for w in node_ids.windows(2) {
            relations.push(SemanticRelation {
                from_id: w[0],
                to_id: w[1],
                relation_type: "adjacent".to_string(),
                weight: 0.4,
            });
        }

        for (id, node) in nodes.iter() {
            if matches!(node.kind, SemanticKind::Actor) {
                for (other_id, other) in nodes.iter() {
                    if matches!(other.kind, SemanticKind::Action) {
                        relations.push(SemanticRelation {
                            from_id: *id,
                            to_id: *other_id,
                            relation_type: "performs".to_string(),
                            weight: 0.8,
                        });
                    }
                }
            }
        }

        let context = self.infer_context(text);

        SceneGraph {
            nodes,
            relations,
            context,
            timestamp: Self::now_ts(),
        }
    }

    /// 2) Scene Graph Builder + Episodic Memory
    pub fn store_scene(&mut self, graph: SceneGraph) -> u64 {
        let scene_id = self.gen_id();
        let summary = self.compress_scene(&graph);

        let episode = EpisodicMemory {
            scene_id,
            graph,
            temporal_links: Vec::new(),
            compressed_summary: summary,
        };

        self.episodes.insert(scene_id, episode);
        scene_id
    }

    /// 3) Temporal Binding
    pub fn link_scenes(&mut self, from_scene: u64, to_scene: u64, relation: &str, strength: f32) {
        if let Some(ep) = self.episodes.get_mut(&from_scene) {
            ep.temporal_links.push(TemporalLink {
                from_scene_id: from_scene,
                to_scene_id: to_scene,
                relation: relation.to_string(),
                strength,
            });
        }
    }

    /// 4) Meaning-based Compression
    fn compress_scene(&self, graph: &SceneGraph) -> String {
        let mut important: Vec<&SceneNode> = graph
            .nodes
            .values()
            .filter(|n| n.salience >= 0.5)
            .collect();

        important.sort_by(|a, b| b.salience.partial_cmp(&a.salience).unwrap_or(std::cmp::Ordering::Equal));

        let labels: Vec<String> = important.iter().map(|n| n.label.clone()).collect();

        let mut summary = String::new();
        if labels.is_empty() {
            summary.push_str("Key entities: (none)");
        } else {
            summary.push_str("Key entities: ");
            summary.push_str(&labels.join(", "));
        }

        if let Some(loc) = &graph.context.location {
            summary.push_str(&format!(" | Location: {}", loc));
        }
        if let Some(mood) = &graph.context.mood {
            summary.push_str(&format!(" | Mood: {}", mood));
        }
        if !graph.context.tags.is_empty() {
            let tags: Vec<String> = graph.context.tags.iter().cloned().collect();
            summary.push_str(&format!(" | Tags: {}", tags.join(", ")));
        }

        summary
    }

    /// 5) Simple recall by time.
    pub fn recall_recent(&self, limit: usize) -> Vec<&EpisodicMemory> {
        let mut episodes: Vec<&EpisodicMemory> = self.episodes.values().collect();
        episodes.sort_by(|a, b| b.graph.timestamp.cmp(&a.graph.timestamp));
        episodes.into_iter().take(limit).collect()
    }

    /// Recall by keyword in compressed summary.
    pub fn recall_by_keyword(&self, keyword: &str) -> Vec<&EpisodicMemory> {
        let kw = keyword.to_lowercase();
        self.episodes
            .values()
            .filter(|ep| ep.compressed_summary.to_lowercase().contains(&kw))
            .collect()
    }

    // --- Helpers ---------------------------------------------------------

    fn simple_tokenize(&self, text: &str) -> Vec<SemanticToken> {
        text.split_whitespace()
            .filter(|w| !w.is_empty())
            .map(|w| {
                let cleaned = w.trim_matches(|c: char| !c.is_alphanumeric());
                SemanticToken {
                    text: cleaned.to_string(),
                    kind: self.guess_kind(cleaned),
                    salience: self.guess_salience(cleaned),
                }
            })
            .collect()
    }

    fn guess_kind(&self, word: &str) -> SemanticKind {
        let lw = word.to_lowercase();
        if ["he", "she", "they", "man", "woman", "person", "child"].contains(&lw.as_str()) {
            SemanticKind::Actor
        } else if ["runs", "walks", "talks", "looks", "sits", "stands", "holds"].contains(&lw.as_str()) {
            SemanticKind::Action
        } else if ["happy", "sad", "angry", "calm", "excited", "tired"].contains(&lw.as_str()) {
            SemanticKind::Emotion
        } else if ["room", "street", "park", "house", "office", "car"].contains(&lw.as_str()) {
            SemanticKind::Environment
        } else if ["party", "meeting", "fight", "conversation", "game"].contains(&lw.as_str()) {
            SemanticKind::Event
        } else {
            SemanticKind::Object
        }
    }

    fn guess_salience(&self, word: &str) -> f32 {
        let lw = word.to_lowercase();
        if ["he", "she", "they", "man", "woman", "person"].contains(&lw.as_str()) {
            0.9
        } else if ["party", "meeting", "fight", "game"].contains(&lw.as_str()) {
            0.85
        } else {
            0.7
        }
    }

    fn infer_context(&self, text: &str) -> SceneContext {
        let mut ctx = SceneContext::default();
        let lower = text.to_lowercase();

        if lower.contains("night") {
            ctx.time_of_day = Some("night".to_string());
            ctx.tags.insert("night".to_string());
        } else if lower.contains("morning") {
            ctx.time_of_day = Some("morning".to_string());
            ctx.tags.insert("morning".to_string());
        }

        if lower.contains("party") {
            ctx.mood = Some("social".to_string());
            ctx.tags.insert("party".to_string());
            ctx.tags.insert("social".to_string());
        } else if lower.contains("meeting") {
            ctx.mood = Some("formal".to_string());
            ctx.tags.insert("meeting".to_string());
        }

        if lower.contains("park") {
            ctx.location = Some("park".to_string());
        } else if lower.contains("office") {
            ctx.location = Some("office".to_string());
        } else if lower.contains("street") {
            ctx.location = Some("street".to_string());
        }

        ctx
    }
}

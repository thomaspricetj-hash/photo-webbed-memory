use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

// Import unified MemoryHit + RetrievalRanker from engine
use crate::engine::{MemoryHit, RetrievalRanker};

// ============================================================
// Tier 1–2: Core Semantic / Episodic Structures
// ============================================================

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

#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub text: String,
    pub kind: SemanticKind,
    pub salience: f32,
}

#[derive(Debug, Clone)]
pub struct SemanticRelation {
    pub from_id: u64,
    pub to_id: u64,
    pub relation_type: String,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: u64,
    pub label: String,
    pub kind: SemanticKind,
    pub salience: f32,
    pub embedding: Option<Vec<f32>>, // Tier‑4: optional embedding
}

#[derive(Debug, Clone, Default)]
pub struct SceneContext {
    pub location: Option<String>,
    pub time_of_day: Option<String>,
    pub mood: Option<String>,
    pub tags: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct SceneGraph {
    pub nodes: HashMap<u64, SceneNode>,
    pub relations: Vec<SemanticRelation>,
    pub context: SceneContext,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct TemporalLink {
    pub from_scene_id: u64,
    pub to_scene_id: u64,
    pub relation: String,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    pub scene_id: u64,
    pub graph: SceneGraph,
    pub temporal_links: Vec<TemporalLink>,
    pub compressed_summary: String,
}

// ============================================================
// Tier‑4: Embedding Provider + Hybrid Retrieval + Ingestion
// ============================================================

pub trait EmbeddingProvider {
    fn embed(&self, text: &str) -> Vec<f32>;
}

pub struct DummyEmbeddingProvider;

impl EmbeddingProvider for DummyEmbeddingProvider {
    fn embed(&self, text: &str) -> Vec<f32> {
        text.bytes().map(|b| (b as f32) / 255.0).collect()
    }
}

// Document ingestion utilities
pub struct DocumentIngestor;

impl DocumentIngestor {
    pub fn extract_text(path: &str) -> String {
        format!("(document from path: {})", path)
    }

    pub fn chunk_text(text: &str, max_len: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut current = Vec::new();
        let mut len = 0;

        for w in words {
            let wlen = w.len();
            if len + wlen > max_len && !current.is_empty() {
                chunks.push(current.join(" "));
                current.clear();
                len = 0;
            }
            current.push(w);
            len += wlen + 1;
        }

        if !current.is_empty() {
            chunks.push(current.join(" "));
        }

        chunks
    }
}

// ============================================================
// Tier‑3 MAX: SemanticEngine
// ============================================================

pub struct SemanticEngine {
    pub episodes: HashMap<u64, EpisodicMemory>,
    pub next_id: u64,
    pub embedder: Box<dyn EmbeddingProvider + Send + Sync>,
}

impl std::fmt::Debug for SemanticEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SemanticEngine")
            .field("episodes_len", &self.episodes.len())
            .field("next_id", &self.next_id)
            .finish()
    }
}

impl SemanticEngine {
    pub fn new(embedder: Box<dyn EmbeddingProvider + Send + Sync>) -> Self {
        Self {
            episodes: HashMap::new(),
            next_id: 1,
            embedder,
        }
    }

    fn gen_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn now_ts() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    // ------------------------------------------------------------
    // Tier‑3: Semantic Encoding
    // ------------------------------------------------------------

    pub fn encode_text_scene(&mut self, text: &str) -> SceneGraph {
        let tokens = self.simple_tokenize(text);
        let mut nodes = HashMap::new();
        let mut relations = Vec::new();

        for token in tokens.iter() {
            let id = self.gen_id();
            let embedding = Some(self.embedder.embed(&token.text));
            nodes.insert(
                id,
                SceneNode {
                    id,
                    label: token.text.clone(),
                    kind: token.kind,
                    salience: token.salience,
                    embedding,
                },
            );
        }

        // Adjacent relations
        let node_ids: Vec<u64> = nodes.keys().cloned().collect();
        for w in node_ids.windows(2) {
            relations.push(SemanticRelation {
                from_id: w[0],
                to_id: w[1],
                relation_type: "adjacent".to_string(),
                weight: 0.4,
            });
        }

        // Actor → Action relations
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

    // ------------------------------------------------------------
    // Tier‑3: Episodic Storage
    // ------------------------------------------------------------

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

    pub fn summarize_scene(&self, graph: &SceneGraph) -> String {
        self.compress_scene(graph)
    }

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

    // ------------------------------------------------------------
    // Tier‑3: Meaning Compression
    // ------------------------------------------------------------

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

    // ------------------------------------------------------------
    // Tier‑3: Episodic Recall
    // ------------------------------------------------------------

    pub fn recall_recent(&self, limit: usize) -> Vec<&EpisodicMemory> {
        let mut episodes: Vec<&EpisodicMemory> = self.episodes.values().collect();
        episodes.sort_by(|a, b| b.graph.timestamp.cmp(&a.graph.timestamp));
        episodes.into_iter().take(limit).collect()
    }

    pub fn recall_by_keyword(&self, keyword: &str) -> Vec<&EpisodicMemory> {
        let kw = keyword.to_lowercase();
        self.episodes
            .values()
            .filter(|ep| ep.compressed_summary.to_lowercase().contains(&kw))
            .collect()
    }

    // ------------------------------------------------------------
    // Tier‑3: Reflex Episodic Trace
    // ------------------------------------------------------------

    pub fn store_reflex_event(&mut self, label: String, now: u64) {
        let scene_id = self.gen_id();

        let mut nodes = HashMap::new();
        nodes.insert(
            scene_id,
            SceneNode {
                id: scene_id,
                label: label.clone(),
                kind: SemanticKind::Event,
                salience: 0.9,
                embedding: Some(self.embedder.embed(&label)),
            },
        );

        let graph = SceneGraph {
            nodes,
            relations: vec![],
            context: SceneContext::default(),
            timestamp: now,
        };

        let episode = EpisodicMemory {
            scene_id,
            graph,
            temporal_links: vec![],
            compressed_summary: format!("Reflex event: {}", label),
        };

        self.episodes.insert(scene_id, episode);
    }

    // ------------------------------------------------------------
    // Tier‑4: Vector Search
    // ------------------------------------------------------------

    pub fn vector_search(&self, query: &str, top_k: usize) -> Vec<MemoryHit> {
        let q_emb = self.embedder.embed(query);
        let mut hits = Vec::new();

        for (scene_id, ep) in self.episodes.iter() {
            let mut best = 0.0;
            for node in ep.graph.nodes.values() {
                if let Some(ref emb) = node.embedding {
                    let score = cosine(&q_emb, emb);
                    if score > best {
                        best = score;
                    }
                }
            }
            if best > 0.0 {
                hits.push(MemoryHit {
                    scene_id: *scene_id,
                    score: best,
                    source: "vector".to_string(),
                });
            }
        }

        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().take(top_k).collect()
    }

    // ------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------

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

// ------------------------------------------------------------
// Cosine Similarity
// ------------------------------------------------------------

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    let len = a.len().min(b.len());
    for i in 0..len {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

// ============================================================
// Tier‑5: Procedural Memory
// ============================================================

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub trigger_keyword: String,
    pub action_description: String,
    pub priority: f32,
}

#[derive(Debug, Clone)]
pub struct ProceduralMemory {
    pub procedures: Vec<Procedure>,
}

impl ProceduralMemory {
    pub fn new() -> Self {
        Self { procedures: Vec::new() }
    }

    pub fn add_procedure(&mut self, name: &str, trigger_keyword: &str, action: &str, priority: f32) {
        self.procedures.push(Procedure {
            name: name.to_string(),
            trigger_keyword: trigger_keyword.to_string(),
            action_description: action.to_string(),
            priority,
        });
    }

    pub fn match_procedures(&self, context: &str) -> Vec<&Procedure> {
        let lower = context.to_lowercase();
        let mut matches: Vec<&Procedure> = self
            .procedures
            .iter()
            .filter(|p| lower.contains(&p.trigger_keyword.to_lowercase()))
            .collect();

        matches.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        matches
    }
}

// ============================================================
// Tier‑6: AgentCore (Legacy)
// ============================================================

#[derive(Debug, Clone)]
pub struct Goal {
    pub description: String,
    pub importance: f32,
    pub deadline_ts: Option<u64>,
}

#[derive(Debug)]
pub struct AgentCore {
    pub memory: MemoryEngine,
    pub goals: Vec<Goal>,
}

impl AgentCore {
    pub fn new() -> Self {
        Self {
            memory: MemoryEngine::new(),
            goals: Vec::new(),
        }
    }

    pub fn add_goal(&mut self, description: &str, importance: f32, deadline_ts: Option<u64>) {
        self.goals.push(Goal {
            description: description.to_string(),
            importance,
            deadline_ts,
        });
    }

    pub fn perceive_text(&mut self, text: &str) -> u64 {
        self.memory.encode_scene(text)
    }

    pub fn reflex(&mut self, label: &str) {
        self.memory.store_reflex(label);
    }

    pub fn decide_action(&mut self, context: &str) -> Option<String> {
        let procedures = self.memory.procedural.match_procedures(context);
        procedures.first().map(|p| p.action_description.clone())
    }

    pub fn recall(&self, query: &str, top_k: usize) -> Vec<MemoryHit> {
        self.memory.hybrid_search(query, top_k)
    }
}

// ============================================================
// Legacy MemoryEngine (kept intact)
// ============================================================

#[derive(Debug)]
pub struct MemoryEngine {
    pub semantic: SemanticEngine,
    pub procedural: ProceduralMemory,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            semantic: SemanticEngine::new(Box::new(DummyEmbeddingProvider)),
            procedural: ProceduralMemory::new(),
        }
    }

    // Tier‑3: basic scene encoding
    pub fn encode_scene(&mut self, text: &str) -> u64 {
        let graph = self.semantic.encode_text_scene(text);
        self.semantic.store_scene(graph)
    }

    pub fn recall_recent_scenes(&self, limit: usize) -> Vec<&EpisodicMemory> {
        self.semantic.recall_recent(limit)
    }

    pub fn recall_scene_by_keyword(&self, keyword: &str) -> Vec<&EpisodicMemory> {
        self.semantic.recall_by_keyword(keyword)
    }

    pub fn link_scenes(&mut self, from: u64, to: u64, relation: &str, strength: f32) {
        self.semantic.link_scenes(from, to, relation, strength)
    }

    pub fn store_reflex(&mut self, label: &str) {
        let now = SemanticEngine::now_ts();
        self.semantic.store_reflex_event(label.to_string(), now);
    }

    // Tier‑4: ingestion + hybrid retrieval
    pub fn ingest_document(&mut self, path: &str) {
        let text = DocumentIngestor::extract_text(path);
        let chunks = DocumentIngestor::chunk_text(&text, 512);
        for chunk in chunks {
            let graph = self.semantic.encode_text_scene(&chunk);
            self.semantic.store_scene(graph);
        }
    }

    pub fn hybrid_search(&self, query: &str, top_k: usize) -> Vec<MemoryHit> {
        // Episodic: keyword in compressed summary
        let episodic_hits: Vec<MemoryHit> = self
            .semantic
            .recall_by_keyword(query)
            .into_iter()
            .map(|ep| MemoryHit {
                scene_id: ep.scene_id,
                score: 0.6,
                source: "episodic".to_string(),
            })
            .collect();

        // Semantic: simple keyword in nodes
        let mut semantic_hits = Vec::new();
        let q_lower = query.to_lowercase();
        for (scene_id, ep) in self.semantic.episodes.iter() {
            let mut score = 0.0;
            for node in ep.graph.nodes.values() {
                if node.label.to_lowercase().contains(&q_lower) {
                    score += node.salience;
                }
            }
            if score > 0.0 {
                semantic_hits.push(MemoryHit {
                    scene_id: *scene_id,
                    score,
                    source: "semantic".to_string(),
                });
            }
        }

        // Vector: embedding similarity
        let vector_hits = self.semantic.vector_search(query, top_k);

        // Reflex: recent reflex events
        let reflex_hits: Vec<MemoryHit> = self
            .semantic
            .recall_recent(16)
            .into_iter()
            .filter(|ep| ep.compressed_summary.to_lowercase().contains("reflex"))
            .map(|ep| MemoryHit {
                scene_id: ep.scene_id,
                score: 0.9,
                source: "reflex".to_string(),
            })
            .collect();

        RetrievalRanker::merge_and_rank(episodic_hits, semantic_hits, vector_hits, reflex_hits)
    }
}

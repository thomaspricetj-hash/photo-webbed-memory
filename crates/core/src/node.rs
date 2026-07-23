use rand::Rng;
use serde::{Serialize, Deserialize};

/// Unique node identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

/// Types of nodes in the cognitive graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Token,
    Concept,
    Document,
    Event,
    Memory,
    Scratch,
    Summary,
}

/// Maximum semantic weight
const MAX_SEMANTIC_WEIGHT: f32 = 50.0;

/// Tier‑3 MAX Node structure with full cognitive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,

    /// Human-readable label
    pub label: String,

    /// Category of node
    pub kind: NodeKind,

    /// 3D position for visualization
    pub position: [f32; 3],

    /// Semantic weight (importance)
    pub semantic_weight: f32,

    /// Confidence score (0–1)
    pub confidence: f32,

    /// Number of activations
    pub activation_count: u64,

    /// Last time activated
    pub last_access: u64,

    /// Age of node (ticks)
    pub age: u64,

    /// Hive alignment (0–1): how strongly this node aligns with hive generalization
    pub hive_alignment: f32,

    /// Cluster alignment (0–1): strength of cluster membership
    pub cluster_alignment: f32,

    /// Semantic gravity (0–1): pull toward conceptual center
    pub gravity: f32,

    // ============================================================
    // 🔥 Tier‑7 Roundabout Routing Additive Fields
    // ============================================================

    /// Zone ID for roundabout routing (semantic or spatial region)
    pub zone: u32,

    /// Centroid ID for region clustering
    pub centroid: u32,

    /// Drift magnitude (0–1): directional instability
    pub drift: f32,

    /// Region stability (0–1): how stable this node’s region is
    pub region_stability: f32,

    /// Exit weight bias for roundabout routing
    pub exit_bias: f32,
}

impl Node {
    /// Create a new node with safe defaults
    pub fn new(label: &str, kind: NodeKind) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            id: NodeId(rng.gen()),
            label: label.to_string(),
            kind,

            position: [
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ],

            semantic_weight: 1.0,
            confidence: 0.5,
            activation_count: 0,
            last_access: 0,
            age: 0,

            hive_alignment: 0.0,
            cluster_alignment: 0.0,
            gravity: 0.0,

            // Tier‑7 roundabout defaults
            zone: rng.gen_range(1..=8),
            centroid: rng.gen_range(1..=64),
            drift: 0.0,
            region_stability: 0.5,
            exit_bias: 1.0,
        }
    }

    /// Tier‑3 MAX reinforcement
    pub fn reinforce(&mut self, now: u64) {
        self.activation_count += 1;
        self.last_access = now;

        let nonlinear = 1.0 / (1.0 + 0.04 * self.activation_count as f32);

        self.semantic_weight += 0.35 * nonlinear;
        if self.semantic_weight > MAX_SEMANTIC_WEIGHT {
            self.semantic_weight = MAX_SEMANTIC_WEIGHT;
        }

        self.confidence += 0.04 * nonlinear;
        if self.confidence > 1.0 {
            self.confidence = 1.0;
        }

        self.hive_alignment = (self.hive_alignment + 0.03).min(1.0);
        self.cluster_alignment = (self.cluster_alignment + 0.02).min(1.0);

        let gravity_boost = (self.semantic_weight / MAX_SEMANTIC_WEIGHT) * 0.4
            + self.confidence * 0.3;
        self.gravity = (self.gravity + gravity_boost).min(1.0);

        // ============================================================
        // 🔥 Tier‑7 Roundabout Reinforcement Additive Logic
        // ============================================================

        // Region stability increases with reinforcement
        self.region_stability = (self.region_stability + 0.02).min(1.0);

        // Drift decreases with reinforcement (node becomes more stable)
        self.drift *= 0.95;

        // Exit bias increases slightly for nodes that are frequently reinforced
        self.exit_bias = (self.exit_bias + 0.01).min(2.0);
    }

    /// Tier‑3 MAX decay
    pub fn decay(&mut self, dt: f32) {
        self.age += dt as u64;

        let aging_factor = 1.0 + (self.age as f32 * 0.003);

        self.semantic_weight *= f32::exp(-0.005 * dt * aging_factor);
        self.confidence *= f32::exp(-0.002 * dt * aging_factor);

        self.hive_alignment *= f32::exp(-0.001 * dt);
        self.cluster_alignment *= f32::exp(-0.001 * dt);
        self.gravity *= f32::exp(-0.003 * dt);

        if self.semantic_weight < 0.0003 {
            self.semantic_weight = 0.0;
        }
        if self.confidence < 0.0003 {
            self.confidence = 0.0;
        }
        if self.hive_alignment < 0.0003 {
            self.hive_alignment = 0.0;
        }
        if self.cluster_alignment < 0.0003 {
            self.cluster_alignment = 0.0;
        }
        if self.gravity < 0.0003 {
            self.gravity = 0.0;
        }

        // ============================================================
        // 🔥 Tier‑7 Roundabout Decay Additive Logic
        // ============================================================

        // Drift increases slightly with age (node becomes less stable)
        self.drift = (self.drift + 0.002 * dt).min(1.0);

        // Region stability decays slowly
        self.region_stability *= f32::exp(-0.001 * dt);

        // Exit bias decays very slowly
        self.exit_bias *= f32::exp(-0.0005 * dt);

        // Remove tiny noise
        if self.drift < 0.0003 {
            self.drift = 0.0;
        }
        if self.region_stability < 0.0003 {
            self.region_stability = 0.0;
        }
        if self.exit_bias < 0.0003 {
            self.exit_bias = 0.0;
        }
    }

    /// Check if node is effectively dead
    pub fn is_dead(&self) -> bool {
        self.semantic_weight == 0.0 &&
        self.confidence == 0.0 &&
        self.hive_alignment == 0.0 &&
        self.cluster_alignment == 0.0 &&
        self.gravity == 0.0
    }

    // ============================================================
    // 🔥 Tier‑7 Roundabout Routing Scoring Additive Methods
    // ============================================================

    /// Compute roundabout stability score for this node.
    pub fn roundabout_stability(&self) -> f32 {
        let drift_penalty = 1.0 / (1.0 + self.drift * 1.25);
        (self.region_stability * drift_penalty).clamp(0.0, 1.0)
    }

    /// Compute roundabout exit score for routing decisions.
    pub fn roundabout_exit_score(&self) -> f32 {
        let stability = self.roundabout_stability();
        let gravity = self.gravity;
        let confidence = self.confidence;

        let base = stability * 0.5 + gravity * 0.3 + confidence * 0.2;
        base * self.exit_bias
    }

    /// Compute full roundabout score (used by linkers + index).
    pub fn roundabout_score(&self) -> f32 {
        let s = self.roundabout_stability();
        let e = self.roundabout_exit_score();
        (s * 0.55) + (e * 0.45)
    }
}


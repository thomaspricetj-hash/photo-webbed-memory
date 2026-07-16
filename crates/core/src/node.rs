use rand::Rng;

/// Unique node identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Types of nodes in the cognitive graph
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Token,
    Concept,
    Document,
    Event,
    Memory,
    Scratch,
}


/// Maximum semantic weight
const MAX_SEMANTIC_WEIGHT: f32 = 50.0;

/// Node structure with full cognitive metadata
#[derive(Debug, Clone)]
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
}

impl Node {
    /// Create a new node with safe defaults
    pub fn new(label: &str, kind: NodeKind) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            id: NodeId(rng.gen()),
            label: label.to_string(),
            kind,

            // Random initial 3D position
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
        }
    }

    /// Reinforce node when activated
    pub fn reinforce(&mut self, now: u64) {
        self.activation_count += 1;
        self.last_access = now;

        // Increase semantic weight with diminishing returns
        self.semantic_weight += 0.3 * (1.0 / (1.0 + 0.05 * self.activation_count as f32));
        if self.semantic_weight > MAX_SEMANTIC_WEIGHT {
            self.semantic_weight = MAX_SEMANTIC_WEIGHT;
        }

        // Increase confidence
        self.confidence += 0.03;
        if self.confidence > 1.0 {
            self.confidence = 1.0;
        }
    }

    /// Apply decay over time
    pub fn decay(&mut self, dt: f32) {
        self.age += dt as u64;

        // Semantic weight decays slowly
        self.semantic_weight *= f32::exp(-0.005 * dt);

        // Confidence decays even slower
        self.confidence *= f32::exp(-0.002 * dt);

        // Noise filtering
        if self.semantic_weight < 0.0003 {
            self.semantic_weight = 0.0;
        }
        if self.confidence < 0.0003 {
            self.confidence = 0.0;
        }
    }

    /// Check if node is effectively dead
    pub fn is_dead(&self) -> bool {
        self.semantic_weight == 0.0 && self.confidence == 0.0
    }
}

use rand::Rng;
use crate::node::NodeId;

/// Unique edge identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);

/// Types of edges in the cognitive graph
#[derive(Debug, Clone, PartialEq)]
pub enum EdgeKind {
    Semantic,     // meaning-based
    Temporal,     // time-based sequence
    Causal,       // cause → effect
    Reference,    // pointer or citation
    Associative,  // loose conceptual link
    Reinforced,   // strengthened through repeated activation
}

/// Maximum allowed weight to prevent runaway growth
const MAX_WEIGHT: f32 = 10.0;

/// Minimum noise threshold
const NOISE_FLOOR: f32 = 0.0003;

/// Edge structure with full cognitive metadata
#[derive(Debug, Clone)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub kind: EdgeKind,

    /// Strength of the relationship
    pub weight: f32,

    /// Confidence score (0–1)
    pub confidence: f32,

    /// Number of times this edge was activated
    pub activation_count: u64,

    /// Last time this edge was used
    pub last_access: u64,

    /// Age of the edge (in ticks)
    pub age: u64,
}

impl Edge {
    /// Create a new edge with safe defaults
    pub fn new(from: NodeId, to: NodeId, kind: EdgeKind, weight: f32) -> Self {
        let mut rng = rand::thread_rng();
        let id = EdgeId(rng.gen());

        Self {
            id,
            from,
            to,
            kind,
            weight: weight.clamp(0.0, MAX_WEIGHT),
            confidence: 0.5,
            activation_count: 0,
            last_access: 0,
            age: 0,
        }
    }

    /// Reinforce the edge when activated
    pub fn reinforce(&mut self, now: u64) {
        self.activation_count += 1;
        self.last_access = now;

        // Increase weight with diminishing returns
        self.weight += 0.2 * (1.0 / (1.0 + 0.05 * self.activation_count as f32));

        // Clamp weight
        if self.weight > MAX_WEIGHT {
            self.weight = MAX_WEIGHT;
        }

        // Increase confidence
        self.confidence += 0.05;
        if self.confidence > 1.0 {
            self.confidence = 1.0;
        }

        // Upgrade kind if heavily reinforced
        if self.activation_count > 20 {
            self.kind = EdgeKind::Reinforced;
        }
    }

    /// Apply decay over time
    pub fn decay(&mut self, dt: f32) {
        self.age += dt as u64;

        // Weight decays slowly
        self.weight *= f32::exp(-0.01 * dt);

        // Confidence decays even slower
        self.confidence *= f32::exp(-0.005 * dt);

        // Noise filtering
        if self.weight < NOISE_FLOOR {
            self.weight = 0.0;
        }
        if self.confidence < NOISE_FLOOR {
            self.confidence = 0.0;
        }
    }

    /// Check if edge is effectively dead
    pub fn is_dead(&self) -> bool {
        self.weight == 0.0 && self.confidence == 0.0
    }
}

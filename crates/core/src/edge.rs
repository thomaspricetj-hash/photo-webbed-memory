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
const MAX_WEIGHT: f32 = 12.0;

/// Minimum noise threshold
const NOISE_FLOOR: f32 = 0.00025;

/// Nonlinear reinforcement curve
const REINFORCE_RATE: f32 = 0.18;

/// Confidence reinforcement rate
const CONFIDENCE_RATE: f32 = 0.045;

/// Aging pressure: older edges decay faster unless reinforced
const AGING_PRESSURE: f32 = 0.004;

/// Decay constants
const WEIGHT_DECAY: f32 = 0.009;
const CONFIDENCE_DECAY: f32 = 0.004;

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

    /// Reinforce the edge when activated (Tier‑3)
    pub fn reinforce(&mut self, now: u64) {
        self.activation_count += 1;
        self.last_access = now;

        // Nonlinear reinforcement curve
        let nonlinear = (1.0 / (1.0 + 0.04 * self.activation_count as f32)).max(0.05);

        // Weight reinforcement
        self.weight += REINFORCE_RATE * nonlinear;
        if self.weight > MAX_WEIGHT {
            self.weight = MAX_WEIGHT;
        }

        // Confidence reinforcement
        self.confidence += CONFIDENCE_RATE * nonlinear;
        if self.confidence > 1.0 {
            self.confidence = 1.0;
        }

        // Upgrade kind if heavily reinforced
        if self.activation_count > 18 {
            self.kind = EdgeKind::Reinforced;
        }

        // Reset aging pressure when reinforced
        self.age = 0;
    }

    /// Apply decay over time (Tier‑3)
    pub fn decay(&mut self, dt: f32) {
        self.age += dt as u64;

        // Aging pressure: older edges decay faster unless reinforced
        let aging_factor = 1.0 + (self.age as f32 * AGING_PRESSURE);

        // Weight decay
        self.weight *= f32::exp(-WEIGHT_DECAY * dt * aging_factor);

        // Confidence decay
        self.confidence *= f32::exp(-CONFIDENCE_DECAY * dt * aging_factor);

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


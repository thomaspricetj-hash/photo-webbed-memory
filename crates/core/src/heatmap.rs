use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatLayer {
    /// Fast‑changing activation (working memory)
    pub short_term: f32,

    /// Slow‑changing importance (long‑term memory)
    pub long_term: f32,

    /// Stability curve (0–1): resistance to decay
    pub stability: f32,

    /// Volatility curve (0–1): sensitivity to new activation
    pub volatility: f32,

    /// Resonance field (0–1): alignment with global activation rhythm
    pub resonance: f32,

    /// Inertia field: resistance to sudden change (0–1)
    pub inertia: f32,
}

impl HeatLayer {
    /// Create a new heat layer with safe defaults
    pub fn new() -> Self {
        Self {
            short_term: 0.0,
            long_term: 0.0,
            stability: 0.0,
            volatility: 1.0,
            resonance: 0.0,
            inertia: 0.0,
        }
    }

    /// Apply saturation control to prevent runaway values
    pub fn clamp(&mut self) {
        const SHORT_MAX: f32 = 12.0;
        const LONG_MAX: f32 = 140.0;

        self.short_term = self.short_term.min(SHORT_MAX);
        self.long_term = self.long_term.min(LONG_MAX);

        self.stability = self.stability.clamp(0.0, 1.0);
        self.volatility = self.volatility.clamp(0.0, 1.0);
        self.resonance = self.resonance.clamp(0.0, 1.0);
        self.inertia = self.inertia.clamp(0.0, 1.0);
    }

    /// Remove tiny noise values
    pub fn denoise(&mut self) {
        const NOISE_FLOOR: f32 = 0.00025;

        if self.short_term < NOISE_FLOOR {
            self.short_term = 0.0;
        }
        if self.long_term < NOISE_FLOOR {
            self.long_term = 0.0;
        }
        if self.resonance < NOISE_FLOOR {
            self.resonance = 0.0;
        }
        if self.inertia < NOISE_FLOOR {
            self.inertia = 0.0;
        }
    }

    /// Increase stability when long‑term memory grows
    pub fn update_stability(&mut self) {
        let boost = 0.01 * self.long_term + 0.02 * self.resonance;
        self.stability = (self.stability + boost).min(1.0);
    }

    /// Increase volatility when short‑term spikes
    pub fn update_volatility(&mut self) {
        let spike = 0.02 * self.short_term * (1.0 - self.inertia);
        self.volatility = (self.volatility + spike).min(1.0);
    }

    /// Update resonance based on global rhythm similarity
    pub fn update_resonance(&mut self, global_rhythm: f32) {
        let local = self.short_term;
        let similarity = 1.0 - (global_rhythm - local).abs().min(1.0);

        let delta = similarity * 0.08 * (0.5 + self.stability);
        self.resonance = (self.resonance + delta).min(1.0);
    }

    /// Update inertia: resistance to sudden change
    pub fn update_inertia(&mut self) {
        let delta = 0.015 * self.long_term + 0.02 * self.stability;
        self.inertia = (self.inertia + delta).min(1.0);
    }
}

#[derive(Debug, Clone)]
pub struct HeatLayer {
    /// Fast‑changing activation (working memory)
    pub short_term: f32,

    /// Slow‑changing importance (long‑term memory)
    pub long_term: f32,

    /// Stability curve (0–1): resistance to decay
    pub stability: f32,

    /// Volatility curve (0–1): sensitivity to new activation
    pub volatility: f32,
}

impl HeatLayer {
    /// Create a new heat layer with safe defaults
    pub fn new() -> Self {
        Self {
            short_term: 0.0,
            long_term: 0.0,
            stability: 0.0,
            volatility: 1.0,
        }
    }

    /// Apply saturation control to prevent runaway values
    pub fn clamp(&mut self) {
        const SHORT_MAX: f32 = 10.0;
        const LONG_MAX: f32 = 100.0;

        if self.short_term > SHORT_MAX {
            self.short_term = SHORT_MAX;
        }
        if self.long_term > LONG_MAX {
            self.long_term = LONG_MAX;
        }

        self.stability = self.stability.clamp(0.0, 1.0);
        self.volatility = self.volatility.clamp(0.0, 1.0);
    }

    /// Remove tiny noise values
    pub fn denoise(&mut self) {
        const NOISE_FLOOR: f32 = 0.0003;

        if self.short_term < NOISE_FLOOR {
            self.short_term = 0.0;
        }
        if self.long_term < NOISE_FLOOR {
            self.long_term = 0.0;
        }
    }

    /// Increase stability when long‑term memory grows
    pub fn update_stability(&mut self) {
        // Stability grows with long-term memory
        self.stability = (self.stability + 0.01 * self.long_term).min(1.0);
    }

    /// Increase volatility when short‑term spikes
    pub fn update_volatility(&mut self) {
        // Volatility increases with short-term activation
        self.volatility = (self.volatility + 0.02 * self.short_term).min(1.0);
    }
}

use serde::{Serialize, Deserialize};

/// Cognitive heat layer with cross‑section mapping.
/// This is now a full spatial‑temporal‑directional memory cell.
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

    /// Cross‑section spatial memory (front/back/left/right)
    pub front: f32,
    pub back: f32,
    pub left: f32,
    pub right: f32,

    /// Quadrant memory (Q1–Q4)
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,

    /// Radial memory (inner/mid/outer)
    pub inner: f32,
    pub mid: f32,
    pub outer: f32,

    /// Motion‑vector drift memory
    pub drift_dx: f32,
    pub drift_dy: f32,

    /// Temporal stability memory
    pub temporal_stability: f32,
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

            front: 0.0,
            back: 0.0,
            left: 0.0,
            right: 0.0,

            q1: 0.0,
            q2: 0.0,
            q3: 0.0,
            q4: 0.0,

            inner: 0.0,
            mid: 0.0,
            outer: 0.0,

            drift_dx: 0.0,
            drift_dy: 0.0,

            temporal_stability: 1.0,
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

        self.front = self.front.clamp(0.0, 1.0);
        self.back = self.back.clamp(0.0, 1.0);
        self.left = self.left.clamp(0.0, 1.0);
        self.right = self.right.clamp(0.0, 1.0);

        self.q1 = self.q1.clamp(0.0, 1.0);
        self.q2 = self.q2.clamp(0.0, 1.0);
        self.q3 = self.q3.clamp(0.0, 1.0);
        self.q4 = self.q4.clamp(0.0, 1.0);

        self.inner = self.inner.clamp(0.0, 1.0);
        self.mid = self.mid.clamp(0.0, 1.0);
        self.outer = self.outer.clamp(0.0, 1.0);

        self.drift_dx = self.drift_dx.clamp(-1.0, 1.0);
        self.drift_dy = self.drift_dy.clamp(-1.0, 1.0);

        self.temporal_stability = self.temporal_stability.clamp(0.0, 1.0);
    }

    /// Remove tiny noise values
    pub fn denoise(&mut self) {
        const NOISE_FLOOR: f32 = 0.00025;

        macro_rules! clean {
            ($field:expr) => {
                if $field < NOISE_FLOOR {
                    $field = 0.0;
                }
            };
        }

        clean!(self.short_term);
        clean!(self.long_term);
        clean!(self.resonance);
        clean!(self.inertia);

        clean!(self.front);
        clean!(self.back);
        clean!(self.left);
        clean!(self.right);

        clean!(self.q1);
        clean!(self.q2);
        clean!(self.q3);
        clean!(self.q4);

        clean!(self.inner);
        clean!(self.mid);
        clean!(self.outer);

        clean!(self.temporal_stability);
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

    /// Update cross‑section mapping from spatial slices
    pub fn update_cross_sections(
        &mut self,
        front: f32,
        back: f32,
        left: f32,
        right: f32,
        q1: f32,
        q2: f32,
        q3: f32,
        q4: f32,
        inner: f32,
        mid: f32,
        outer: f32,
        drift_dx: f32,
        drift_dy: f32,
        temporal_stability: f32,
    ) {
        self.front = front;
        self.back = back;
        self.left = left;
        self.right = right;

        self.q1 = q1;
        self.q2 = q2;
        self.q3 = q3;
        self.q4 = q4;

        self.inner = inner;
        self.mid = mid;
        self.outer = outer;

        self.drift_dx = drift_dx;
        self.drift_dy = drift_dy;

        self.temporal_stability = temporal_stability;

        self.clamp();
        self.denoise();
    }

    // ============================================================
    // 🔥 Tier‑7 Roundabout Routing Additive Logic
    // ============================================================

    /// Compute directional stability for roundabout routing.
    /// High stability + low drift = strong routing candidate.
    pub fn roundabout_direction_score(&self) -> f32 {
        let drift_mag = (self.drift_dx.abs() + self.drift_dy.abs()).min(1.0);
        let drift_penalty = 1.0 / (1.0 + drift_mag * 1.25);

        let spatial =
            (self.front + self.back + self.left + self.right) * 0.25;

        let quadrant =
            (self.q1 + self.q2 + self.q3 + self.q4) * 0.25;

        let radial =
            (self.inner * 0.4) + (self.mid * 0.35) + (self.outer * 0.25);

        let stability = self.stability * 0.65 + self.temporal_stability * 0.35;

        let score =
            stability * 0.45 +
            spatial * 0.20 +
            quadrant * 0.15 +
            radial * 0.20;

        score * drift_penalty
    }

    /// Compute heat‑fusion score for roundabout exit ranking.
    pub fn roundabout_heat_fusion(&self) -> f32 {
        let cognitive =
            self.short_term * 0.55 +
            self.long_term * 0.35 +
            self.resonance * 0.30 +
            self.inertia * 0.20 -
            self.volatility * 0.15;

        let spatial =
            (self.front + self.back + self.left + self.right) * 0.25;

        let drift_mag = (self.drift_dx.abs() + self.drift_dy.abs()).min(1.0);
        let drift_penalty = 1.0 / (1.0 + drift_mag * 0.75);

        let fused = cognitive * 0.6 + spatial * 0.4;
        fused * drift_penalty
    }

    /// Full roundabout score combining direction + heat fusion.
    pub fn roundabout_score(&self) -> f32 {
        let dir = self.roundabout_direction_score();
        let heat = self.roundabout_heat_fusion();
        (dir * 0.55) + (heat * 0.45)
    }
}


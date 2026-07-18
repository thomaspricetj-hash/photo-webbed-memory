use crate::heatmap::HeatLayer;

/// Maximum values to prevent runaway growth
const SHORT_MAX: f32 = 12.0;
const LONG_MAX: f32 = 140.0;

/// Adaptive decay constants
/// Short-term decays fast, long-term decays slow
const BASE_SHORT_DECAY: f32 = 0.055;
const BASE_LONG_DECAY: f32 = 0.006;

/// Promotion scaling
/// Higher values = faster long-term consolidation
const PROMOTION_RATE: f32 = 0.14;

/// Noise floor: values below this are treated as zero
const NOISE_FLOOR: f32 = 0.0004;

/// Stability curve factor
/// Higher values = more resistance to decay for stable nodes
const STABILITY_FACTOR: f32 = 0.18;

/// Importance factor: high-importance nodes decay slower
fn importance_factor(importance: f32) -> f32 {
    // importance ∈ [0, 10]
    // high importance → slower decay
    1.0 / (1.0 + 0.03 * importance)
}

/// Adaptive decay based on access frequency
fn adaptive_decay_factor(access_count: u64) -> f32 {
    let x = access_count as f32;
    1.0 / (1.0 + 0.05 * x)
}

/// Main decay function (Tier‑2)
pub fn decay(
    heat: &mut HeatLayer,
    dt: f32,
    access_count: u64,
    pinned: bool,
    stability: f32,
    importance: f32,
) {
    if pinned {
        // Pinned nodes barely decay
        heat.short_term *= f32::exp(-0.004 * dt);
        heat.long_term *= f32::exp(-0.0008 * dt);
        return;
    }

    let adaptive = adaptive_decay_factor(access_count);
    let importance_mod = importance_factor(importance);

    // Short-term decay (volatile)
    let short_decay = BASE_SHORT_DECAY * adaptive * importance_mod;
    heat.short_term *= f32::exp(-short_decay * dt);

    // Long-term decay (stable)
    let long_decay = BASE_LONG_DECAY * adaptive * importance_mod * (1.0 - STABILITY_FACTOR * stability);
    heat.long_term *= f32::exp(-long_decay * dt);

    // Noise filtering
    if heat.short_term < NOISE_FLOOR {
        heat.short_term = 0.0;
    }
    if heat.long_term < NOISE_FLOOR {
        heat.long_term = 0.0;
    }
}

/// Promotion from short-term → long-term (Tier‑2)
pub fn promote(heat: &mut HeatLayer, stability: f32, importance: f32) {
    // Nonlinear promotion curve
    let nonlinear = heat.short_term.powf(0.85);

    // Stability increases consolidation
    let stability_boost = 1.0 + stability * 0.25;

    // Importance increases consolidation
    let importance_boost = 1.0 + (importance / 10.0) * 0.20;

    let delta = PROMOTION_RATE * nonlinear * stability_boost * importance_boost;

    heat.long_term += delta;

    // Saturation control
    heat.short_term = heat.short_term.min(SHORT_MAX);
    heat.long_term = heat.long_term.min(LONG_MAX);
}

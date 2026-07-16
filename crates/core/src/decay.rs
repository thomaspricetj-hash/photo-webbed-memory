use crate::heatmap::HeatLayer;

/// Maximum values to prevent runaway growth
const SHORT_MAX: f32 = 10.0;
const LONG_MAX: f32 = 100.0;

/// Adaptive decay constants
/// Short-term decays fast, long-term decays slow
const BASE_SHORT_DECAY: f32 = 0.06;
const BASE_LONG_DECAY: f32 = 0.008;

/// Promotion scaling
/// Higher values = faster long-term consolidation
const PROMOTION_RATE: f32 = 0.12;

/// Noise floor: values below this are treated as zero
const NOISE_FLOOR: f32 = 0.0005;

/// Stability curve factor
/// Higher values = more resistance to decay for stable nodes
const STABILITY_FACTOR: f32 = 0.15;

/// Adaptive decay based on access frequency
fn adaptive_decay_factor(access_count: u64) -> f32 {
    // More accesses = slower decay
    // Fewer accesses = faster decay
    let x = access_count as f32;
    1.0 / (1.0 + 0.05 * x)
}

/// Main decay function
pub fn decay(heat: &mut HeatLayer, dt: f32, access_count: u64, pinned: bool) {
    if pinned {
        // Pinned nodes barely decay
        heat.short_term *= f32::exp(-0.005 * dt);
        heat.long_term *= f32::exp(-0.001 * dt);
        return;
    }

    let adaptive = adaptive_decay_factor(access_count);

    // Short-term decay
    let short_decay = BASE_SHORT_DECAY * adaptive;
    heat.short_term *= f32::exp(-short_decay * dt);

    // Long-term decay with stability curve
    let long_decay = BASE_LONG_DECAY * adaptive * (1.0 - STABILITY_FACTOR);
    heat.long_term *= f32::exp(-long_decay * dt);

    // Noise filtering
    if heat.short_term < NOISE_FLOOR {
        heat.short_term = 0.0;
    }
    if heat.long_term < NOISE_FLOOR {
        heat.long_term = 0.0;
    }
}

/// Promotion from short-term → long-term
pub fn promote(heat: &mut HeatLayer) {
    let delta = PROMOTION_RATE * heat.short_term;

    heat.long_term += delta;

    // Saturation control
    if heat.short_term > SHORT_MAX {
        heat.short_term = SHORT_MAX;
    }
    if heat.long_term > LONG_MAX {
        heat.long_term = LONG_MAX;
    }
}

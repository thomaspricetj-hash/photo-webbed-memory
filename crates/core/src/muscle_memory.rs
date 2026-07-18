use std::time::SystemTime;

/// How strongly a pattern is "known" as muscle memory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MuscleStrength {
    pub value: f32,        // 0.0 – 1.0
    pub stability: f32,    // resistance to change
    pub volatility: f32,   // how quickly it adapts to new variations
}

impl MuscleStrength {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            stability: 0.6,
            volatility: 0.4,
        }
    }

    pub fn reinforce(&mut self, amount: f32) {
        let delta = amount * (1.0 - self.value) * (1.0 + self.volatility);
        self.value = (self.value + delta).clamp(0.0, 1.0);
    }

    pub fn decay(&mut self, amount: f32) {
        let delta = amount * self.value * (1.0 + (1.0 - self.stability));
        self.value = (self.value - delta).clamp(0.0, 1.0);
    }

    pub fn is_active(&self, threshold: f32) -> bool {
        self.value >= threshold
    }
}

/// A single procedural pattern: "muscle memory" for a behavior.
#[derive(Debug, Clone)]
pub struct MusclePattern {
    pub id: u64,
    pub label: String,
    pub context_tags: Vec<String>,
    pub signature: Vec<f32>,
    pub strength: MuscleStrength,
    pub last_reinforced: SystemTime,
    pub created_at: SystemTime,
}

impl MusclePattern {
    pub fn new(
        id: u64,
        label: impl Into<String>,
        signature: Vec<f32>,
        context_tags: Vec<String>,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            label: label.into(),
            context_tags,
            signature,
            strength: MuscleStrength::new(),
            last_reinforced: now,
            created_at: now,
        }
    }

    pub fn similarity(&self, other_sig: &[f32]) -> f32 {
        if self.signature.is_empty() || other_sig.is_empty() {
            return 0.0;
        }

        let len = self.signature.len().min(other_sig.len());
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..len {
            let a = self.signature[i];
            let b = other_sig[i];
            dot += a * b;
            norm_a += a * a;
            norm_b += b * b;
        }

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot / (norm_a.sqrt() * norm_b.sqrt())
    }

    pub fn reinforce(&mut self, similarity: f32) {
        let base = 0.1;
        let scaled = base * similarity;
        self.strength.reinforce(scaled);
        self.last_reinforced = SystemTime::now();
    }

    pub fn decay_time_based(&mut self, base_rate: f32, now: SystemTime) {
        if let Ok(elapsed) = now.duration_since(self.last_reinforced) {
            let secs = elapsed.as_secs_f32();
            if secs <= 0.0 {
                return;
            }
            let amount = base_rate * (secs / 60.0);
            self.strength.decay(amount);
        }
    }
}

/// The full muscle-memory store: all procedural patterns.
#[derive(Debug, Default)]
pub struct MuscleMemoryStore {
    patterns: Vec<MusclePattern>,
    next_id: u64,
    pub global_decay_rate: f32,
    pub activation_similarity_threshold: f32,
    pub activation_strength_threshold: f32,
}

/// Borrow‑safe match result (no references)
#[derive(Debug, Clone)]
pub struct MuscleMatchResult {
    pub id: u64,
    pub label: String,
    pub similarity: f32,
    pub is_active: bool,
}

impl MuscleMemoryStore {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            next_id: 1,
            global_decay_rate: 0.02,
            activation_similarity_threshold: 0.75,
            activation_strength_threshold: 0.4,
        }
    }

    pub fn add_pattern(
        &mut self,
        label: impl Into<String>,
        signature: Vec<f32>,
        context_tags: Vec<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut pattern = MusclePattern::new(id, label, signature, context_tags);
        pattern.strength.reinforce(0.25);

        self.patterns.push(pattern);
        id
    }

    /// Borrow‑safe best match (returns ID + data, not references)
    pub fn best_match(&self, signature: &[f32]) -> Option<MuscleMatchResult> {
        let mut best: Option<MuscleMatchResult> = None;

        for pattern in &self.patterns {
            let sim = pattern.similarity(signature);
            if sim < self.activation_similarity_threshold {
                continue;
            }

            let is_active = pattern.strength.is_active(self.activation_strength_threshold);

            match &best {
                None => {
                    best = Some(MuscleMatchResult {
                        id: pattern.id,
                        label: pattern.label.clone(),
                        similarity: sim,
                        is_active,
                    });
                }
                Some(current) => {
                    if sim > current.similarity {
                        best = Some(MuscleMatchResult {
                            id: pattern.id,
                            label: pattern.label.clone(),
                            similarity: sim,
                            is_active,
                        });
                    }
                }
            }
        }

        best
    }

    /// Borrow‑safe process_stimulus
    pub fn process_stimulus(
        &mut self,
        label_hint: Option<&str>,
        signature: Vec<f32>,
        context_tags: Vec<String>,
    ) -> Option<MuscleMatchResult> {

        // Step 1: find best match (immutable borrow only)
        let best = self.best_match(&signature);

        // Step 2: reinforce using ID (mutable borrow happens AFTER immutable borrow ends)
        if let Some(ref result) = best {
            if let Some(p) = self.patterns.iter_mut().find(|p| p.id == result.id) {
                p.reinforce(result.similarity);
            }
            return best;
        }

        // Step 3: no match → create new pattern
        let label = label_hint.unwrap_or("auto_procedural");
        self.add_pattern(label, signature, context_tags);

        None
    }

    pub fn autopilot_maintenance(&mut self) {
        let now = SystemTime::now();

        for pattern in &mut self.patterns {
            pattern.decay_time_based(self.global_decay_rate, now);
        }

        self.patterns.retain(|p| p.strength.value > 0.05);
    }

    pub fn patterns(&self) -> &[MusclePattern] {
        &self.patterns
    }

    pub fn patterns_mut(&mut self) -> &mut [MusclePattern] {
        &mut self.patterns
    }
}

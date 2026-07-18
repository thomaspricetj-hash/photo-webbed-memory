use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct WordStats {
    pub count: u64,
    pub co_occurrence: HashMap<String, u64>,
    pub last_seen: u64,
    pub importance: f32,
}

impl WordStats {
    pub fn new() -> Self {
        Self {
            count: 0,
            co_occurrence: HashMap::new(),
            last_seen: 0,
            importance: 0.0,
        }
    }

    pub fn observe(&mut self, context: &[String], now: u64) {
        self.count += 1;
        self.last_seen = now;

        for w in context {
            *self.co_occurrence.entry(w.to_lowercase()).or_insert(0) += 1;
        }

        let base = (self.count as f32).ln().max(0.0);
        self.importance = (self.importance * 0.9 + base * 0.1).min(10.0);
    }

    pub fn correlation(&self, other: &str) -> f32 {
        let key = other.to_lowercase();
        let co = self.co_occurrence.get(&key).copied().unwrap_or(0) as f32;
        if self.count == 0 {
            0.0
        } else {
            (co / self.count as f32).min(1.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct WordCluster {
    pub id: u32,
    pub label: String,
    pub words: HashSet<String>,
    pub strength: f32,
    pub importance: f32,
}

impl WordCluster {
    pub fn new(id: u32, label: String) -> Self {
        Self {
            id,
            label,
            words: HashSet::new(),
            strength: 0.0,
            importance: 0.0,
        }
    }

    pub fn add_word(&mut self, word: &str, weight: f32, importance: f32) {
        self.words.insert(word.to_lowercase());
        self.strength = (self.strength + weight).min(1.0);
        self.importance = (self.importance * 0.9 + importance * 0.1).min(10.0);
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }
}

#[derive(Debug, Clone)]
pub struct HiveCell {
    pub id: u32,
    pub label: String,
    pub clusters: HashSet<u32>,
    pub strength: f32,
    pub importance: f32,
}

impl HiveCell {
    pub fn new(id: u32, label: String) -> Self {
        Self {
            id,
            label,
            clusters: HashSet::new(),
            strength: 0.0,
            importance: 0.0,
        }
    }

    pub fn add_cluster(&mut self, cluster_id: u32, weight: f32, importance: f32) {
        self.clusters.insert(cluster_id);
        self.strength = (self.strength + weight).min(1.0);
        self.importance = (self.importance * 0.9 + importance * 0.1).min(10.0);
    }
}

#[derive(Debug, Clone)]
pub struct WordHive {
    pub stats: HashMap<String, WordStats>,
    pub word_to_cluster: HashMap<String, u32>,
    pub clusters: HashMap<u32, WordCluster>,
    pub hive_cells: HashMap<u32, HiveCell>,
    next_cluster_id: u32,
    next_hive_id: u32,
    pub cluster_threshold: f32,
    pub hive_threshold: f32,
    pub importance_threshold: f32,
}

impl WordHive {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            word_to_cluster: HashMap::new(),
            clusters: HashMap::new(),
            hive_cells: HashMap::new(),
            next_cluster_id: 1,
            next_hive_id: 1,
            cluster_threshold: 0.35,
            hive_threshold: 0.45,
            importance_threshold: 0.5,
        }
    }

    pub fn observe_word(&mut self, word: &str, context: &[String]) {
        let key = word.to_lowercase();
        let now = 0;

        let stats = self.stats.entry(key.clone()).or_insert_with(WordStats::new);
        stats.observe(context, now);
    }

    pub fn integrate_word(&mut self, word: &str) {
        let key = word.to_lowercase();

        let stats = match self.stats.get(&key) {
            Some(s) => s,
            None => return,
        };

        if stats.importance < self.importance_threshold {
            return;
        }

        let mut best_cluster = None;
        let mut best_score = 0.0;

        for (cid, cluster) in self.clusters.iter() {
            let mut score = 0.0;
            for w in cluster.words.iter() {
                score += stats.correlation(w);
            }

            if score > best_score {
                best_score = score;
                best_cluster = Some(*cid);
            }
        }

        if let Some(cid) = best_cluster {
            let cluster = self.clusters.get_mut(&cid).unwrap();
            cluster.add_word(&key, best_score, stats.importance);
            self.word_to_cluster.insert(key.clone(), cid);
        } else {
            let cid = self.next_cluster_id;
            self.next_cluster_id += 1;

            let mut cluster = WordCluster::new(cid, key.clone());
            cluster.add_word(&key, 1.0, stats.importance);

            self.clusters.insert(cid, cluster);
            self.word_to_cluster.insert(key.clone(), cid);
        }
    }

    pub fn bias_propagation(&self, word: &str) -> Option<String> {
        let key = word.to_lowercase();
        let cid = self.word_to_cluster.get(&key)?;

        let cluster = self.clusters.get(cid)?;
        let mut best_word = None;
        let mut best_score = 0.0;

        for w in cluster.words.iter() {
            if w == &key {
                continue;
            }

            let stats = self.stats.get(w)?;
            let score = stats.importance;

            if score > best_score {
                best_score = score;
                best_word = Some(w.clone());
            }
        }

        best_word
    }

    // ============================================================
    // FIXED: Deterministic cluster rebuild using next_cluster_id
    // ============================================================
    pub fn rebuild_clusters(&mut self) {
        let mut new_clusters = HashMap::new();

        for (word, stats) in self.stats.iter() {
            if stats.importance < self.importance_threshold {
                continue;
            }

            let cid = self.next_cluster_id;
            self.next_cluster_id += 1;

            let mut cluster = WordCluster::new(cid, word.clone());
            cluster.add_word(word, 1.0, stats.importance);

            new_clusters.insert(cid, cluster);
        }

        self.clusters = new_clusters;
    }

    // ============================================================
    // FIXED: Deterministic hive rebuild using next_hive_id
    // ============================================================
    pub fn rebuild_hive(&mut self) {
        let mut new_hive = HashMap::new();

        for (cid, cluster) in self.clusters.iter() {
            let hid = self.next_hive_id;
            self.next_hive_id += 1;

            let mut cell = HiveCell::new(hid, cluster.label.clone());
            cell.add_cluster(*cid, cluster.strength, cluster.importance);

            new_hive.insert(hid, cell);
        }

        self.hive_cells = new_hive;
    }

    pub fn generalize_word(&self, word: &str) -> String {
        word.to_lowercase()
    }
}

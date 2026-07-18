//! Photo Webbed Core

pub mod graph;
pub mod node;
pub mod edge;

pub mod heatmap;
pub mod scratchpad;
pub mod decay;
pub mod engine;

pub mod memory_predictor;
pub mod memory_linking;
pub mod memory_reuse;
pub mod photonic;
pub mod memory_cognition;
pub mod semantic_scene;
pub mod memory_index;
pub mod reflex;
pub mod memory_lock_north_star;
pub mod muscle_memory;

// ⭐ ONLY ONE declaration
pub mod word_hive;

// ------------------------------------------------------------
// Re-exports
// ------------------------------------------------------------

pub use graph::Graph;
pub use node::{Node, NodeId, NodeKind};
pub use edge::{Edge, EdgeId, EdgeKind};

pub use heatmap::HeatLayer;
pub use scratchpad::Scratchpad;
pub use engine::{MemoryEngine, NodeState};

pub use memory_predictor::MemoryPredictor;
pub use memory_linking::MemoryLinker;
pub use memory_reuse::MemoryReuseEngine;
pub use memory_index::MemoryIndex;

pub use photonic::PhotonicPropagationEngine;

pub use memory_cognition::{
    MemoryConsolidationEngine,
    MemoryDriftEngine,
    MemoryClusteringEngine,
    MemoryCognitionEngine,
};

pub use reflex::{ReflexSystem, ReflexEntry};

// ⭐ ONLY ONE re-export
pub use word_hive::{WordHive, WordCluster, HiveCell};

// ------------------------------------------------------------
// Prelude
// ------------------------------------------------------------

pub mod prelude {
    pub use crate::graph::Graph;
    pub use crate::node::{Node, NodeId, NodeKind};
    pub use crate::edge::{Edge, EdgeId, EdgeKind};

    pub use crate::heatmap::HeatLayer;
    pub use crate::scratchpad::Scratchpad;
    pub use crate::engine::{MemoryEngine, NodeState};

    pub use crate::memory_predictor::MemoryPredictor;
    pub use crate::memory_linking::MemoryLinker;
    pub use crate::memory_reuse::MemoryReuseEngine;
    pub use crate::memory_index::MemoryIndex;

    pub use crate::photonic::PhotonicPropagationEngine;

    pub use crate::memory_cognition::{
        MemoryConsolidationEngine,
        MemoryDriftEngine,
        MemoryClusteringEngine,
        MemoryCognitionEngine,
    };

    pub use crate::reflex::{ReflexSystem, ReflexEntry};
    pub use crate::word_hive::{WordHive, WordCluster, HiveCell};
}



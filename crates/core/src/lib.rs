//! Photo Webbed Core
//!
//! Cognitive graph + dual-layer heatmap + scratchpad memory engine.
//! This crate provides the core primitives for building a 3D memory graph
//! with short-term / long-term activation, stability, volatility,
//! edge reinforcement/decay, prediction, linking, memory reuse,
//! photonic propagation, consolidation, drift, clustering,
//! and full cognition-cycle orchestration.

//
// Core graph structures
//
pub mod graph;
pub mod node;
pub mod edge;

//
// Core cognitive layers
//
pub mod heatmap;
pub mod scratchpad;
pub mod decay;
pub mod engine;

//
// Cognitive subsystems
//
pub mod memory_predictor;
pub mod memory_linking;
pub mod memory_reuse;
pub mod photonic;
pub mod memory_cognition;
pub mod semantic_scene;
pub mod memory_index;
pub mod reflex;

//
// Re-exports for external users
//
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

//
// Prelude for easy importing
//
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

    pub use crate::photonic::PhotonicPropagationEngine;

    pub use crate::memory_cognition::{
        MemoryConsolidationEngine,
        MemoryDriftEngine,
        MemoryClusteringEngine,
        MemoryCognitionEngine,
    };
}


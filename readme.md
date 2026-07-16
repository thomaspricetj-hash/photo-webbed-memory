Photo-Webbed Core
Synthetic Memory Graph + Cognitive Dynamics Engine

Photo-Webbed Core is a Rust-based cognitive memory engine built around a dynamic 3D graph, dual-layer heatmaps, semantic encoding, episodic memory, photonic propagation, fractal echo reinforcement, and adaptive long-term consolidation. It provides a foundation for synthetic cognition, emergent reasoning, and memory-driven agents.

Features

Cognitive Memory Architecture

Dual-layer heatmaps (short-term and long-term)

Node stability and volatility

Scratchpad working-memory lanes

Semantic decay, promotion, and reinforcement

Episodic memory storage with contextual tagging

Dynamic Graph Engine

Spatial nodes with 3D positions

Edges with reinforcement, decay, and pruning

Automatic link formation via cognition subsystems

Concept nodes formed from semantic scenes

Photonic Propagation Engine

Wave-based activation propagation

Interference modeling

Resonance-driven memory boosting

Hybrid propagation across the cognitive graph

Semantic and Episodic Memory

Text-to-scene semantic encoding

Scene graphs with actors, actions, objects, and events

Context extraction (location, time-of-day, mood, tags)

Meaning-based compression

Episodic memory storage and retrieval

Temporal linking (before, after, causal)

Memory Cognition Subsystems

Consolidation Engine (stabilizes resonant nodes)

Drift Engine (models forgetting and concept drift)

Clustering Engine (forms semantic groups)

Fractal Echo Engine (reinforces repeating activation rhythms)

Cognition Cycle Engine (orchestrates full memory dynamics)

Full Cognitive Cycle
Activation -> Propagation -> Interference -> Resonance -> Consolidation -> Drift -> Clustering -> Echo -> Semantic Encoding -> Episodic Storage -> Recall

Installation

cargo add photo-webbed-core

Or add to Cargo.toml:

photo-webbed-core = "0.1"

Usage

Basic Setup

use photo_webbed_core::prelude::*;

fn main() {
let mut engine = MemoryEngine::new();
let now = 0;

// Create nodes
let a = engine.add_node("cat", NodeKind::Concept);
let b = engine.add_node("animal", NodeKind::Concept);

// Link nodes
engine.link(a, b, EdgeKind::Associative, 1.0);

// Activate nodes
engine.activate_main(a, now);
engine.activate_main(b, now + 1);

// Run decay tick
engine.decay_tick(now + 10);

// Export view of memory state
let view = engine.export_view();
println!("Memory view: {:?}", view);
}

Using Semantic and Episodic Memory

fn main() {
let mut engine = MemoryEngine::new();
let now = 100;

// Ingest a semantic scene
let scene_id = engine.ingest_text_scene(
"The man walks in the park at night",
now,
);

println!("Stored scene id: {}", scene_id);

// Recall recent scenes
let recent = engine.semantic.recall_recent(5);
println!("Recent scenes: {:?}", recent);

// Recall by keyword
let night_scenes = engine.semantic.recall_by_keyword("night");
println!("Night scenes: {:?}", night_scenes);
}

Full Cognitive Cycle Example

fn main() {
let mut engine = MemoryEngine::new();
let now = 0;

let n1 = engine.add_node("idea", NodeKind::Concept);
let n2 = engine.add_node("memory", NodeKind::Concept);

engine.link(n1, n2, EdgeKind::Associative, 0.8);

// Activate repeatedly to build stability
for t in 0..50 {
engine.activate_main(n1, now + t);
engine.activate_main(n2, now + t);
engine.decay_tick(now + t);
}

let view = engine.export_view();
println!("Final memory state: {:?}", view);
}

Scene to Concept Integration Example

fn main() {
let mut engine = MemoryEngine::new();
let now = 500;

engine.ingest_text_scene(
"A woman sits quietly in the office during the morning meeting",
now,
);

// Concepts created from scene labels
for (id, node) in engine.graph.nodes.iter() {
println!("Node {} -> {}", id, node.label);
}
}


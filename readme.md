Photo‑Webbed Core
Synthetic Memory Graph + Cognitive Dynamics Engine
Photo‑Webbed Core is a Rust‑based cognitive memory engine built around a dynamic 3D graph, dual‑layer heatmaps, semantic encoding, episodic memory, summary‑based semantic anchoring, photonic propagation, fractal echo reinforcement, and adaptive long‑term consolidation. It provides a foundation for synthetic cognition, emergent reasoning, and memory‑driven agents.

Features
🧠 Cognitive Memory Architecture
Dual‑layer heatmaps (short‑term + long‑term)

Node stability, volatility, and decay

Scratchpad working‑memory lanes

Semantic decay, promotion, and reinforcement

Episodic memory storage with contextual tagging

Summary nodes as long‑term semantic anchors

🔗 Dynamic Graph Engine
Spatial nodes with 3D positions

Edges with reinforcement, decay, pruning

Automatic link formation via cognition subsystems

Concept nodes formed from semantic scenes

Summary nodes linked to high‑salience entities

🌐 Photonic Propagation Engine
Wave‑based activation propagation

Interference modeling

Resonance‑driven memory boosting

Hybrid propagation across the cognitive graph

Summary nodes participate in propagation

🧩 Semantic & Episodic Memory
Text‑to‑scene semantic encoding

Scene graphs with actors, actions, objects, events

Context extraction (location, time‑of‑day, mood, tags)

Meaning‑based compression

Episodic memory storage + retrieval

Temporal linking (before / after / causal)

Summaries stored and used as cognitive nodes

🔁 Memory Cognition Subsystems
Consolidation Engine — stabilizes resonant nodes

Drift Engine — models forgetting and concept drift

Clustering Engine — forms semantic groups

Fractal Echo Engine — reinforces repeating activation rhythms

Cognition Cycle Engine — orchestrates full memory dynamics

🔄 Full Cognitive Cycle
Activation → Propagation → Interference → Resonance
→ Consolidation → Drift → Clustering → Echo
→ Semantic Encoding → Episodic Storage → Summary Anchoring → Recall

Installation
bash
cargo add photo-webbed-core
Or add to your Cargo.toml:

toml
photo-webbed-core = "0.1"
Usage
Basic Setup
rust
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
Using Semantic, Episodic, and Summary Memory
rust
fn main() {
    let mut engine = MemoryEngine::new();
    let now = 100;

    // Ingest a semantic scene (creates nodes + summary node)
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
rust
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
Scene → Concept Integration Example
rust
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
Summary‑Based Memory (New Feature)
Photo‑Webbed Core now includes summary nodes, automatically generated from each ingested scene:

Summaries are compressed meaning representations

Stored as NodeKind::Summary

Linked to high‑salience entities

Activated strongly for long‑term stability

Participate in photonic propagation

Improve recall, clustering, and concept drift modeling

This gives the engine human‑like gist‑based memory, where meaning becomes the anchor for long‑term recall.


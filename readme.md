📘 Photo‑Webbed Core
Synthetic Memory Graph + Cognitive Dynamics Engine
Photo‑Webbed Core is a Rust‑based cognitive memory engine built around a dynamic 3D graph, dual‑layer heatmaps, semantic encoding, episodic memory, summary‑anchored meaning nodes, reflex‑driven fast‑path activation, photonic propagation, fractal echo reinforcement, and adaptive long‑term consolidation. It provides a foundation for synthetic cognition, emergent reasoning, and memory‑driven agents.

🚀 Features
🧠 Cognitive Memory Architecture
Dual‑layer heatmaps (short‑term + long‑term)

Node stability, volatility, and decay

Scratchpad working‑memory lanes

Semantic decay, promotion, and reinforcement

Episodic memory with contextual tagging

Summary nodes as long‑term semantic anchors

Reflex subsystem for instant stimulus → response activation

Reflex learning (Hebbian‑style)

Reflex decay (time‑based weakening)

Reflex‑biased propagation and stability shaping

🔗 Dynamic Graph Engine
Spatial nodes with 3D positions

Edges with reinforcement, decay, and pruning

Automatic link formation via cognition subsystems

Concept nodes derived from semantic scenes

Summary nodes linked to high‑salience entities

Reflex nodes become high‑stability anchors

🌐 Photonic Propagation Engine
Wave‑based activation propagation

Interference modeling

Resonance‑driven memory boosting

Hybrid propagation across the cognitive graph

Summary nodes participate in propagation cycles

Reflex nodes bias amplitude and resonance

🧩 Semantic & Episodic Memory
Text‑to‑scene semantic encoding

Scene graphs with actors, actions, objects, events

Context extraction (location, time‑of‑day, mood, tags)

Episodic memory storage + retrieval

Temporal linking (before / after / causal)

Summary‑anchored semantic nodes for long‑term recall

Reflex events optionally link into episodic traces

⚡ Reflex Memory System (New)
Instant stimulus → response activation

Reflex strength modeling (0–1)

Reflex heat + stability shaping

Reflex usage tracking

Reflex learning (strength increases with use)

Reflex decay (weakens if unused)

Reflex generalization hooks (e.g., “dog” → “animal”)

Reflex‑biased photonic propagation

Reflex nodes become long‑term stability anchors

This subsystem provides biologically inspired fast‑path cognition, enabling rapid associative recall and meaning‑anchored activation.

🔁 Memory Cognition Subsystems
Consolidation Engine — stabilizes resonant nodes

Drift Engine — models forgetting and concept drift

Clustering Engine — forms semantic groups

Fractal Echo Engine — reinforces repeating activation rhythms

Cognition Cycle Engine — orchestrates full memory dynamics

Reflex Engine — fast‑path activation + learning + decay

🔄 Full Cognitive Cycle (Updated)
Code
Stimulus
→ Reflex Activation (fast path)
→ Propagation
→ Interference
→ Resonance
→ Consolidation
→ Drift
→ Clustering
→ Echo
→ Semantic Encoding
→ Episodic Storage
→ Summary Anchoring
→ Recall
Reflex activation now occurs before propagation, biasing the entire cognitive cycle.

📦 Installation
bash
cargo add photo-webbed-core
Or add to your Cargo.toml:

toml
photo-webbed-core = "0.1"
🛠 Usage Examples
Basic Setup
rust
use photo_webbed_core::prelude::*;

fn main() {
    let mut engine = MemoryEngine::new();
    let now = 0;

    let a = engine.add_node("cat", NodeKind::Concept);
    let b = engine.add_node("animal", NodeKind::Concept);

    engine.link(a, b, EdgeKind::Associative, 1.0);

    engine.activate_main(a, now);
    engine.activate_main(b, now + 1);

    engine.decay_tick(now + 10);

    println!("{:?}", engine.export_view());
}
Semantic + Episodic Memory
rust
fn main() {
    let mut engine = MemoryEngine::new();
    let now = 100;

    let scene_id = engine.ingest_text_scene(
        "The man walks in the park at night",
        now,
    );

    println!("Scene id: {}", scene_id);

    println!("Recent scenes: {:?}", engine.semantic.recall_recent(5));
    println!("Night scenes: {:?}", engine.semantic.recall_by_keyword("night"));
}
Full Cognitive Cycle
rust
fn main() {
    let mut engine = MemoryEngine::new();
    let now = 0;

    let n1 = engine.add_node("idea", NodeKind::Concept);
    let n2 = engine.add_node("memory", NodeKind::Concept);

    engine.link(n1, n2, EdgeKind::Associative, 0.8);

    for t in 0..50 {
        engine.activate_main(n1, now + t);
        engine.activate_main(n2, now + t);
        engine.decay_tick(now + t);
    }

    println!("{:?}", engine.export_view());
}
Scene → Concept Integration
rust
fn main() {
    let mut engine = MemoryEngine::new();
    let now = 500;

    engine.ingest_text_scene(
        "A woman sits quietly in the office during the morning meeting",
        now,
    );

    for (id, node) in engine.graph.nodes.iter() {
        println!("Node {} -> {}", id.0, node.label);
    }
}
📘 Summary‑Based Memory
Each ingested scene generates a summary node:

compressed meaning representation

stored as NodeKind::Summary

linked to high‑salience entities

strongly activated for long‑term stability

participates in photonic propagation

improves recall, clustering, and concept drift modeling

This provides human‑like gist‑based memory, where meaning becomes the anchor for long‑term recall.

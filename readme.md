📘 Photo‑Webbed Core
Synthetic Memory Graph + Cognitive Dynamics Engine
Photo‑Webbed Core is a Rust‑based cognitive memory engine built around a dynamic 3D graph, dual‑layer heatmaps, semantic encoding, episodic memory, reflex‑driven activation, photonic propagation, fractal echo reinforcement, procedural muscle‑memory learning, and adaptive long‑term consolidation.

It provides a foundation for synthetic cognition, emergent reasoning, and memory‑driven agents.

🚀 Features
🧠 Cognitive Memory Architecture
Dual‑layer heatmaps for short‑term and long‑term activation

Node physics: stability, volatility, resonance, inertia

Scratchpad working‑memory lanes: main, reflex, hive‑bias, scene

Semantic decay & promotion via decay() + promote()

Episodic memory with contextual tagging + compressed summaries

Summary nodes as long‑term semantic anchors

Reflex subsystem for instant stimulus → response activation

Reflex learning (Hebbian‑style strengthening)

Reflex decay when unused

Reflex‑biased propagation shaping stability + importance

⚡ NEW: Procedural Muscle‑Memory System (MAX‑Tier)
A full procedural learning subsystem modeled after biological muscle memory.

Capabilities
Learns repeated activation patterns

Stores compressed signatures of behavior

Reinforces patterns based on similarity

Decays unused procedural traces

Prunes dead routines

Context‑tagged procedural learning (lane‑aware)

Integrates with reflex, semantic, and photonic layers

Autopilot‑driven reinforcement and decay

Enables fast‑path procedural activation

Impact
Faster reaction loops

More stable behavior

Automatic learned routines

Reduced cognitive load on semantic engine

Higher consistency across repeated tasks

True procedural cognition

This subsystem elevates Photo‑Webbed Core into a full cognitive stack.

🔗 Dynamic Graph Engine
Spatial nodes with 3D cognitive geometry

Edges with lifecycle: reinforcement, decay, pruning

Automatic link formation via cognition subsystems

Concept nodes derived from semantic scenes

Summary nodes linked to high‑salience entities

Reflex nodes promoted to long‑term stability anchors

🌐 Photonic Propagation Engine
Wave‑based activation propagation

Interference modeling across the graph

Resonance‑driven memory boosting

Hybrid propagation integrating heat, reflex, and semantic layers

Summary nodes participate in propagation cycles

Reflex nodes bias amplitude and resonance

🧩 Semantic & Episodic Memory
Text‑to‑scene semantic encoding

Scene graphs: actors, actions, objects, events

Context extraction: location, time‑of‑day, mood, tags

Episodic storage + retrieval

Temporal linking: before / after / causal relations

Summary‑anchored semantic nodes for long‑term recall

Reflex events optionally linked into episodic traces

⚡ Reflex Memory System
Instant stimulus → response activation

Reflex strength modeling (0.0–1.0)

Reflex heat + stability shaping

Reflex usage tracking

Reflex learning (strength increases with repeated activation)

Reflex decay when unused

Reflex generalization hooks (“dog” → “animal”)

Reflex‑biased photonic propagation

Reflex nodes promoted to long‑term anchors

Provides biologically inspired fast‑path cognition.

🐝 Word Hive Semantic Engine
Word statistics: frequency, co‑occurrence, importance

Semantic clusters (WordCluster)

Hive cells (HiveCell) for higher‑order semantic hubs

Bias propagation: bias_propagation(&label)

Generalization: generalize_word(&summary)

Rebuild cycles integrated into decay ticks

Provides semantic correlation, cluster‑level reasoning, and hive‑biased activation.

🔁 Memory Cognition Subsystems
Consolidation Engine: stabilizes resonant nodes

Drift Engine: models forgetting + concept drift

Clustering Engine: forms semantic groups

Fractal Echo Engine: reinforces repeating activation rhythms

Cognition Cycle Engine: orchestrates full memory dynamics

Reflex Engine: fast‑path activation + learning + decay

Procedural Engine (Muscle Memory): learns repeated activation patterns

🔄 Full Cognitive Cycle
Code
Stimulus
→ Reflex Activation (fast path)
→ Hive Bias Propagation
→ Procedural Muscle Memory Activation (new)
→ Graph Activation
→ Photonic Propagation
→ Interference
→ Resonance
→ Consolidation
→ Drift
→ Clustering
→ Fractal Echo
→ Semantic Encoding
→ Episodic Storage
→ Summary Anchoring
→ Recall
Reflex + hive bias + procedural memory now occur before normal activation, biasing the entire cycle toward meaning‑anchored, behavior‑anchored, semantically coherent recall.

🧠 Autopilot System (v1 + v2)
Autopilot v1
Importance rebalance

High‑value reinforcement

Soft pruning of low‑value nodes

Hive rebuild

Index rebuild

Autopilot v2
Predictive reinforcement (anticipatory cognition)

Adaptive decay curves (biological forgetting model)

Cluster‑level resonance propagation

Procedural reinforcement + decay (new)

Autopilot v2 gives the engine self‑healing, self‑optimizing, and self‑stabilizing behavior.

📦 Installation
Add to your Cargo.toml:

toml
photo-webbed-core = "0.1"
Or install via Cargo:

bash
cargo add photo-webbed-core
🛠 Usage Examples
Basic Setup
rust
use photo_webbed_core::prelude::*;

fn main() {
    let mut engine = MemoryEngine::new();
    let now = 0;

    let cat = engine.add_node("cat", NodeKind::Concept);
    let animal = engine.add_node("animal", NodeKind::Concept);

    engine.link(cat, animal, EdgeKind::Associative, 1.0);

    engine.activate_main(cat, now);
    engine.activate_main(animal, now + 1);

    engine.decay_tick(now + 10);

    println!("{:?}", engine.export_view());
}
Semantic + Episodic Memory
rust
use photo_webbed_core::prelude::*;

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
use photo_webbed_core::prelude::*;

fn main() {
    let mut engine = MemoryEngine::new();
    let now = 0;

    let idea = engine.add_node("idea", NodeKind::Concept);
    let memory = engine.add_node("memory", NodeKind::Concept);

    engine.link(idea, memory, EdgeKind::Associative, 0.8);

    for t in 0..50 {
        engine.activate_main(idea, now + t);
        engine.activate_main(memory, now + t);
        engine.decay_tick(now + t);
    }

    println!("{:?}", engine.export_view());
}
Scene → Concept Integration
rust
use photo_webbed_core::prelude::*;

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

This provides human‑like gist‑based memory, where meaning becomes the anchor for long‑term 
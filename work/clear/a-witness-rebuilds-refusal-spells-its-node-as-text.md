---
id: a-witness-rebuilds-refusal-spells-its-node-as-text
kind: issue
title: clearance: the witness rebuild's refusal spells the node it has no value for as text, so the mention is lost
status: open
opened: 2026-11-23
priority: P3
cost: C
refs: [some-spoken-causes-and-reports-are-still-text]
---


Found by the OFFER sweep for spoken text that drops its mentions (`some-spoken-causes-and-reports-are-still-text`).

`verify_witness` (`crates/editor-core/src/clearance.rs`) rebuilds the violating pair at `f64` and, where the rebuild has no value for a face's node, writes `format!("the f64 rebuild has no value for a face of the violating pair: {standing}")` into its `Err(String)`. The `NodeStanding` is spelled by tag there, outside any sentence (so the flattening guard does not see it), and the text travels as `EngineRefusal::WitnessUnverified { what: String }` (`crates/topo/src/clearance.rs`), a topo type that cannot hold a kernel `Sentence`. The refusal a clearance then says names the node as words: the viewer cannot draw it as a link, and it is said by tag rather than as the document names it.

The neighbouring `SelectionRefusal::no_body` arm of the same closure (`framed_at`) is spelled the same way.

**The fix** wants the node carried beside the topo refusal (the kernel's `ClearanceRefusal` arm that wraps `EngineRefusal` can hold the standing, and say it as a piece), not a `Sentence` inside topo. The arm is said to be unreachable in practice (`measure-clearance-refused-carries-arms-its-door-never-raises`), so this is P3.

---
id: edge-classes-mixes-decided-and-measured-rows
kind: issue
title: BooleanReduction::edge_classes mixes rows the boolean decided (vertex-on-face) with rows it only measured (vertex-vertex), and nothing in the type says which
status: open
opened: 2026-11-01
priority: P3
cost: M
---


Found by the designers weighing `the-booleans-cone-reader-is-a-second-link` (2026-11-01).

`crates/topo/src/boolean/mod.rs` `EdgePieceClass` docs say the vertex-vertex rows are "a measurement of their own … not a record of what it decided", while the vertex-on-face rows are rows the boolean acted on (D5's birth rows). Both go into one `BooleanReduction::edge_classes` list, and nothing in the type says which a row is, so a reader (naming, `emit_topo::senses_at`, `emit_union`) cannot tell a decision from a measurement. Owed: the row's provenance in its type, or a reason none is needed.

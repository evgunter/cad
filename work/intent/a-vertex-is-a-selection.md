---
id: a-vertex-is-a-selection
kind: issue
title: Stage 2 FORK-VTX: a vertex is a selection; a measure reads it, a pose reader reads its point (D10)
status: dispatched
opened: 2026-10-10
priority: P0
cost: E
refs: [select-defines-face-and-edge-variables, a-selection-is-a-definition-of-a-body-s-faces-or-edges]
branch: intent/s2-e-select
---


Raised as FORK-VTX by stage 2 E (`select-defines-face-and-edge-variables`). Measure references resolve to vertices today (`eval/measure.rs` `carrier_of`'s `Carrier::Point`, and `distance`'s point–point and point–plane arms), but D10's kinds sentence listed only `Face`, `Edge`, `Faces` and `Edges`. A designer pair converged in one round; the question and both reports' substance are in the `[ev]` PR titled "a vertex is a selection; a point is read off it". Ev approved it (PR 4505); stage 2 E builds it on `intent/s2-e-select` and closes this item.

**What is decided.**

- `Vertex` is a selection kind beside `Face` and `Edge`: one cell of a body, named by a `StableName` and resolved by the select's N5 ladder. There is no `Vertices` set until a slot reads one.
- A measure reads the cell, the `Vertex` selection, as it reads a `Face` or an `Edge`. It does not read a pose, because its primitives judge cells with extent and material (`reach`, `gap`'s outward normal, `min_clearance`), which a pose lacks.
- A pose reader (stage 3's point mate, or any other) reads `Point { of: v }` off the vertex, as a face reads as a plane. One selection serves both readers, with one resolution, one N5 diagnosis and one `Rebind`.
- A selection's kind is fixed at minting, so each measure primitive's admitted kinds are checked at the edit and load doors as an ordinary `SlotVarKind` refusal: `distance` Face, Edge, Vertex; `angle` Face, Edge; `min_clearance` Body, Face; `gap` Face. The carrier-class refusals stay at evaluation.

**How stage 2 E builds it.**

- `VarKind::Vertex` beside `Face` and `Edge`; `Select { body, names }` whose names denote vertices defines one. The ladder (`wire.rs` `mod ladder`, `named_entity`) is kind-agnostic and needs no vertex rung; a vertex name in a `Face` select refuses `SelectKind { expected, found }`.
- A stored vertex measure ref migrates like a face ref: one `Vertex` select on that body plus a read. No committed golden or fixture holds a vertex ref.
- Python's `MeasurePrimitive.distance((body, v), …)` is unchanged at the call site: the tuple sugar lowers to one select per operand, its kind taken from the name.
- `wire_entity_door.rs`'s vertex rows ("resolves to a vertex") become door refusals; `MeasureSelectionKind`'s kind half and `scope_of` as `min_clearance`'s kind check go.
- The viewer's "picks become select edits" gains a `Vertex` arm (`frame.rs` maps a vertex pick to the node today).
- Stage 3 A's `Point { of: Face | Edge | Vertex }` is the pose half; nothing in E builds it.

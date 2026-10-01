---
id: viewer-refusals-speak-the-node
kind: unit
title: The viewer's own refusal types speak the node with its label, built where the frame owns the document
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
---


Split from `refusal-values-speak-the-node-with-its-label` (its PR spoke `EditError` and the `NodeError` headline, which the feature tree's failed rows and carried lines now draw through `NodeError::spoken` and `CarriedLevel::line_in`). The rule is DESIGN.md Band 1, "Node labels": where the frame owns the document, the refusal holds the spoken node, built at the raise.

## The types (from the parent row, with the sweep's hits)

- `MateToolError` (`matetool.rs`, `NotAnInstancePick`, `SamePick`; its `Display` near `face_of`).
- `HeldRefusal` (`sketch.rs`).
- `SlotUnitFault` (`props.rs`).
- `Refusal` and `FaceFrameFault` (`session/refuse.rs`).
- `DuplicateFault` (`combine.rs`).
- `AdmissionFault` (`display.rs`).
- Also hit by the same sweep: `blend.rs` and `pickindex.rs` (`Display`s), `session.rs` (`set_program_of`), `drafts.rs` (`profile_edit`).

The sweep matched `"…node {…}"` in non-test `src/` at that PR's merge base. A second pass for a node printed under another noun closes the gap it cannot see.

---
id: refusal-values-speak-the-node-with-its-label
kind: unit
title: Refusal values speak the node as the document holds it: kind, label and tag, built at the raise site
status: open
opened: 2026-10-01
priority: P1
cost: H
parent: node-labels-are-document-data
---

The second half of `node-labels-are-document-data`, split from it at PR 3713: that PR landed the label (data, edit, save, pin, split/inline, `SpokenNode`'s `Extrude "base plate" (3fa9c1d2a0b1)`, Python, the viewer's rows, rename and create forms). What it did not do is give the label to the refusal VALUES that name a node, which still hold a bare `RecipeNodeId` and print `node <tag>` with no kind and no label.

## Scope (DESIGN.md Band 1, "Node labels"; ruled on PR 3565)

- **Kernel refusals that name a node** hold the spoken node, built at the raise site from the document the door holds: `EditError` (every arm with a `node`/`id`/`input`/`at` field — `UnknownNode` keeps the absent spelling, `node <tag>`), `RangeRefusal`, `MateFault`, `AssemblyError`, `PartFault`, `NodeStanding`, and the rest of that sweep. The typed field keeps the full id (`SpokenNode::id`), so a caller's match and the Python payload's `node` are unchanged.
- **The viewer's own refusal types** do the same where the frame owns the document: `MateToolError` (`matetool.rs`, `NotAnInstancePick`, `SamePick`), `HeldRefusal` (`sketch.rs`), `SlotUnitFault` (`props.rs`), `Refusal` and `FaceFrameFault` (`session/refuse.rs`), `DuplicateFault` (`combine.rs`), `AdmissionFault` (`display.rs`).
- **The memoized `NodeError` keeps a bare id** (`node <tag> failed: …`), because the evaluation memo reuses it and a label captured there would go stale on a rename; its node frame speaks it when the evaluation is handed out.
- `SpokenNode` is `Clone`, not `Copy` (it carries the label), so a variant that holds one stops being `Copy`.

## Why split

About forty variants change type, each with its `Display`, its Python payload projection (`edit_payload.rs`, `tags.rs`) and the display-contract and refusal-concision rows that pin its text; doing it inside PR 3713 would have doubled a diff that already crosses editor-core, pncad-py and the viewer.

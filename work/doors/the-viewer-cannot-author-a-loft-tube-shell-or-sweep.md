---
id: the-viewer-cannot-author-a-loft-tube-shell-or-sweep
kind: issue
title: viewer: loft, tube, hollow tube, shell and sweep have no create path in the viewer
status: open
opened: 2026-11-22
priority: P1
cost: H
---

**Ev reported this** (in chat, 2026-11-22): "there seems to be no way to create lofts in the gui (and probably other verbs too)".

The kernel's `Node` has `Tube`, `HollowTube`, `Loft`, `Sweep` and `Shell` (`crates/editor-core/src/node.rs`, ~1651, ~1685, ~1723, ~1748, ~1836). Tube, hollow tube, loft and shell are bound in Python (`crates/pncad-py/src/py/doc.rs:2950`, `:2992`, `:3040`, `:3200`). No sweep constructor turned up in `py/doc.rs`; whoever takes this row should confirm whether sweep has any API door at all. Measure and the non-clearance assertions are in Python too (`py/doc.rs:1690`, `:3600`, `:3634`); assertions are filed separately as `a-check-offers-no-way-to-add-the-assertion-it-asks-for`.

The viewer's create vocabulary (`crates/viewer/src/session/op.rs`: `AddProfile` 612, `AddExtrude` 721, `AddRevolve` 733, `AddBoolean` 777, `AddSplit` 804, `AddMirror` 828, `AddClearance` 839, `AddPattern` 863, `AddFusedPattern` 894, `AddFillet` 934, `AddChamfer` 953, …) and its tools (`crates/viewer/src/tools.rs:59-82`) have no arm for any of these. The viewer only classifies these nodes when a document already holds them (`crates/viewer/src/session/refuse.rs:134-140`, `crates/viewer/src/session.rs:6371-6377`).

This is the same shape as the closed `work/doors/the-viewer-cannot-author-a-mirror.md` (PR 5683), so that row's create-commits-then-edits pattern (FORMS, merged into FORMWORDS) is the template. A taker should split this into one unit per verb. Each new `SessionOp` touches VSEAM's vocabulary in `session/op.rs` and `tools.rs`, and the exhaustive op tables (`tests/gesture_table.rs`'s `OP_COUNT`).

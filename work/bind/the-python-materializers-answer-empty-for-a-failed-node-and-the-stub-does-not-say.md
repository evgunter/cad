---
id: the-python-materializers-answer-empty-for-a-failed-node-and-the-stub-does-not-say
kind: issue
title: Evaluation.all_faces / all_edges / all_vertices answer [] for a node that failed, and the stub does not say so
status: open
opened: 2026-11-22
priority: P3
cost: E
---



Found by the VSEAM lane binding `Doc.draw_on_face`
(`work/vseam/doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face`), during PR 7465's fix pass.

A profile drawn on an instance's bottom face without `resolver=` writes the identity orientation, which is backwards there. Its extrude is then poisoned at `evaluate`. Asked of that extrude, `Evaluation.all_vertices` answers `[]` and raises nothing. `Evaluation.value` on the same node raises `EvaluationError` (`pose_degenerate`, `face_chart_backwards`).

The Rust contract says this: "empty for a node with no value or no table" (`crates/editor-core/src/names/mod.rs`, `all_faces` and its siblings). The Python side does not:
- `pncad.pyi` `Evaluation.all_edges` says nothing about a node with no value;
- `all_faces`, `all_vertices` and `all_bodies` have no docstring;
- `crates/pncad-py/src/py/value.rs` (`all_vertices` and its siblings) passes the empty list straight through.

A Python reader that collects a node's vertices and reduces them can therefore pass vacuously on a node that failed. Two examples of the pattern:
- a check over every vertex is vacuously true;
- a `sorted(...)` compared against an expected list fails only on its length.

What closes it:
- say the empty answer in the stub, on each materializer;
- or have the Python doors raise the node's standing, as `Evaluation.value` does, for a node with no value.

Which of the two is a binding-contract choice for BIND.

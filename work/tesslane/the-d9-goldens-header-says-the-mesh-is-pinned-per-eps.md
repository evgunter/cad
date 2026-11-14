---
id: the-d9-goldens-header-says-the-mesh-is-pinned-per-eps
kind: issue
title: d9_mesh_goldens.rs says D9 pins bytes per (body, δ, ε) where [tolerance.chordal] says (body, δ)
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by a designer of the flat-triangle fork (`docs/DESIGN-FORK-LOG.md` row 269).

`crates/mesh/tests/d9_mesh_goldens.rs`'s header says D9 promises bytes for "(body, δ, ε)". `docs/DESIGN.md` `[tolerance.chordal]`, as Ev ruled it on 2026-08-21 (commit `1240686`), says the mesh is a function of (body, δ) alone and that a classification reading ε owes a guard. Two descriptions of one promise that disagree. Owed: the header says what the design says, or the row says why a golden is keyed by ε (the body it meshes may be built at that ε, which is a fact about the fixture and not about the mesh).

---
id: dual-tangent-is-unpinned-for-seven-recipe-verbs
kind: issue
title: One table-driven Dual64 tangent row over the recipe verbs no tangent row reads, under the fast-set bar
status: open
opened: 2026-10-02
priority: P3
cost: M
---

`crates/editor-core/tests/m10_di_dual_corpus.rs:98`
(`every_document_evaluates_at_dual64_with_the_f64_value_channel`) compares
only the value channel at `Dual64`, so a verb that drops its tangent stays
green there. Tangent-reading rows exist for the blend
(`sweep/tests/blend_dual_tangent.rs`, `editor-core/tests/blend_dual_sensitivity.rs`)
and none name `Sweep`, `shell`, `chamfer` (recipe node), `PlacedUnion`,
`Pattern`, `Tube` or `HollowTube`.

Owed: ONE row, a table over those verbs on the smallest document each needs,
seeding one size and checking the tangent of one moved quantity against a
central difference of the `f64` build; with a planted tangent drop shown red
once in the PR body. It must stay under the fast-set bar (`.config/nextest.toml`);
if it cannot, it goes in the slow set rather than seven per-PR rows.

What would close it: that one row landed under the bar, or in the slow set.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Silent-wrong-derivative coverage, no fix-cycle incident. As written, seven new
editor-core rows (cost H) on every PR selecting editor-core. Grain worth keeping: one
table-driven row under the fast-set bar (or in the slow set). `m10_di_dual_corpus.rs:98`
confirmed value-channel only. Reform: reform/dual-tangent-is-unpinned-for-seven-recipe-verbs.md

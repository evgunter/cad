---
id: comb-rows-ask-a-scope-against-itself-since-the-metric-doors-dropped-pairing
kind: issue
title: Four m10_5_r2 comb rows still ask one scope against itself, which reads 0 since #5955, so they may assert nothing they used to
status: open
opened: 2026-10-29
priority: P3
cost: E
refs: [self-intersection-drops-every-vertex-sharing-face-pair-globally]
---


## Finding

Found by PR #5979, which re-fixtured `the_cost_curve_is_flat_where_the_bound_is_broken`. That row went red on main after #5955 (SELFX): under Ev's E7 ruling, a scope that meets itself reads 0.

Four sibling rows in `crates/editor-core/tests/m10_5_r2_probes_interval.rs` still call `clearance_with(.., &sel, &sel, ..)` with one multi-wall scope, near lines 797, 920, 957 and 995.
- They pass.
- But each now answers "0, on the shared cell" or "Violated", so it may no longer measure the gap it names.

For each row, read what it asserts. Then either re-fixture it to two scopes (the #5979 pattern), or show that it still pins its subject.

A test that passes for the wrong reason is this row's subject. The class is any test whose assertion survived #5955's semantics change only because its predicate is weak.

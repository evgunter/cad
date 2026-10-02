---
id: a-union-over-a-declared-continuation-keeps-its-walls-split
kind: issue
title: B ∪ A over a declared rounded continuation builds the thick plate with its walls split where the thin plate's lay (18 or 14 faces against 10)
status: open
opened: 2026-10-02
priority: P3
cost: M
---


Found on `reach/door-backstop`. It is pinned by
`crates/sweep/tests/reach_continuation.rs`
`declared_rounded_continuations_inside_a_wall_build_subtract_and_intersect`.

## Repro

A is the rounded 6 × 4 × 1 plate (corner fillets r = 0.5). B is the
same outline 0.5 thick, sunk inside A or flush with its top or bottom.
Every finding is declared, keyed for (B, A). `union_with(&b, &a, …)`
builds A's solid at its oracle volume, `24 − (4 − π)/4`, and passes
tier 3 and 3′. But it carries 18 faces (sunk) or 14 (flush) where A
has 10. The walls B shared with A, declared continuations, stay split
at B's edges instead of merging back into A's walls.

## What would close it

The merge (`merge_coplanar_faces_declared`, and its curved-wall
counterpart) folds a declared continuation's split walls back into one
face where the result is one surface patch, so `B ∪ A` returns A's 10
faces. Measure first whether the curved fillet walls or the planar
sides are the ones left split.

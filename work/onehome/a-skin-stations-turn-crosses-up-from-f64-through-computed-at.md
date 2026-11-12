---
id: a-skin-stations-turn-crosses-up-from-f64-through-computed-at
kind: issue
title: a skin's station turn crosses up from f64 through RigidMotion::computed_at, the one up door Ev's ruling on PR 5068 leaves no room for
status: open
opened: 2026-11-10
priority: P1
---


Filed by PR 6717 (ONEHOME rigid-motion, fork 258's final state: "no up door anywhere").

A skin's carry is computed at `f64`: each station holds its point and its turn as a `RigidMotion<f64>` (`crates/sweep/src/skin.rs:1845`, built at `:1903`). `place` composes each turn onto a section's plane at the lane scalar `T` by lifting it with `turn.computed_at::<T>()` (`crates/sweep/src/skin.rs:1973`). `RigidMotion::<f64>::computed_at` (`crates/geom-core/src/linalg/rigid_motion.rs:1094`) carries each of the twelve components up through `Real::from_computed`.

At `Sym`, `from_computed` is `Sym::opaque`, a fresh unknown per call, so the lifted turn's columns are twelve unrelated atoms. Their orthonormality is unprovable, which is the crossing Ev's ruling on PR 5068 forbids ("a frame exists at a scalar only by being minted there"). The module's own "Crossing scalars" section says no door carries a frame up from `f64`, and this one does.

Other callers: `crates/topo/tests/lane0_r2_probes.rs:83` (a suite) and two of geom-core's own tests (`rigid_motion.rs:2647`, `:2702`).

What is owed (sweep's lane):
- Mint the station turn at `T` from the path's own data at `T`, so the frame is built at the lane scalar. That means the rotation the carry computes, over the path's tangents read at `T`.
- Then remove `computed_at`, or keep it test-only.

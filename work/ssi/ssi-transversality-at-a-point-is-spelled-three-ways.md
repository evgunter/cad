---
id: ssi-transversality-at-a-point-is-spelled-three-ways
kind: issue
title: ssi: transversality at a point is spelled three ways, and the march's lever reads the chart's parameterisation
status: open
opened: 2026-10-04
priority: P2
design: true
---


(Delta review of PR 3983 at `43998e7c81`, finding S1 and the m2 ruling; probes on `analysis/hermite-review/3983`, `crates/geom-brep/tests/rev3983_probes.rs`, `rev3983_warp_mid_arm`.)

## What

Whether two surfaces cross at a clear angle at a point is decided three ways on the SSI lanes, each `sin θ` levered by a different length:

- **The march** (`ssi/march.rs`, `decide_transversality`): `sin θ · min(lever_arm, extent)`. On the plane × NURBS lane the lever arm is the system's `lever_arm`, the wall chart's speed² over its second derivative. That reads the chart's parameterisation, not the surface's geometry: a flat wall whose `u` lines are spaced unevenly has a short lever arm where they bunch.
- **The boundary pass** (`ssi/boundary.rs`, the strip classification before `SsiError::BoundaryTangent`): its comment says "levered as the march's transversality is", but it levers by `self.extent` alone.
- **Limb 3's tube** (`ssi/certify.rs`): its clearance is the ring's zero-free lower bound times `TubeScale::uniform(extent)`'s arm, the extent alone.

Since PR 3983 a Hermite branch is decided by the march's spelling at its two ends and by the tube's between them. A marched branch is decided by the march's spelling at every state. So the same geometry can refuse when marched and answer as a Hermite.

## Witness

`m5_pr7_ssi::a_flat_wall_whose_chart_bends_answers_as_the_plane_it_is` (`warped_flat_wall`): the plane `z = α·x`, its chart `x = X·(s − ½) + X·κ·4t(1 − t)·2s(s − ½)²`, cut by `z = 0`. On main before PR 3983 (march first) 3 of the reviewer's 5 configurations refused "too close to call: margin 7.81e-9" mid-branch; with Hermite first all 5 answer the line.

## Design question

Which length levers transversality: one that is a property of the geometry (a curvature radius of the surfaces, clamped by the extent), shared by all three decisions, or the extent alone? A chart-parameterisation lever makes the answer depend on how a surface is parameterised, which no other SSI decision does.

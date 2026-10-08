---
id: ssi-transversality-at-a-point-is-spelled-three-ways
kind: issue
title: ssi: transversality at a point is spelled three ways, and the march's lever reads the chart's parameterisation
status: closed
opened: 2026-10-04
closed: 2026-10-06
branch: ssi/transversality-lever
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

## Closed (2026-10-06, `ssi/transversality-lever`)

Settled by a two-designer fork (four rounds; probes on
`analysis/design-fork/transversality-lever-a` and `-b`) and built as
its converged state. Point decisions (the march's states, the
Hermite's two ends, refinement's unsettled chord midpoint, and the
at-rest per-sample check in `plane_nurbs_limbs`) read
`sin θ · min(ρ, E)`, `ρ` the surfaces' curvature radius from the shape
operator (`dihedral::max_principal_curvature`), replacing the ℝ⁴ chart
arm. Region decisions (the boundary strip, limb 3's tube on both
lanes) read the certified least sine levered by `E` alone:
`TubeScale` holds only the extent, and the ℝ³ tube's arm read at the
carrier's first point is gone. `SsiError::TransversalityBand` names
which length its arm was (`PointLever`). README C3 states the rule.

The live witness is `m5_pr7_ssi::a_flat_wall_whose_chart_path_bends_answers_as_the_plane_it_is`
(the reviewer's witness no longer reaches the march on main).

Residue, each its own row:
`ssi-the-chart-lane-has-no-near-tangent-pose-decision`,
`ssi-limb-three-takes-the-widest-one-arc-rungs-band-verdict`,
`ssi-boundary-strip-sine-divides-by-the-whole-walls-speed`,
`ssi-limb-three-chart-sine-reads-the-chart-perpendicular`,
`ssi-a-chart-rung-can-be-narrower-than-eps-in-metres`.

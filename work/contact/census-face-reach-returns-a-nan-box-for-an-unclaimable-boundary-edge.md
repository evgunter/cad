---
id: census-face-reach-returns-a-nan-box-for-an-unclaimable-boundary-edge
kind: issue
title: census::face_reach returns a NaN-ended Some for a cylinder or cone face with an unclaimable boundary edge, against its own None contract
status: open
opened: 2026-10-02
---

## What

`census::face_reach` (`crates/topo/src/census.rs`) promises in its doc
that "a `Some` never carries a NaN end": it answers `None` for a
description with no claim in it. Its `CylinderSlab` and `ConeSlab`
arms do not keep that promise. Both read the axial window through
`boundary_axial`, which maps a boundary edge with no sound box
(`EdgeBoxRule::NoSoundBox`: a NURBS carrier or a null-scaffold edge;
`EdgeBoxRule::Spiric`) to `AxialCarrier::Unclaimable`. Then
`edge_axial_span` widens that edge's span by `poison_value`, and the
hull carries NaN into `slab_extent`. The arm wraps the result in `Some`,
so the caller gets a box whose six ends are all NaN.

Observed by TANG (`torus-carrier-axis-margin-is-levered-by-one-not-the-ring`).
It happens mid-union in `sweep`'s
`m9_3_zip::kissing_rounds_rim_unions_and_carries_the_tangent_intersection`
and `r1_probes_m9_3::probe_tube_chain_additivity_error_measured`, under
workspace feature unification only. There `insert::record_germ_dir`
read a declared-Tangent cylinder face whose boundary carries a
`CurveGeom::NullScaffold` edge, and `face_reach` handed back
`(NaN, NaN, NaN)`–`(NaN, NaN, NaN)`. TANG's door now reads the
declared pair's extent at rest and refuses a ball that is not finite
(`ExtentBall::readable`), so nothing it consumes depends on this. The
contract violation is still open for every other caller of
`face_reach`.

## Shape of a fix

Make the slab arms answer `None` when the axial window is poison, as
the `ControlNet` arm already does for a placeholder. Alternatively,
weaken the doc to match the arms and audit the callers. Either way, a
row should pin a cylinder face with an unclaimable boundary edge.

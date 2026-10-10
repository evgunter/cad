---
id: patch-doors-take-a-control-net-beside-two-vectors
kind: issue
title: The patch-flux doors take a control net and weights beside two knot vectors, related by count
status: open
opened: 2026-10-10
---


Found by the `coefficient-vector-pairing-survivors` sweep (NURBS); the
shape that unit closed elsewhere — a coefficient array beside a knot
vector related by length alone — at the patch-flux doors, which that
unit's row did not list.

- `crates/geom-brep/src/props/quad.rs` `nurbs_patch_face`,
  `nurbs_patch_face_rounds` (pub; `topo::props::quad_lane` calls it
  with a `NurbsSurface`'s parts), `trimmed_patch_face_rounds` (pub),
  `rational_patch_face` — each takes `(kv_u, kv_v, control: &[RVec3],
  weights: &[f64])`. A short net refuses the slots it misses
  (`row_major`, beside `PatchGrid::base`), and the rational arm
  re-checks `weights.len() != control.len()`.
- `refine_net` / `refine_dir` return `(KnotVector, KnotVector,
  Vec<RVec3>)`, a refined net beside its refined vectors.
- `TrimPiece` (pub, pub fields) holds `knots`, `control`, `weights`
  side by side.

Disposition: the rational, multi-channel tensor pair has no type yet —
`TensorCoeffs` (`crates/geom-core/src/spline/net.rs`) is one scalar
channel. Either a door taking the caller's `NurbsSurface` (which checks
the counts once at construction), or a rational tensor pair minted once,
closes all four; `PatchGrid::base` already takes an entry function at
the vectors' extent, so only the door's spelling moves.

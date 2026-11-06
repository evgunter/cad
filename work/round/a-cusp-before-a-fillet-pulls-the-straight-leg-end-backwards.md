---
id: a-cusp-before-a-fillet-pulls-the-straight-leg-end-backwards
kind: issue
title: line(L).cusp().fillet(r) moves the straight leg's end vertex BACKWARDS: the authored end leaves the path, and past the start the declared cusp is silently dropped; both validate
status: review
opened: 2026-11-06
priority: P0
cost: E
branch: round/cusp-fillet-keeps-its-leg
refs: [carriers-are-identical-reads-carrier-identity-at-the-lattice]
pr: 6545
---


**Found by ROUND's probe lane (2026-11-06), by execution**, while
measuring a design fork (`carriers-are-identical-reads-carrier-identity-at-the-lattice`).

## Finding

`cusp_kernel` (`crates/profile/src/path.rs`) sets `ang_by_tangent` with
the ray reversed. The ray-extension merge — `merge_of`'s Ray arm
(`path/family.rs`), and the line×line and arc-carrier arms that call
`extend_leg_to` in `path.rs` — treats every by-tangent ray off a
straight leg as a FORWARD extension of that leg, so it rewrites the
leg's end vertex to the trim point t1, which lies BEHIND the leg's end.

Probes (`round/probes`, `crates/profile/tests/round_probe_ray_extension_joints.rs`,
`m1c_*`, `m1d`), setup `Open.at(0,0).angle(0).line(2).cusp()`, then
`.fillet(0.5)`, `.toward(0,1).to(x,2)`, `line_to(Start)`:

- corner (1,0): vertices (0,0), (1.5,0), (1,0.5), (1,2) — the authored
  `line(2)` is 1.5 m long and (2,0) is off the path (§4 item 3); the
  cusp moved to (1.5,0); validates.
- corner (−1,0): vertices (0,0), (−0.5,0), (−1,0.5), (−1,2) — the leg
  reversed heading, the arc now leaves it smoothly, and the declared
  cusp is gone from `cusp_joints`; validates.
- corner (−0.5,0): refuses `SeamTangent { margin: 0 }`.
- `.cusp().fillet_arc(0.2, Center{..})`: the same move through the
  fused door; (2,0) off the path; validates.

A live wrong answer on a normal verb: an authored point leaves the
final path, silently. No test covered `cusp().fillet`.

## Fix

The merge may only extend a leg along its own FORWARD tangent: a ray
reversed by `.cusp()` is not the leg drawn longer, so the incoming run
is emitted as its own segment from the leg's end, with the declared
cusp at that vertex. This holds under either answer to the open fork
on the straight-leg merge (keep it for forward rays, or retire it), so
it does not wait for that fork. The probes become the pins.

---
id: two-donuts-in-general-pose-stop-at-the-interior-loop-guard
kind: issue
title: Two donuts in general pose (placements 1 and 3) pass the result gate and stop at the interior-loop guard, whose section certificate has no torus × torus arm
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed by SPIRICARM's rotational-lane stage 3 (`spiricarm/rotational-lane-torus`),
which gave every trimmed torus face a volume lane.

## What

`crates/sweep/tests/germ_torus_components.rs`
`two_donuts_in_general_pose_pass_the_germ_mint`: the donut (`R = 0.6`,
`r = 0.2`) against itself moved to placements 1 and 3 of the germ row,
under ∪ in both member orders. Each run used to stop at the result
gate's volume on a torus face (`"ring on a torus face"`, `"torus trim"`),
which masked what lies past the gate. With the torus faces measured, the
gate passes, and each run now refuses at the crossings path's
interior-loop guard (`topo::boolean::ops` `interior_loop_verdict`,
raised after the gate):

```
CurvedPairUnsupported { op: Some(Union), site: InteriorLoopGuard, operand: A,
  face: FaceKey(3v1), kind: Torus, other_face: FaceKey(4v1), other_kind: Torus }
```

The section certificate (`topo::boolean::section_cert`) refuses a
torus × torus pair whose boxes overlap, so the guard cannot exclude an
interior loop there. Placement 2 builds. The row is pinned at the new
stop (`TORUS_PAIR`).

## Done when

The certificate classifies a general-pose torus × torus pair, or the
guard reads the pair's section the march already carried; the row moves
to a body held to an independent volume.

## The steep tilt meets the same guard on torus × cylinder

`crates/sweep/tests/germ_tilted_rod.rs`
`the_steeper_tilts_union_stops_by_identity` (the half donut ∪ a rod
tilted 0.7 rad): with the half donut's torus wall measured (its spiric
and its chords alike), the union passes the gate and stops at the same
guard on the torus wall × the rod's cylinder wall
(`CurvedPairUnsupported { site: InteriorLoopGuard, kind: Torus,
other_kind: Cylinder }`), pinned by identity.

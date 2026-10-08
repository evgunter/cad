---
id: a-pierce-whose-run-chords-enclose-one-another-needs-a-tree-ring
kind: issue
title: A pierce whose Out runs have one between others needs a tree of ring struts; it refuses typed (PierceRunsEnclose)
status: closed
opened: 2026-10-08
closed: 2026-10-08
priority: P1
cost: H
design: true
---


Weighed by a designer pair, who both recommended the tree below and
agreed on its final state; built in the PR that filed this.

## What

`boolean/vtxfac.rs` `classify_vertex_on_face` hangs one ring strut per
Out run at one ring vertex `w` in the pierced face, in the order of the
ring's corners (`ring_corners`). Where one run has other runs on both
sides of it about the pierced face's normal, it refuses
`BooleanError::PierceRunsEnclose` before any write. The result there is
a typed success: a pinch at the point, one vertex per region outside
the result (tier 3′), which `zip::split_cones` splits per cone. The
refusal is a limit of the star ring, not of the pose.

Witnesses, each refusing in all six ops against the plate:
- `topo::test_support::meeting::arch`, three wedges leant across one
  another. Its germs, in degrees about +z: run 0 30 → 120, run 1
  150 → 0, run 2 270 → 240. Clockwise from run 0's start:
  `[0, 3, 4, 5, 2, 1]` (germ `2i` is run `i`'s start, `2i + 1` its
  end). Run 1's germs (2, 3) are not neighbours: its chord has run 0's
  on one side and run 2's on the other.
- `meeting::arch_cone`, an apex pyramid over a meander whose three runs
  above the top lie one inside another, at every pose; the middle run
  is between the other two.

## The derivation

The Out runs are chords of the vertex's link above the face, with
endpoints (the germs) on the face's circle of directions; disjoint arcs
of a simple closed curve, so the chords do not cross. The pierced face's
loop round the point is made of zero-length struts, each strut's two
halves facing its run's start and end germs (the null-edge record
fixes this). Every corner of that loop lies between the germs its two
halves face, and the corners are disjoint about the normal exactly when
the loop passes the germs in clockwise order.

- With a star (every strut at `w`), corners alternate between a strut's
  far end (its own two germs) and `w`, so each run's two germs must be
  neighbours in the clockwise order. That holds exactly when one region
  above the face borders every chord (`ring_corners`), and the star then
  builds (`a_cone_whose_runs_nest_under_one_builds_in_every_op`).
- In general the loop is the Euler tour of a plane tree of struts whose
  label sequence is the clockwise germ sequence: the dual tree of the
  chord diagram, one strut per chord, the tree's vertices the regions of
  the face's circle cut by the chords. A chord with chords on both sides
  is an inner edge of the tree. For `meeting::arch` the sequence is
  e0 e1 e2 e2 e1 e0 and the tree is the path c0 –e0– w –e1– Y –e2– c2:
  run 2's strut hangs off the far end of run 1's, not off `w`.
- Measured: every star for `meeting::arch` (3! hang orders × 2³ facings,
  16 distinct cyclically) refuses `Join` or `JoinDesync` in all six ops.

## What the tree needs

- Ring null edges whose `at_vertex` is not the ring vertex: a strut site
  at another strut's far end (`MevSite::Fan` on that strut's half), and
  its facing and below/above sides derived by the sense theorem there.
- The join's ring lane (`join.rs`, `ring_run_ccw`, `choose_roles`) and the
  weld of pierce copies accepting struts that are not all at one vertex.
- `zip::split_cones` splitting such a point per cone.
- Rows: `meeting::arch` and `meeting::arch_cone` building in every op,
  with `corners_disjoint`, tiers 3 and 3′, and a point-membership oracle,
  and a mutant that hangs every strut at `w` dying.

## Closed

`crate::null::ring_tree` reads the tree off `germ_order`'s clockwise
sort with a stack, no predicate of its own: each germ opens its run's
chord, hung at the far end of the open chord it lies under or at the
root, the half leaving that node facing it. The root is the region
bordering the most runs (the hub, where there is one, so every star
ring hangs as before), the walk starting at its corner before the
lowest run. A child strut's site is `MevSite::Fan` on its parent's half
leaving the parent's far end; its record's `at_vertex` and attribute
name the parent's far end, and `PierceRingRecord::ring_vertex` stays the
ring vertex. Struts at one node face alike; facings differ by depth.
Crossing chords would break the derivation, so they refuse
`ClassificationInvariant`. `PierceRunsEnclose` is retired.

`zip::split_side` splits a vertex's cones one at a time, each a cone
whose runs lie together: a ring deeper than a path nests cones round
the point, which the one-pass split refused as interleaved
(`meeting::branching_cone`'s P − U).

Rows (`crates/topo/tests/holes_meeting_at_a_vertex.rs`):
`meeting::arch`, `meeting::arch_cone` and `meeting::branching_cone`
build in every op and pose, with `corners_disjoint`, tiers 3 and 3′, a
point-membership oracle and the cones' closed-form volumes; every root
of each ring builds. `null::tests` pins the tree for each shape.

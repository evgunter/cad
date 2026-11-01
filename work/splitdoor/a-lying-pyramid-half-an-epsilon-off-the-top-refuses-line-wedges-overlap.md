---
id: a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap
kind: issue
title: The lying pyramid 0.5ε off the top refuses LineWedges Overlap since PR 6212 (a slow row red on main)
status: open
opened: 2026-11-01
priority: P1
cost: M
---


## What

The slow-set row
`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`
`a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose`
is red on main. Its pose "around the lying pyramid, the pyramid lying
0.5ε off the top, at rest", op `x − y`, refuses
`LineWedges { fault: Overlap { edges: [EdgeKey(7v1), EdgeKey(11v1)] }, from: "(1.75, 1.4330127018922192, 1.0)", to: "(1.5, 1.0, 1.0)" }`,
which the row's `typed` does not admit (`:1699`).

## Evidence (2026-11-01, SPLITDOOR, measured while driving PR 6221)

- Red with the same text on main `c0faf181` and on main `83030715`, at the
  default ε.
- It passed on PR 6221's branch when that merged main `45d0f75f`.
- `git bisect --first-parent 45d0f75f..c0faf181` names the merge of
  PR 6212 (`splitdoor/prism-tilt-reach`, "read two faces' line at the
  bounds' offsets from the other face's plane, so a within-zero tilt
  decides as upright", `367ff259`) as the first bad commit.

So the within-zero tilt now decides the pyramid's face as upright where
the line-wedge read then finds the two edges overlapping. Whether the
answer is a typed refusal admitted by the row, or a build, is
SPLITDOOR's to decide; it is not PR 6221's change.

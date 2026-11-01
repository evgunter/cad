---
id: a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap
kind: issue
title: The lying pyramid 0.5ε off the top refuses LineWedges Overlap since PR 6212 (a slow row red on main)
status: closed
closed: 2026-11-01
branch: splitdoor/lying-pyramid-line-wedges
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

## 2026-11-01 (SPLITDOOR, `splitdoor/lying-pyramid-line-wedges`)

This is an honest refusal. The exact on-top scene pins this cell: `REFUSING` lists "around the lying pyramid, x − y" as `LineWedges`, the unbuilt re-pairing (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`). Since PR 6212 the 0.5ε tilt decides as upright, as the row's own contract asks ("within the zero every scene builds as on the top"), so the pose refuses as the on-top scene does. It went red only because `builds_or_refuses` looked pins up by the tipped label, which never matches `REFUSING`. The row now passes the on-top scene's label as its pins for poses within the zero, with `not_pinned` enforced too. The file passes at 1e-6, 1e-9 and 1e-12.

## Closed

The refusal is the on-top scene's pinned `LineWedges`, the unbuilt re-pairing of coincident sheets (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`), which a pose within the zero now shares. The row's pin lookup is fixed so that pose is held to it; the re-pairing row owns the build.

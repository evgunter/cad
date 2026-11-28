---
id: a-wall-on-a-carrier-that-is-not-simply-connected-keeps-the-period-skip
kind: issue
title: Two one-band walls on one cylinder, cone or torus record stay two faces across their shared rim: the merge keeps the period-closure skip there
status: open
opened: 2026-11-23
priority: P2
refs: [a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard]
cost: M
---


Found by the LATHE interior-loop lane (2026-11-23, PR 7440).

## What

`merge_faces`' `merge_group` closes a curved run through its same-face
edges only where the survivor's carrier is simply connected (a sphere, a
spindle sheet): no closed curve on it winds, so a same-loop duplicate
that is no spur is a bridge and goes by `kemr`, the run's wrap edges
kept as its seam. On any other curved carrier every same-face
duplicate the absorption leaves still skips `PeriodClosure`, the wrap
edges included, because there a same-face edge may be the cut that
keeps the face's period open:

- a cylinder or a torus: a seam from rim to rim is essential (it cuts
  the period the face would otherwise close), and a torus face has two
  periods to keep open;
- a cone: its carrier less the apex is not simply connected either. A
  generator from rim to rim on a face clear of the apex is essential; a
  second edge into the apex on a face that holds it is a bridge, as on a
  sphere. Telling the two apart needs the face's own windings
  (`pcurves::loop_windings`, read after a trial `kemr`), not the
  carrier's kind.

So two one-band walls on one record meeting along a full rim stay two
faces at rest, against `[topology]`'s maximal faces: the union of two
stacked full-turn collars of one radius
(`crates/sweep/tests/a_peg_in_a_two_segment_bore_moves_only_its_wall.rs`,
"the bore's wall is two faces") is the measured case. With the skip
lifted for wrap edges on every curved chart, the bore glued into one
wall face and built valid (measured on the PR's branch before the
narrowing), and that row's fixture lost its premise.

## Open

Whether a cylinder or torus run lifts the skip for its wrap edges too,
which needs the wrap pieces of the members to chain into one seam from
rim to rim (one record, one chart, so one azimuth), and a re-built
fixture for the peg row, whose subject is one record worn by two
faces.

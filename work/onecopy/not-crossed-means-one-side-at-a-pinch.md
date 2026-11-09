---
id: not-crossed-means-one-side-at-a-pinch
kind: issue
title: a ring that touches or crosses the run at a vertex the division did not identify reads both sides through the 'not crossed, so one side' ladders (fate's rehome, chord_join's rehome_rings); a torus's essential cycles unprobed
status: open
opened: 2026-11-09
priority: P3
cost: E
---


## Finding

Filed by `onecopy/side-at-infinity` (PR 6595's review). That PR found
`whole_body_side`'s premise, "the plane does not cut the body, so it lies
on one side", false at a pinch: two lumps that meet only within the plane
lie on opposite sides. The same shape of premise underwrites other
`Answering::Any` ladders, which take their first answer:

- `boolean/fate.rs`, the ring rehome (~1358): "a ring the cut does not
  cross is on one side".
- `one_region.rs` `read_cycle` (~233).
- `chord_join.rs`'s side ladders (the premise at ~4021; climbs at ~3981,
  ~4042, ~4086, ~4834, ~5470, ~5549).

A ring or cycle that meets the dividing loop only at a vertex, with its
two halves on either side of the loop there, is not crossed and yet
reads both sides. **Probe:** for each site, build a ring that touches the
dividing loop at one vertex (a figure-eight pinch), read every rung, and
log whether the answers disagree, as PR 6595's scratch instrument did.
Disagreement makes it a defect at that site; agreement for a reason the
site can state settles the premise.

## Narrowed (`onecopy/pinch-premise`)

The premise holds where the loop does not cross the run. A simple loop
that meets the run only at touches that do not cross lies on one side,
because a touch switches no side. That covers a touch at a vertex the loop
shares with the run, and a touch at a copy of one: the split's reduction
keeps one vertex per piece at a pinch, so a ring can touch the run at a
vertex it does not share, and it reads `OnBoundary` there and passes
(`chord_join::ring_side`). The argument has one home, `chord_join::paths_rung`'s
premise doc. Fate's rehome, `one_region::read_cycle`, chord_join's
`ring_side`, `path_ring_side` and the lone ring's outer side cite it.

**What remains** is not specific to figure-eights:

- **A loop that touches or crosses the run at a vertex the division did
  not identify.** Such a loop reads both sides through the same
  mechanism: a crossing it should have joined, or a touch read as a
  crossing. The paths that can produce it are fate's rehome (the
  boolean's cut through a divided face) and `ChordJoiner::rehome_rings`.
  The plane split identifies every vertex within band of the plane, so
  it cannot. No probe of this case is built. Its guard is the debug check
  on the `Any` ports that their answers agree
  (`a-ladder-that-checks-its-answers-agree-has-no-home`), which waits on
  `[ev]` PR 6592.
- **A torus face's essential cycles** in `one_region`, which are not
  probed.

`one_region` is a validator, and nothing joins there. A cycle that
crosses another at a point they share is check 9's to refuse
(`RingMeetsOuter` / `RingMeetsRing`, or `PinchCornerCrossed`), as
`one-region-reads-a-cycle-crossing-at-a-shared-vertex-by-its-first-vertex`
found and closed.

**Evidence that a touch at a pinch reads one side.**

- *The instrument.* A scratch check read every rung at fate's rehome,
  `one_region::read_cycle`, chord_join's planar `ring_side`, the lone
  ring's outer side, the quadric ring pairs and the winding references.
  It found no disagreement at these sites over topo ci and sweep ci.
- *The figure-eight row.* It ships as a test,
  `crates/topo/tests/a_figure_eight_hole_cut_through_its_pinch.rs`. A
  plate with a figure-eight hole, one loop through the origin twice, is
  built raw (`drill_hole`, described, `finish`), since the public doors
  refuse it (`LineWedges` overlap). It is then cut through the pinch by
  `split` (x + y = 0) and by `subtract` and `intersect` against a prism
  whose face lies on x + y = 0. Each side keeps its own lobe, merged into
  its face's outer loop, and the boolean results pass the census.

The same instrument found one real disagreement that is not a pinch: a
cone winding whose fixture's island left its sheet,
`cone-ring-rows-sheets-do-not-hold-their-islands` (PR 6607).

---
id: not-crossed-means-one-side-at-a-pinch
kind: issue
title: a division within band of a ring's pinch vertex that does not identify with it leaves a figure-eight ring a bystander the 'not crossed, so one side' ladders read both sides of; a torus's essential cycles unprobed
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

The premise holds where the division identifies with the ring's
vertices, and each site now states why. A simple ring that meets the
division only at points lies on one side, since crossing once forces a
second crossing. A ring reaches both sides without being crossed only by
switching lobes at a self-touch, which on a face is a vertex (two corners
at one point). That vertex lies on the division, so the division runs
through it and joins the ring into the run, and the ring never reaches
these ladders as a bystander.

**Evidence.**

- *Instrument.* A scratch check read every rung at fate's rehome,
  `one_region::read_cycle`, chord_join's planar `ring_side`, the lone
  ring's outer side, the quadric ring pairs and the winding references.
  It found no disagreement at the pinch sites over topo ci and sweep ci.
- *Figure-eight probe.* A plate with a figure-eight hole: one loop
  through the origin twice, lobes in quadrants I and III. It is built raw
  (`drill_hole`, described, then `finish`). The public doors refuse to
  build it (`LineWedges` overlap). The plate was cut through the pinch by
  `split` (x + y = 0), and by `subtract` and `intersect` against prisms
  with a face on x + y = 0 or on x = 0. In every run each side kept its
  own lobe, merged into the face's outer loop, and no rung disagreed.
  The boolean results pass the census.

**What remains:**

- a division that passes within band of the pinch vertex without
  identifying with it, which would leave the figure-eight a bystander
  touching the division at a vertex reading `OnEdge`, with its two lobes
  on opposite sides. No probe of it is built. The sites move to the
  agreeing climb (`a-ladder-that-checks-its-answers-agree-has-no-home`)
  when it lands, which refuses this case instead of answering it;
- a torus face's essential cycles in `one_region`, which are not probed.

The same instrument found one real disagreement that is not a pinch, a
cone winding whose references disagreed because its fixture's island left its sheet: `cone-ring-rows-sheets-do-not-hold-their-islands` (PR 6607).

---
id: not-crossed-means-one-side-at-a-pinch
kind: issue
title: 'not crossed, so one side' Answering::Any ladders may read both sides at a pinch: fate's ring rehome, one_region read_cycle, chord_join's side ladders
status: open
opened: 2026-11-09
priority: P3
cost: M
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

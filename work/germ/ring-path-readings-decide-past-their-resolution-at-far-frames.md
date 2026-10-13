---
id: ring-path-readings-decide-past-their-resolution-at-far-frames
kind: issue
title: The ring lane's plane readings decide past their resolution at far frames (ε = 1e-12, origins ~1e6)
status: open
opened: 2026-10-13
---


## What

At ε = 1e-12, a cone placed far from the origin reads wrong path
parities in the ring lane. The frame is the cone rows' turned frame
moved to the origin `(8, −3, 5)·1e5` (coordinates ~1e6) at scale 1e3.
The run is `ring_path::tests::a_cone_path_crosses_an_island_by_the_oracle_parity`'s
grid with the heights 2.2 added, against the lune, the lune by the
apex and the sector, every path, measured against the islands'
oracles:

- **main's readings: 440 wrong paths of 92 408 decided.**
- Origins ≤ 1e5 (scale 1 or 1e3): 0 wrong. The PR 4910 reviewer
  measured 1 wrong at ≤ 1e5 on main, and 0 at ≤ 1e4.
- ε unset and 1e-6: 0 wrong at every origin.

A coordinate there is stored to ~1e-10 (an ulp of 1e6). That is ~100×
the zero band. So points that should lie on one plane, or one circle,
disagree by more than the band, and margins read between them decide
on rounding. This is the regime that `plane_roots`' own pick comment
excludes ("the rounding is `δ ≤ ε` wherever the coordinates resolve
the band at all", `crates/topo/src/ring_path.rs`, `plane_roots`,
:1244). Nothing refuses it.

## Where

- `ring_path.rs`, `plane_roots` (:1212), **`split_ring_path_meets_plane`**
  (:1224): the gap `r·ρ − |D|` is decided with no charge for the
  coordinates' rounding. In the far frame, a parallel lying in a loop
  circle's own plane reads a decided meeting, two roots or none, where
  its true gap is in the band.
- `ring_path.rs`, `path_parity`'s in-span readings (`in_span`, :875 →
  `span_readings`, :1092, **`split_ring_path_in_span`**) at those
  roots, and `split_ring_path_segment_side`: likewise uncharged.

## What PR 4910 did on its own readings

`misses_in_plane` and `segment_misses` charge `resolution` (:1109,
four roundings of the farthest point's distance from the origin) to
their margins: the coplanar gap, the segment's apart margin, and the
same-circle span ends. Before the charge, the PR's readings added 8
wrong paths in that frame (6 by `split_ring_path_coplanar_apart`, 2
by `split_ring_path_segment_apart`). After it, 2 remain, and
both are this item's class: the segment reading is right on each
(margin 705 m against a true distance of 959 m). It decides the
path's lead, which used to leave the path undecided. The path's
parallel, which lies in the top circle's plane, is then read wrong by
`plane_roots` above.

## Fix shape

Charge the same `resolution` to `split_ring_path_meets_plane`'s gap,
the segment sides and the in-span readings. Then a reading that cannot
resolve the band stands down, or escalates, instead of deciding. Pin
it with the far frame above, holding 0 wrong at ε = 1e-12.

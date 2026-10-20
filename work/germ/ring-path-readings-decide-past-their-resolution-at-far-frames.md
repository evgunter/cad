---
id: ring-path-readings-decide-past-their-resolution-at-far-frames
kind: issue
title: The ring lane's plane readings decide past their resolution at far frames (ε = 1e-12, origins ~1e6)
status: closed
pr: 5012
branch: germ/ring-path-far-frames
opened: 2026-10-13
closed: 2026-10-14
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

## Findings (germ/ring-path-far-frames)

Reproduced on main at d44ee2f: **442 wrong of 92 654 decided**. The
item's own grid adds 2.2 to the cone row's heights, both nappes. The
counts differ from the item's 440 / 92 408 because PR 4927 landed
between the two runs. The torus islands from their arcs' carriers and
planes, in the same far frame, read **5270 wrong of 598 847** on main.

Those counts read the islands' oracles on the far frame's rounded
coordinates, and from grid points on the islands' boundaries (the
sector's four on its top arc at height 2.2, and the points on the
ovals and parallels within their spans), where the oracle reads
rounding noise. Read at each point's local twin, with points within
`1e-6` local units of a boundary set aside, main at 6f2656a reads
**162 wrong of 91 356** on the cone grid and **1914 wrong of 584 313**
on the torus's.

The fix charges the coordinates' `resolution` to every reading between
points stored in the frame, shrinking each margin toward zero by it
(`charge`, `crates/topo/src/ring_path.rs`). In `path_parity` the slack
is `path_resolution`: of every piece's ends and centre and every arc's
centre.

- `plane_roots`, **`split_ring_path_meets_plane`** (:1506): charged.
- `span_readings`, **`split_ring_path_in_span`** (:1344): charged, by
  `slack / arm` in the parameter. `misses_in_plane`'s span widening
  becomes this same charge.
- **`split_ring_path_segment_side`** (:1219), **`split_ring_path_torus_loop`**
  (`ArcLoop::read`, :825), **`split_ring_path_crest_graze`** (:1120),
  **`split_ring_path_crest_side`** (:1181) and **`split_ring_path_rulings_apart`**
  (:1268): charged.
- `Quadric::paths`: **`split_ring_path_span`**, **`_nappe`**, **`_rise`**
  and **`_off_core`** are charged, and so is `Quadric::off_apex`
  (**`split_ring_path_off_apex`**). On main, `_rise` decided on rounding
  for two ends at one height in the far frame. It offered a second path
  with an arbitrary arrival direction: 94 638 paths asked, against
  91 776 at the origin. Charged, the far frame offers exactly the
  origin's paths.
- Not charged: **`split_ring_path_line_across`** reads two stored
  directions. **`split_ring_path_lead_point`** and
  **`split_ring_path_same_circle`** decide by a zero that hands the
  case to charged readings, so a charge would only widen it. The four
  `torus_circle_arc` classifications are filed as
  `torus-circle-arc-classifies-far-frame-carriers-uncharged`.

Rows: `ring_path::far_frame_rows` (cone, torus, and one unit row per
charged reading). Each is red on main. After the fix there are 0 wrong
at every ε. Paths decided at ordinary frames, and at far frames at
ε unset and 1e-6, are unchanged to the path.


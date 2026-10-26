---
id: the-side-readers-aims-decide-exact-ties-at-a-documents-nominal
kind: issue
title: The side reader's aims land on exact ties at symmetric nominals
status: open
priority: P2
opened: 2026-10-19
cost: H
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
---

Since PR 5119, check 7 reads each solid with the side reader's aimed
rays (`crates/topo/src/side_reader.rs`, `aim_at`), so every at-rest
gate takes the aims' own decisions. Two aims sit on symmetric points of
the stored geometry, where those decisions tie exactly at a document's
nominal:

- a planar face aims at its certified interior point
  (`crates/topo/src/stands.rs`, `planar_interior_point`), whose in-face
  walk decides `point_in_loop_arm` against an axis-aligned schedule
  direction: `die_chamfer` reads it exactly 0 at decision 1617 of 4547;
- a cylinder, cone or torus face aims at the middle of its chart window
  (or the point (π, π)), where the ray's foot on a circular edge decides
  `curve_foot_branch`: `cut_cylinder` reads two exactly 0.

The reader's answer turns on none of them (`[verdicts]`), and its
decisions are recorded under one family (`topo::SIDE_READER_FAMILY`,
`geom_core::k_stats::in_family`, tagged on the symbolic recorder too),
which `editor-core`'s N5 flip lane and `revalue_corpus` read apart, so
no row moves. What remains is the instrument representing the tie at
all.

**The fix is the edge-meeting ray**
(`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`),
which decides each edge once for both faces it bounds and stops
representing those ties.

## Measured on the edge-meeting ray

`crates/editor-core/tests/revalue_corpus.rs` `reader_ties_at_the_nominal`
(ignored) lists every decision under `topo::SIDE_READER_FAMILY` whose
margin is exactly 0 in a build at the nominal:
- `die_chamfer`: 2 on main (`point_in_loop_arm`, decisions 1616 and
  1647), **0** with the edge-meeting ray. A line-bounded planar face aims
  at its loop's mean point, which asks nothing, and is crossed edge by
  edge or by the turn of its loops (`ray_crossings`), so no in-face walk
  casts an auxiliary direction. The planar half is retired.
- `cut_cylinder`: 90 on main and 90 with it (`curve_foot_branch` 12,
  `point_in_arc_loop_side` 6, and the structural wall and chart reads
  `bool_wall_iso_rim`, `bool_wall_iso_meridian`, `bool_wall_section_seat`,
  `pcurve_chart_radial_moving`). The curved aims and trims are unchanged,
  so the wall half waits on the curved edge-meeting ray.

## Measured on the curved edge-meeting ray

The curved half (`a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim`)
decides each edge once for both faces, but casts the same aims and reads
a crossing no edge stands for through the same in-face walk and trims,
so it retires none of these ties. `reader_ties_at_the_nominal`, at the
nominal:

- `die_chamfer`: 0 before and after.
- `cut_cylinder`: 62 on main (`28e9e447`) and 62 with it, of 1719 and
  1827 reader decisions. The rows named above as 90 have since moved
  with the ruled region's rework (PR 5664). What remains:
  - `curve_foot_branch` 12 and `point_in_arc_loop_side` 6: a cap's aim,
    the certified interior point `planar_interior_point` reads through
    the in-face walk (a two-arc disc's vertex chord midpoint, its
    centre), and the in-face walk the cap's crossing takes there. Both
    belong to a planar face with a circle edge.
  - `bool_ruled_region_roots_coaxial` 44: the wall's region walk
    (`ruled_region`) at each wall root, whose parallel legs are coaxial
    with the rims by construction.

**What retiring them takes, re-priced.**

1. *A planar face with circle edges* (M): aim at a point that asks
   nothing (its loop's mean over the vertices and the arcs' middles, as
   a line-bounded face does), and cross it by the turn of its loops,
   each arc read as chords of at most a quarter turn plus the turn of
   each piece's circular segment where the crossing lies in it. Where
   every edge is decided clear of the ray, the crossing is definitely
   off each circle, so the segment reads are definite.
2. *A curved face's root no edge stands for* (H): a reading of the
   root against the face's loops that casts no auxiliary path, which
   the ruled region's coaxial legs are. Nothing in the tree reads a
   curved face's region that way yet.

The row is H for the second half; the first could land alone.

## Measured on the trimmed-sphere aim (FLUSH, PR 5730, 2026-10-26)

PR 5730 gives a trimmed sphere face an aim (`sphere_region.rs`
`SphereFaceRegion::aim`), which no corpus document casts, so
`revalue_corpus::reader_ties_at_the_nominal` cannot see it. Its review
recorded `Sym<f64>` builds of trimmed-sphere bodies (PR 5730's review
probe `review_sphere_aims_sym_ties`, on `flush/sphere-aims-review` @
`6f5073f6`) and counted the decisions under `SIDE_READER_FAMILY` whose
margin is exactly 0. Lens = unit ball ∩ 0.8 ball at `z = 0.9`; the
symmetric pair is two unit balls at `z = ±0.5`; block = a 4-cube.

| body | main | first aim (mean of steps) | PR 5730 aim (golden-section step) |
|---|---|---|---|
| symmetric lens | 0 | 2 | 1 |
| lens | 1 | 2 | 2 |
| symmetric bite | 0 | 1 | 1 |
| block less the symmetric lens | 0 | 13 | 5 |

The same counts at ε 1e-9, 1e-6 and 1e-12. Every tie left is
`bool_pierce_normal_on_chart`. The first aim's `bool_sphere_region_order`
and `curve_foot_branch` ties (the in-face walk of the aim's crossing on
the symmetric plane through an arc's middle) went when the aim moved off
the middle to the golden section of the arc. None moves a reading:
every aim of every body reads the same side. What is left is this row's.

## Measured: from PR 5730's review (2026-10-26)

- **At `Interval`, two arcs that tie for the deepest step hull their aims into one box.** The tie is on `SphereFaceRegion::aim`'s pick of the deepest step. The symmetric quarter lune's aim is an x/y box [0.535, 0.787], off the sphere, and the on-chart normal check likely refuses it. So a symmetric face offers no aim at `Interval` and falls back to its volume; no side changes. This is an exact tie at a symmetric point, so it belongs to this row.
- **The ruled aim still steps from its arcs' middles.** `RuledFaceRegion::aim` (`ruled_region.rs`) uses `mid_param` and the meridian fallback. Moving the sphere aim off the middle (the golden section) removed its symmetric-middle ties (`bool_sphere_region_order`, `curve_foot_branch`). The same move was not tried on the ruled aim, where `cut_cylinder`'s `curve_foot_branch` ties live.


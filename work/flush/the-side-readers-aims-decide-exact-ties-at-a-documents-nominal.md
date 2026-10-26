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


## Measured on main (FLUSH, `flush/aims-ties`, 2026-10-26)

`revalue_corpus::reader_ties_at_the_nominal`, widened to every corpus
document, at `ea22954f` (PR 5730 merged): **483** exact ties under
`SIDE_READER_FAMILY` in 11 of 30 documents. Each tie was attributed to
the aim whose ray took it by a marker decision logged before each aim
and each ray (a probe, not kept):

| tie | where | why it is exactly 0 |
|---|---|---|
| `curve_foot_branch`, `point_in_arc_loop_side` (cut_cylinder 18, boss_union 16, die_pips 3, die_tool 3, die_composed 6, die_composed_tour 9, plate_param 12) | a planar cap with circle edges: its aim (`planar_interior_point`, a two-arc disc's vertex-chord midpoint, its centre) and its crossing (`point_in_face`'s in-face walk), on rays aimed at that cap or at a line-bounded face | the walk's ray from the disc's centre runs along the arcs' symmetry line |
| `point_in_loop_arm` (die_tool 2, die_composed_tour 6) | the in-face walk of an arc-bearing cap a ray crosses | a schedule direction along the cap's axis-aligned normal |
| `side_at_infinity_vertex`/`_edge`/`_order` (symmetric_plate 50, kiss_carry 14, heat_sink 1) | a line-bounded face's aim, its loop's mean | an L whose arms match has its mean at its reflex vertex: the ray passes exactly through that vertex and its two edges |
| `bool_pierce_normal_on_chart` (die_composed_tour 43; the trimmed-sphere bodies' 1/2/1/5) | a curved aim's normal (`face_outward_normal_at`) | the aim is on its surface by construction, so `‖∇F‖ − 1` is 0; the aim's certificate asks it |
| `bool_ruled_region_roots_coaxial` (cut_cylinder 44, boss_union 110, plate_param 114, die_composed_tour 22, reshaped_rod 10) | a wall root's region walk (`ruled_region`), on rays aimed at planar faces | a parallel leg against a rim, coaxial by construction |

## Fix (FLUSH, `flush/aims-ties`)

- **A planar face of lines and circle arcs aims at a weighted mean**
  (`side_reader.rs` `loop_aim`): its vertices and its arcs'
  golden-section points, the `k`th weighted `2 + cos(k·g)`. It asks
  nothing, and no sum of the weights with rational coefficients
  vanishes, so it is no symmetric point.
- **Such a face is crossed by the turn of its loops**
  (`solid_contain.rs` `crosses_by_turn`, `loop_turn`, `arc_turn`): an
  arc turns by `Arg(−conj M(m)) − Arg(−M(p) conj M(m))`,
  `M(x) = (x − a)/(x − b)`, whose one cut is the arc itself. It needs
  no chord split and no test of the crossing against a segment, so it
  asks nothing.
- **A cylinder face's root is held by the turn of its loops on an image
  of its wall** (`wall_crosses_by_turn`), where every edge is a ruling
  or a circle and is decided clear of the ray. The image is an annulus,
  `ρ(Δh)·s` with `ρ(Δ) = 1 + Δ/(|Δ| + r)`: rulings go to radial
  segments and rims to circles about the origin.
- **A curved aim's normal is its chart's gradient**, with no on-surface
  certificate (an aim only steers). The cone keeps its reader, because
  of its apex.
- **The sphere aim steps from its first arc**, not the deepest step,
  so an enclosure never hulls two tied steps.
  `geom_core::GOLDEN_SECTION` is the one spelling of the constant
  (`sphere_region.rs`, `seqgen.rs`, and the golden-section searches in
  `ring_path.rs`, `conic_oracle.rs`, `geom-brep` `extent.rs` and
  `span_reach_hunt.rs`).

After: **22** ties, all `bool_ruled_region_roots_coaxial` in
`cut_cylinder`; every other document reads 0. The `Sym<f64>`
trimmed-sphere bodies (`sweep` `sphere_aims_reader_ties`) read 0/0/0/0
against 1/2/1/5.

## What remains

- **A wall whose loops hold an oblique section.** `cut_cylinder`'s
  walls are bounded by an ellipse (the tilted cut), whose image on the
  annulus is no circle. So its roots still take the region walk: 22
  coaxial ties. There are at least two candidate readings, and the
  choice between them is a design question:
  - split the section at the root's azimuth and read each piece by
    chords inside a sector of under half a turn, which asks where the
    root's azimuth lies against the pieces;
  - a different image of the wall in which a plane section is a circle.
- **Cone, sphere-patch and torus-patch roots** still take their region
  walks. Nothing in the corpus or in the sphere bodies ties on them at
  a nominal, but no fixture stands for a symmetric cone or torus.
- **The ruled aim still steps from its edges' middles.** Moved to
  their golden-section points, it aims one of
  `split_seam_vanishing_tilt`'s walls at the seam ruling. That ray
  starts about `4·10⁷` m out and reads the side inside out
  (`an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side`).
  Nothing in the corpus ties on the ruled aim: `cut_cylinder`'s
  `curve_foot_branch` ties were the cap's.
- **The ruled aim's on-axis fallback** (`select_le_zero` on the mean's
  distance from the axis) hulls at `Interval` where the mean is on the
  axis.

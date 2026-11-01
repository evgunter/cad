---
id: the-side-readers-aims-decide-exact-ties-at-a-documents-nominal
kind: issue
title: The side reader's aims land on exact ties at symmetric nominals
status: closed
priority: P2
opened: 2026-10-19
cost: H
branch: flush/aims-ties
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
pr: 5781
closed: 2026-10-31
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

- **A planar face of lines and circle arcs aims at a point stepped in
  from its first edge** (`side_reader.rs` `loop_aim`). The point is the
  outer loop's first edge's point at the golden section of its span,
  stepped along `normal × τ` by half a lower bound on its distance from
  every other edge (`solid_contain.rs` `edge_clearance`), and by at most
  half its own arc's radius.
  - It lies in the face and off its middles, and it asks nothing.
  - It hangs on the outer loop's first half-edge, so a rebuilt body can
    aim elsewhere.
  - It replaced a first, weighted-mean aim. That aim landed in an
    annular cap's hole and off a sliced rod's sliver, which set those
    aims aside: on FLUSH review R1's off-battery bodies, 0 of 117 aims
    are now set aside, against 6 on main and more on the weighted mean.
- **Such a face is crossed by the turn of its loops**
  (`solid_contain.rs` `crosses_by_turn`, `loop_turn`, `arc_turn`). An
  arc turns by `Arg(−conj M(m)) − Arg(−M(p) conj M(m))`, with
  `M(x) = (x − a)/(x − b)`, whose one cut is the arc itself. It needs no
  chord split and no test of the crossing against a segment, so it asks
  nothing.
- **A cylinder face's root is held by the turn of its loops on an image
  of its wall** (`wall_crosses_by_turn`) where every edge is a ruling or
  a circle. The image is an annulus, `ρ(Δh)·s` with
  `ρ(Δ) = 1 + Δ/(|Δ| + r)`: rulings go to radial segments, and rims to
  circles about the origin.
  - It holds only where the root lies farther from each edge than twice
    how far the edge is off its ruling or parallel
    (`bool_wall_turn_true`). Tier 3 admits a rim tilted up to `√(2εr)`
    along the axis.
  - For an edge on its ruling or parallel, that margin is the root's
    clearance of the edge, so it never ties.
- **A face's own seam is no boundary of its turn.** The turn leaves it
  out, since its two passes cancel in the reals. In floats, a pass whose
  products vanish reads `atan2(+0, x < 0) = π` both ways, and a turned
  tube read in its seam plane misread 608, 509, 403, 456 and 352 of 7381
  points at five turns (R1; base 0).
- **What licenses a turn is now its own set, `EdgeMeets::clear`.** It
  holds the edges a face's turn may be read past: every line pass
  `edge_meets` reads, met or not (a face met at any edge never takes the
  turn, so a met line in it licenses nothing), and each curved edge read
  clear from every
  crossing, or whose carrier the ray definitely misses. `read` keeps
  meaning "decided". A seam is in neither: `curved_edges` no longer
  inserts it unread, and `every_bound_read` skips it by name. An edge
  read past its span's end (`Passes::End`) stays in `read` and is not
  `clear`.
- **A cylinder's or a sphere's aim takes its chart's gradient as its
  normal**, with no on-surface certificate (an aim only steers). A
  torus's and a cone's keep their face's reader, which refuses outside
  the ring convention and at the apex.
- **The sphere aim steps from its first arc**, not the deepest step, so
  an enclosure never hulls two tied steps.
- **`geom_core::GOLDEN_SECTION` and `GOLDEN_ANGLE` are the one spelling
  of their constants.** Every golden-section search and every golden-angle
  sample in the tree reads them. `π(3 − √5)` equals `GOLDEN_ANGLE` in
  `f64`, so the batteries' inputs keep their bits.

- **The fix pass moves which seam-split poses meet the P1** (R1's
  delta): 6 poses that build on main now refuse with `NegativeVolume`,
  15 that refuse on main now build, and the census total goes from 65
  to 58 of 384 at 1e-9. That is the P1's aim luck, refused rather than
  shipped: no wrong side builds, and the P1 row carries the defect. My
  census reads 50 of the 58 shared with main, so 8 newcomers, not 6;
  R1's 6 and 15 against 65 would give 56, so the two counts disagree
  by two and I have not reconciled them.

After, on the merged head: **22** ties, all
`bool_ruled_region_roots_coaxial` in `cut_cylinder`; every other
document reads 0. The `Sym<f64>` trimmed-sphere bodies (`sweep`
`sphere_aims_reader_ties`) read 0/0/0/0, against 1/1/2/5 on main.

## What remains

Each has its own row on this slate:

- `a-wall-root-bounded-by-an-oblique-section-still-walks-and-ties-at-the-nominal`:
  `cut_cylinder`'s 22, a design question with two readings stated.
- `cone-sphere-patch-and-torus-patch-roots-still-walk-their-regions`.
- `the-ruled-aims-on-axis-fallback-hulls-at-interval`.
- `a-torus-face-aims-at-a-fixed-chart-point-that-a-symmetric-pose-holds`.
- `the-census-ladder-asks-at-symmetric-points-unmeasured`.
- `an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side`
  (P1, live on main). The ruled aim stays at its middles because of it:
  moved to its edges' golden sections, it steered one more wall aim
  onto a seam ruling.

## Resolution

Closed by PR 5781 (2026-10-31). The side reader's aims and the readers
that cross them no longer decide exact ties at a document's nominal,
except the 22 `cut_cylinder` wall roots that "What remains" leaves to a
design question:
- `revalue_corpus::reader_ties_at_the_nominal` reads 22, all
  `bool_ruled_region_roots_coaxial` in `cut_cylinder`;
- the `Sym<f64>` trimmed-sphere bodies read 0/0/0/0, against 1/1/2/5 on
  main.

No body changed side. On both reviewers' bodies, at three ε, no ray,
point or edge ray reads wrong, and no aim moves Out↔In.

- **The changes.**
  - A planar face of lines and circle arcs aims at a point stepped in
    from its first edge, inside the face by construction, and is
    crossed by the turn of its loops.
  - A cylinder wall's root is read by the turn of the wall's annulus
    image, behind a rim guard that checks each edge lies on its ruling
    or parallel (`bool_wall_turn_true`).
  - A face's own seam is no boundary of its turn. `EdgeMeets::clear`,
    not `read`, licenses a turn.
  - Every golden-section search and golden-angle sample in the tree
    reads `geom_core::GOLDEN_SECTION` / `GOLDEN_ANGLE`.
- **Review.** A dual review (concurrent pair, one frozen head) found one
  MAJOR, by one reviewer, run and confirmed by a mutant. The first
  turn readers took a face's own seam as an edge decided clear, so a
  root on it read outside the face: `point_in_solid` misread 352–608
  of 7381 points in a turned tube's seam plane, against 0 on main.
  The fix pass took the seam out of the turn and gave the turn its own
  licence. Both delta checks approved, and a last pass took their
  notes.
- **The P1 moved, but no side shipped wrong.** The fix pass moves which
  seam-split poses meet the P1: 58 of 384 against main's 65, 50 of
  them shared. That is the P1's aim-luck mechanism, and each of those
  poses refuses at the gate.

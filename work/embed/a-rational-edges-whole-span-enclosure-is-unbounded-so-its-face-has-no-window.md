---
id: a-rational-edges-whole-span-enclosure-is-unbounded-so-its-face-has-no-window
kind: issue
title: A rational edge's one-interval enclosure over its whole span is unbounded, so its face's window is not a finite rectangle and two valid f64 cone cavities refuse before any cell is read
status: closed
opened: 2026-11-18
priority: P0
cost: M
closed: 2026-11-22
---

Traced by lane `embed/cavity-trace` (2026-11-18) for
`the-f64-cavity-gate-refuses-thirty-two-valid-shells`. Main `da49c63f`,
f64, the cavity asked through a local, uncommitted `CavityGate` switch,
with `topo::clearance` instrumented (uncommitted).

## Evidence

Two valid f64 cavity rows refuse `Unsupported { carrier: "a face whose
carrier window is not a finite rectangle", face: 1v1 }` before any cell
pair is read (an empty receipt):
- `sf2b_r2_probes::r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`;
- `shell7_seam_corner::a_three_quarter_turn_cone_frustum_hollows`.

Both are cheap (0.2 s). They have refused this way since 2026-11-13.

## Trace

Face `1v1` is in each case a plane cap (`x = 0.0078125` and
`x = −0.05`) with an ordinary frame. Its window is
`(−∞, ∞) × (−∞, ∞)`, because `window_of` reads it off `boundary_of`'s
box, and that box is all of space.
- **The edge that makes it so** is `8v1`, the cap's hyperbola: the cut
  of the cone by a plane off its axis. It is stored as a rational
  quadratic NURBS, weights `[1, 1.0722, 1]` and `[1, 1.0638, 1]`.
- **`boundary_of` encloses each edge by one interval evaluation** of its
  carrier over the whole span (`carrier.eval([t0, t1])`). For a spline
  curve that evaluation comes out unbounded, rational or not: at
  `t = [0, 1]` every basis function's enclosure reaches 0 at one end
  of the span, so the weight sum `Σ Nⱼ·wⱼ` holds 0 and the quotient is
  the whole line (measured below; the evaluator's row is
  `nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t`).
- **Every other point of the face is finite**: its vertices and its
  other edges.

**The reading it wanted** was there already: an edge's region over its
span is its carrier's cover (`geom::Curve3::cover_over`, read in topo
through `cover::arc_cover`), which for a spline is its control net met
with the hull of its pieces' nets. With positive weights the curve lies
in each, and both are finite.

**Not checked:** whether this is the shape behind the at-rest gate
lane's census of 59 workspace tests refusing the same text. Any face
bounded by a rational edge whose whole-span denominator interval reaches
zero would refuse so. The row named here is the cheapest evidence.

## 2026-11-18 — measured and fixed (EMBED, lane rational-window)

**Where it starts.** `NurbsCurve3::eval` at `t = [0, 1]` on a one-span
quadratic: the basis encloses `[0, 1]`, `[0, 2]`, `[0, 1]`, so
`Σ Nⱼ·wⱼ` holds 0 and every coordinate is the whole line at `Trv`, for
middle weights 0.5, 1, 1.0722 and 2 alike. That is the evaluator's
open row `nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t`
(contributor 1), not a weight effect, and every spline edge read by
`boundary_of` over a whole knot span had it. The CLEAR row
`boundary-of-evaluates-a-spline-edge-over-its-whole-span` (2026-10-28,
unmeasured then) is this defect.

**What landed.** `boundary_of` reads each edge's region off its cover
over its span, topo's one reading of an edge's region
(`cover::arc_cover`, so `geom::Curve3::cover_over`), and takes the
cover's own box corners (`Cover::box_in` at `BoxAxes::World`) into the
points it hulls, as it takes the vertices. It does not read
`Cover::aabb`: that ends in `Aabb::padded`, which steps every side out
by an ulp even at a zero pad. The evaluator is unchanged.

**What the box is, against the old reading.** `boundary_of` lifts the
carrier through `certified_lift` at every body scalar, so the cover is
read at `Interval` on lifted data whatever the body's scalar, and the
f64 lane reads the same windows as the `Interval` table below.
- On a segment and on circle arcs of radius ½ and 1 (a quarter, a
  half, a short arc, more than a turn, named either way round) the
  cover's box is the old whole-span evaluation, bracket for bracket.
  At a crest whose radius's square rounds (0.7, 0.9, 1.3) it reads one
  or two ulps out, `√(p² + q²)` against `r·cos`
  (`cover_box_against_the_whole_span_eval`; filed as
  `onecopy/an-arcs-cover-reads-its-crest-a-rounding-out-where-the-radius-squared-rounds`).
- On a spline edge it is finite where the evaluation was the line.
- On the tour walk at f64, window by window against main `c5bc653ae`
  (a one-time figure, 2026-11-18, from an uncommitted print of each
  window in `window_of`; main's walk prints no windows), over 57 of the
  85 bodies: 89 window ends 1–4 ulps wider, 6 wider by under 1e-12
  relative, 3 moved off `0` by at most 4.4e-16 (sheave, the lily's
  rootstock), 63 ends 1–4 ulps narrower, and 151 narrower by more (the
  rocker's `±3.53 → ±2.5`, where the evaluation overshot a spline).

**Measured** (main `45023f4e3` against the branch merged onto it, f64,
2026-11-21; re-run on main `0cae9969f`, every figure the same):
- the cavity rows, through the uncommitted `CavityGate` switch
  (`origin/embed/cavity-trace-measure`'s `shell.rs`): the conical wedge
  and the three-quarter frustum have windows on every face and refuse
  later, `Uncertified { 1v1, 4v1, Edge(8v1), Departure }` (wedge 14
  candidates, 100 discharged, 2 refused, 88 splits; frustum 14, 161, 2,
  149), the plane cap against the cone wall across the hyperbola, both
  faces `Structural` at depth 0
  (`curved-presentation-reads-plane-and-cylinder-images-only`, re-banded
  P0 for it). The same 14 fail on main and on the branch otherwise; the
  other ten valid rows and the two crossing pins keep their verdicts,
  their receipts moving by up to 28 discharged cells;
- `self_intersection_proves_injectivity`: 49 of 49 on main and 50 of 50
  on the branch (the radius-0.7 drum added) at the default ε, 1e-6 and
  1e-12. The collinear-cap drum refuses 210 cells on both. It held on
  the branch at `c5bc653ae` by the alignment of its halvings with the
  half-circle, which PR 7131's frames moved
  (`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`);
- the tour walk, main's `demos/tour/examples/embed_walk.rs` (`grep '^WALK'`,
  diffed against main's own): 78 of 85 hold on main and on the branch,
  and no verdict differs at any stop; the seven that refuse are the same
  on both (the three bench assemblies' recorded contacts, and the three
  die bodies and the lily's lantern, departure). 13 bodies' counts move:
  candidates 6,929 → 6,925 (`tiltedcut_above` 150 → 146), discharged
  499,153 → 498,677, splits 492,679 → 492,207, refused 230 → 230. The
  largest: `lily_bud_b` 676 → 537, `lily_bud_a` 733 → 606, `budfillet`
  3,423 → 3,547, `projectbox/cutaway_below` 6,815 → 6,717,
  `cutaway_above` 2,996 → 2,917, `lily_bud_c` 794 → 726,
  `tiltedcut_above` 1,826 → 1,782, `fivewall` 15,090 → 15,070;
- every stored edge of every tour body lies inside its cover's box:
  3,815 edges, worst escape 0
  (`demos/tour/examples/edge_cover_soundness.rs`, 297 s with the tour's
  build).

**The census's 59.** On the at-rest gate's branch (`origin/embed/gate`
`9d271e43e`) with this fix's first form, the census's named row for its
37-test cylinder group (`mesh`
`lift_probes::the_spline_section_bodies_read_lifts_that_close`) moved
from this refusal to a departure refusal at a spline edge,
`Uncertified { 4v1, 5v5, Edge(17v1), Departure }`. The other 22 were not
run.

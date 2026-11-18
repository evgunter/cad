---
id: a-rational-edges-whole-span-enclosure-is-unbounded-so-its-face-has-no-window
kind: issue
title: A rational edge's one-interval enclosure over its whole span is unbounded, so its face's window is not a finite rectangle and two valid f64 cone cavities refuse before any cell is read
status: open
opened: 2026-11-18
priority: P0
cost: M
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

**What landed.** `boundary_of` hulls each edge's cover over its span,
topo's one reading of an edge's region (`cover::arc_cover`, so
`geom::Curve3::cover_over`), instead of one evaluation. The evaluator
is unchanged.

**Measured** (main `ff2319654`, f64, the cavity asked through the
uncommitted `CavityGate` switch):
- the conical wedge and the three-quarter frustum now have windows on
  every face, and refuse later:
  `Uncertified { 1v1, 4v1, Edge(8v1), Departure }`, the plane cap
  against the cone wall across the hyperbola, both faces `Structural`
  at depth 0 (`curved-presentation-reads-plane-and-cylinder-images-only`);
- the other ten valid cavity rows keep their verdicts, and so do the
  two crossing pins; receipts move by a few cells either way;
- `self_intersection_proves_injectivity`: 49 of 49 rows on main, 48 on
  the branch at the default ε, 1e-6 and 1e-12, the drum's refused count
  re-pinned 282 → 564 (`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`);
- the tour walk: 82 of 85 hold on main, 81 on the branch. `budfillet`
  refuses on that same cell-side row. Totals: 6,929 → 6,925 candidates,
  479,866 → 476,763 discharged, 473,162 → 470,075 splits, refused
  0 → 12, and 40 bodies' counts move.

**Width.** On a spline edge the cover is finite where the evaluation
was the whole line. On a closed-form edge it is narrower where the
interval evaluation overshot (a fillet's arc: `max x` 1.16 → 1.0) and
one or two ulps wider where the evaluation was exact, since each
support is read in interval arithmetic.

**The census's 59.** On the at-rest gate's branch with this fix, the
census's named row for its 37-test cylinder group
(`mesh` `lift_probes::the_spline_section_bodies_read_lifts_that_close`)
moves from this refusal to a departure refusal at a spline edge. The
other 22 were not run.

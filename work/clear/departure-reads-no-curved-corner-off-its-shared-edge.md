---
id: departure-reads-no-curved-corner-off-its-shared-edge
kind: issue
title: The injectivity check reads a curved corner only through a shared edge of a curved chart or a plane's arc, so a pair touching at the corner's vertex alone, or across a plane's line edge that meets an arc, refuses; the tee refuses so
status: open
opened: 2026-10-29
priority: P0
cost: M
---

## Finding

From CORNER (`departure-reads-no-curved-corner`).

`topo::injective`'s corner presentation (`present_corner`, `reading`) reads a corner where a curved edge meets another in two cases only: when the shared edge is a curve `present_curved` reads, the other edge straight in the chart; and when the shared edge is a coordinate line of a cylinder's chart, the other a harmonic chart curve. Three kinds of corner fall outside them. `beside_curve` refuses each `Structural`, so a pair whose two faces both meet one refuses at once.

- **A pair touching at the corner's vertex alone.** `corner_of` needs one of the corner's two edges on the shared cell. A face pair that shares only the vertex has neither, so no zero side holds the vertex. The iso corners answer this with `Zero::Point`; a curved corner has no such reading.
- **A corner of two curved edges.** The curve reading needs the other edge straight in the chart (`straight`). The line reading needs the read edge a coordinate line.
- **A plane face whose shared edge is a line meeting an arc.** The line reading (`present_line_corner`) is built on curved charts only. A plane has `present_plane`'s affine line presentation, which needs a span check (`self_intersection_span`) that a line ending at the corner fails.

## Evidence

The boolean tee `crossing_equal_rods` at spins `(0, 0)`, `B` from `x = 0.2`, pinned by `a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone` in `crates/sweep/tests/self_intersection_proves_injectivity.rs` (which pins `refused == 9`). It refuses `Uncertified { Departure }` with nine refusals. The CORNER review instrumented them: all at depth 0, both faces' whole windows.

- **Six vertex-only pairs.** These are the walls against the mitre and against each other:
  - at `(0.2, ±0.98, 0.2)`, where `B`'s cap circle meets the mitre plane;
  - at `(1, 0, 1)`, where `A`'s seam line meets the junction.
- **One plane–plane pair across a `Line`.** `A`'s mitre against `B`'s cap, along the line where the mitre plane cuts the cap (in the review's run: mitre `5v9`, cap `6v5`, edge `10v3`). Its ends are corners where the line meets each face's arcs.
- **Two cylinder–mitre pairs across `Ellipse` edges.** These are each of `A`'s wall halves against the mitre, across its arc of the mitre's ellipse (`4v1`–`5v9` on `3v5`, `7v1`–`5v9` on `14v1`). The likely cause is a corner of two curved edges, which no reading reads:
  - the mitre's two ellipse arcs meet at `(−1, 0, −1)`, on `A`'s seam;
  - on the wall, the mitre's arc gives way to the junction's arc of the same ellipse at `(0.2, ±0.98, 0.2)`.

  At the root the cell also holds three or more walls in reach, which no corner reading takes either.

The junction never reaches the node plane `x = 0` (`x0 > 0` keeps the walls' tangent points out of both solids), so none of these refusals is the second-order ground.

## What a fix needs

- For the vertex alone: a point zero for a curved corner. The quadrant reading generalises if the line reading's rectangle is cut at the vertex on the moving axis too, and the curve is decided to leave into that quadrant on both axes; then `Zero::Point`'s two derivatives apply.
- For two curved edges: the other edge's height over the continued curve, decided along it, in place of `leaves_graph`'s straight-line rate. Where the two are arcs of one closed form (the ellipse split at a vertex), `continued` reads them as one curve.
- For a plane's line edge: the line reading through `present_plane`'s affine map, with `leaves_line`'s functional the line's face-side normal and the span check dropped at the vertex end.

## Further evidence (SEAMDIAG, 2026-11-01)

Lofted caps meet the second bullet, a corner of two curved edges on a
plane, at depth rather than at the root. `lily_leaf_a`'s and
`lily_leaf_b`'s caps are bounded by two circles meeting at the lens
tip. The Klein loop's caps are bounded by four arcs of one circle,
meeting where the wall seams land. A cap cell at such a vertex is
`beside_curve`'s `Structural`, and one beside it is `Local`. The pair
still splits, because the wall presents. Today it refuses at depth 40
(`work/clear/departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget.md`
has the cells). There the binding obstacle is the wall's zero-side
residual, so this row's corner reading alone would not certify those
pairs.

## Further evidence: eight finished tour bodies (CLEAR, lane clear-gate, 2026-11-12)

Measured for `the-at-rest-gate-certifies-embedding` (its Measured
section), at main `348ec09727`, f64, the body-level question at the
default depth and 65,536 pairs. These tour bodies pass tier 3 and
refuse `Uncertified { arm: Departure }` at depth 40. Their pairs have
this row's shapes (not traced to the cell):
- **Plane × plane across a `Line`**: `chute` (faces 1 and 6, edge
  `20v1`, 0.06 s) and the lily's three buds (faces 1 and 6, edge
  `12v1`, 0.18–0.31 s). This is the third bullet's shape, a plane's
  line edge meeting an arc.
- **Plane × sphere at a vertex alone**: `diefillet` (1.5 s) and
  `heatsink5`, `7` and `9` (1.8–2.2 s, about 29,000 pairs each). This
  is the first bullet's shape.

Re-banded P0: the at-rest gate (P0) refuses these bodies at their
doors until this row lands, and a blocker inherits the band of what
waits on it.

## Further evidence: a plane cap's two semicircles (CLEAR, lane clear-planecap, 2026-11-12)

The tour's seven plane-cap bodies (`the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle`)
have this row's second bullet on the cap. Each cap is a disc on two
arcs of one circle, meeting where the walls' seams land, and a cap
cell at that vertex is `beside_curve`'s `Structural`. Their refusal
was not this row's, though. The carrier arm decides such a pair with
no presentation of the cap, and it lacked only a tight reading of the
cap's plane along the wall's side. That reading has landed, and the
seven certify with the cap still unpresented at its vertices.

A reading for a corner of two arcs of one closed conic would still
pay where no carrier arm applies, as on a curved face whose two
arcs meet on a non-planar partner. In that reading, the corner's
other edge is the read edge's own continuation, so `continued` holds
both.

## Further evidence: the plane × sphere bodies certify (CLEAR, lane clear-pairs, 2026-11-12)

`diefillet` and `heatsink5`, `7` and `9`, the "plane × sphere at a
vertex alone" bodies above, hold once `present_chart` reads a
sphere's pole as a collapsed side of its chart
(`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`,
its Fix): 4,133 and about 16,100–16,800 pairs, against 20,885 and
about 29,000 refused before. The vertex those pairs share was the
sphere patch's pole, which the presentation read as a loose vertex.
So this row's first bullet no longer has tour evidence; `chute` and
the lily's three buds (the third bullet) still refuse.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

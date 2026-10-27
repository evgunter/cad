---
id: edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short
kind: issue
title: edge_extent levers a NURBS edge by its chord, 4-10x short of the point-set diameter on a steep hyperbolic arc, which can turn a Positive verdict into a definite Zero once such an arc rests
status: closed
opened: 2026-10-25
priority: P0
closed: 2026-10-27
pr: 5875
branch: offset/edge-extent-true-bound
---


`crates/geom-brep/src/certify.rs` `edge_extent` reads a `Curve3::Nurbs`
edge's lever arm as its chord, documented as a certified lower bound on
the point-set diameter "for the open fitted branches the zip mints".
The plane×cone hyperbola is now minted as a NURBS arc
(`ConicBranch::arc`), and on a steep arc the farthest pair of its points
is the vertex and an end, not the two ends. Measured
(`crates/geom-brep/tests/plane_cone_arc.rs`,
`the_chord_on_a_steep_arc`, the plane `x = 0.3` on cones about `z`, the
window symmetric about the vertex): `α = π/6`, `y = ±2`, ratio 1.00;
`α = 0.1`, `y = ±2`, chord 4 against a diameter of 17.28, ratio 0.23;
`α = 0.05`, `y = ±6`, chord 12 against 114.2, ratio 0.105.

**A short arm is not the safe side.** `edge_extent`'s own doc says a
smaller arm "only escalates more", but the arm is a lever:
`Margin::levered(x, extent)` shrinks with it, so a reading that is
Positive at the true diameter can read Zero at the chord. Zero is a
definite verdict wherever a predicate reads it as a stand-in,
coincidence or tangency, and the same doc gives that failure for the
collapsing chord of a closed edge (a full-period 90° rim read
"definitely Smooth"). The consumers that lever at this extent include
`topo::validate`'s wedge test (its departure arm), `topo::census::curved`'s
`tangent_on_interior`, `geom_brep::conic_bend`'s `ConicBend`,
`topo::boolean::carrier_pair` (the extent ball of a rest locus),
`topo::boolean::contact_verify`, `topo::replace_face`'s edge levers, and
`certify.rs`'s spline lanes (transversality and the uniqueness tube's
size). `validate`'s departure and `conic_bend` take no NURBS carrier
today; the others do. So this is possibly unsound, not conservatism,
from the moment a steep open-conic arc can rest in a body. None rests
yet: the axial offset door refuses the arc at its gate
(`ReplaceFaceError::TogetherConicArc`), and OFFSET's
`a-classified-pairs-section-states-the-bounds-that-retire-the-tube`, the
unit that would let it rest, is parked on this row.

A distance between any two points of the edge is a lower bound on the
diameter too, so the chord could be maxed with the distances from the
carrier's mid-parameter point to each end; on a single-span conic arc
the control polygon's hull gives an upper bound to read the gap
against. Found by the OFFSET conic-arc unit (branch
`offset/conic-arc-and-axial-section`); the sign of the error was
corrected by that PR's dual review.

## Closed

PR 5875. The edge extent is two true bounds, and every caller names
its side: `geom_brep::edge_extent_inf` (≤ the point-set diameter and so
≤ the arc: the chord, the circle and minor-axis folds, and for an
ellipse, spiric or spline the farthest pair among its quarter-parameter
points) and `geom_brep::edge_extent_sup` (≥ the diameter: exact on a
circle, the major circle's arc on an ellipse, a stretch or diagonal
bound on a spiric, the control net's farthest pair on a spline). A
reading that acts on a Zero levers at the bound from above:
`topo::validate::side`'s wedge, `census::curved::tangent_on_interior`,
`boolean::carrier_pair::rest_locus`'s ball, `contact_verify`'s tangent
table, and `boolean::fate::leaving` (a sweep hit, which levered the
chord for every kind). A reading that acts on a definite nonzero sign
levers at the bound from below: `certify.rs`'s spline lanes,
`replace_face`'s section and corner roots, and `ConicDeparture`, whose
`conic_bend_side` witness needs an arm no longer than the arc. On the
steep arcs measured above, the bound from below now reads 0.99999 to
1.0 of the diameter and the bound from above 1.0 to 1.0012
(`plane_cone_arc::the_extent_on_a_steep_arc`). The row that goes red at the
chord lever is
`boolean::carrier_pair::extent_rows::a_steep_spline_rests_on_the_pair_its_ends_separate`.

Residue, filed: `work/band/blend-battery-levers-a-chains-g1-from-below`,
`work/twice/wedge-reach-calls-a-reach-short-of-its-faces-the-safe-direction`,
and evidence on CONTACTHOLD's
`conic-ties-outside-the-vertex-on-face-pass-still-read-first-order`
(the sector walk's non-plane `On` tie at the bound from below).

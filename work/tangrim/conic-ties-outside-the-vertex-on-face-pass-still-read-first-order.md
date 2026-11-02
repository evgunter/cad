---
id: conic-ties-outside-the-vertex-on-face-pass-still-read-first-order
kind: issue
title: Outside the vertex-on-face pass a conic tangent to a face still reads On at first order: the sector passes, recl's flank readings, curved faces and fitted edges
status: closed
opened: 2026-10-10
priority: P2
cost: H
refs: [boolean-conic-side-code-zero-is-first-order, vtxfac-tangent-sector-should-descend-per-bound]
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
closed: 2026-11-01
pr: 6253
branch: tangrim/sector-passes-descend
---


## Finding

`boolean-conic-side-code-zero-is-first-order` reads a conic's
first-order tie with a PLANE one order down in the vertex-on-face pass
only (`sectors::pierce_side`, called from `vtxfac::classify_vertex_on_face`
and `vtxfac::partner_side`). Every other reading of a conic bound still
returns `On` at the tie:

1. **The sector passes** (`sectors::side_code` through `plane_side_code`:
   `pair_codes`, `wedge_classes`, `cone_side`) **and recl's flank
   readings** (`recl::flank_key`, `resolve_edge_edge`'s membership,
   `wedge_is_reflex`'s `bool_wedge_reflex`, which before INTENT's E
   refused the `flush` posture's union at margin `-6.1e-17`, the arc's
   tangent). Descending there was measured on the fix's branch: 15 sweep
   rows (`join2_r2_probes`, `rest_zip_admission`, `reach_continuation`,
   `rest_nested_strut`) went from building to `CurvedBooleanUnsupported`
   in `recl::require_same`, because the on-edge and tangent machinery
   keys on first-order ties: an arc tangent to a plane reads as an
   on-edge event there, and its neighbours' records are built on it.
   Descending these needs those gateways keyed on first-order ties
   (as the vertex-on-face pass's whole-sector lump now is,
   `PierceSide::tied`) with the descended codes carried beside them.
2. **A curved face.** Against a cylinder, sphere, cone or torus a tie
   means the bound rides the face to first order, and which way it
   leaves is the two carriers' relative curvature along the tangent
   (`sectors::tangent_relative_side`'s quantity), which needs the face's
   second fundamental form at the vertex in the reading, and a bound on
   the face's third-order remainder for a certified verdict.
3. **A fitted (NURBS) edge's chord** (`Reach::Extent`): a secant. A
   Zero says the far end is in the band; where the edge goes between is
   not read. Its descent needs a certified jet and curvature bound of
   the fitted carrier.
4. **`recl::flank_rep`'s offset off the common line** (`bool_flank_offset`):
   a conic whose tangent runs along the line has a zero projection and
   refuses, though its bend carries it off the line.
5. **The vertex-on-face pass's tangent sectors**: a sector tangent to
   the pierced face whose bounds both tie lumps whole
   (`sectors::tangent_lump`), and a conic bound's own descended side is
   overwritten by the lump. Reading such sectors per bound instead
   breaks `wall_face_tangent_reach`'s subtract (`BisectorSide`: the
   wall sector's bisector rides the tangency locus between two bounds
   read `Out`), and is
   `work/tang/vtxfac-tangent-sector-should-descend-per-bound.md`'s.

**Reproduction (both reviews of PR 4575, executed).** The tilted dip
(`an_arc_tangent_at_a_boolean_vertex.rs`'s `tilted_dipping_round`, α =
0.3 or 0.6) with its rim vertex on the slab's top EDGE (slab
x ∈ [2, 5], y ∈ [−6, 6]) reaches item 1: the arc's tie is read by the
vertex-edge pass, not the vertex-on-face one, and every op refuses
`Join(UnpairedLooseEnds { count: 2 })`, the wrong germ the vertex-on-face
pass no longer makes. Descending in `side_code` as well moves it to
`ClassificationInvariant "odd number of surviving crossing records at a
vertex pair"`, so the descent there needs the gateways keyed first.
Printed by `review_tilted_dip_off_the_vertex_on_face_pass` (`--ignored
--nocapture`). `a_tangent_arc_at_other_boolean_vertices.rs`'s
`a_rising_round_tilted_onto_a_slab_edge_keeps_its_operands` goes red if
`side_code` descends without them.

A direction-only reading is listed so the next sweep does not re-find
it: `sectors::arc_side` (`bool_cone_arc`) reads a bound's direction
against a great arc of the cone of directions, and a conic tangent to
that arc ties there whatever its bend. Whether that can decide a verdict
is unmeasured.

Fixture: `crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs`
(the `flush` and `corner` postures reach the vertex-vertex pass's
`pair_codes` with the arc tied; their refusal is not this one,
`a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex`).

## The tie's lever is the edge's extent from below (OFFSET, 2026-10-27)

OFFSET's `edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short`
split the edge extent into `geom_brep::edge_extent_inf` and
`geom_brep::edge_extent_sup`. `ConicDeparture` (and so `Reach::Conic`)
keeps the bound from below, because `conic_bend_side`'s witness and the
sector walk's other `Reach::length` readings (`sagitta_charge`'s reach,
`crossing`'s location) need an arm no longer than the arc. So the
first-order reading that ties here (`side_code`'s `enters_material` at
`Reach::length`) is levered short of the edge's diameter on a steep
arc, by up to the ratio that unit measured on the steep plane×cone
hyperbola (the chord there is 0.1 to 0.23 of the diameter; the bound
from below now reads within 0.1% of it on those arcs). That matters only
where the tie is read as `On` outright, which is this item's ground:
the plane face's descent levers its own `Zero` at `BendBounds::Planar`'s
`reach` (an upper bound), and a spline's descent escalates. Whatever
replaces the first-order `On` here should lever its `Zero` at
`edge_extent_sup` rather than at `Reach::length`.

## A curved face tangent to a rim at its vertex (2026-10-28, SPLITDOOR `splitdoor/boolean-strut-order`)

Item 2 is now in the way of a reading that exists. PR 5968 orders a
pierce ring's two germs that are tied along one tangent by their
bends. In a curved pierced face that bend is
`geom_brep::implicit_section_acceleration`'s curved-reference term,
which is unit-pinned (`crates/geom-brep/tests/section_acceleration_review.rs`)
but has no end-to-end Boolean.

Every pose that reaches the term has a rim tangent to the curved face
at the vertex: the face's trace on the rim's face is tangent to the rim
there. So the crossing layer meets the rim's arc tangent to the face
before the ring reads anything.

Measured at ε = 1e-9, with `a_split_through_a_wrap_edges_vertex.rs`
`a_ball_tangent_to_a_two_arc_holes_rim_is_held_or_builds_sound`:

- A: the two-arc holed block, turned 0.4 about x.
- B: the ball of radius 3 touching `z = z₀` from below at the turned
  vertex, poles along x.
- A − B, A ∩ B and A ∪ B all refuse `CurvedPierceUnsupported { operand:
  A, face: <the sphere>, edge: <a bottom rim arc> }`.

That test holds the refusal, and goes red into its full checks (tiers
1–3, 3′, and the operands' volume identities) when the arc's tie reads
one order down.

**2026-11-01 (ONEREAD, PR 6247):** `wedge_classes`, `cone_side` and `arc_side` (`bool_cone_arc`) are deleted. The vertex-vertex rows read each own edge through one gathered `link::Cone` (`boolean::vtxfac::cone_classes`), its signs a point's distance from a plane (`link_plane`, `link_wedge`, `link_ahead`, `link_arc`). Item 1's list keeps `pair_codes` and recl's flank readings; the direction-only arc reading is gone.

## Closed

2026-11-01, PR 6253 (`tangrim/sector-passes-descend`). Every sector
pass reads a tied bound one order down through `bound_side::Tie::descend`
(`sectors::bound_read`): item 1's sector passes and recl's flank
readings, with the on-edge gateways re-keyed on the tie (the graze and
on-entry bisector guards, the edge-edge membership); item 2's curved
face, by `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` against the face's own surface
(`FaceBend`, carried on `BoolSector`); item 5's tangent sectors read
their bounds' own descended sides. CONTACTHOLD's hazard set builds as on
main (most of its 15 rows were the descent reading a curved face's
tangent plane as a plane, item 10 of the parent). Item 3 (a fitted
edge's chord) refuses typed (`ConicBend`); item 4 (`flank_rep`) and the
witness are residue on `the-edge-edge-rule-descends-a-flanks-projection-not-its-bound`
and `a-leaves-verdict-carries-no-witness-point`.

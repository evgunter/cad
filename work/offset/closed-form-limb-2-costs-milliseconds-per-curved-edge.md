---
id: closed-form-limb-2-costs-milliseconds-per-curved-edge
kind: issue
title: Limb 2 through the Taylor enclosing net costs 1-6 ms per circle and 8-73 ms per spiric edge in release; curved boolean rows run about ten times slower
status: open
opened: 2026-10-25
priority: P1
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---

Measured by the unit's implementer lane (branch
`offset/limb2-for-every-carrier`, 2026-10-25, release, workspace profile,
ε = 1e-9).

Per edge, through `EdgeCurve::certify_via` with the lane (both operands'
limb 2, `pcurve_cache::closed_form_offset_sup`):

| edge | before (samples only) | after |
|---|---|---|
| plane × cylinder circle, full turn | < 1 µs of limb work | 5.7 ms |
| plane × cylinder circle, quarter | < 1 µs | 1.3 ms |
| plane × torus spiric, `d = 0.1` | < 1 µs | 8–17 ms |

That is after cutting each Taylor span into one composite part rather than
`projected::INCIDENCE_CUTS` (a first cut that read every span eight ways
cost 76 ms per full circle and 370 ms per spiric). Rows of
`sweep::all` that build curved booleans (`a_closed_conic_in_the_partners_face`,
`a_circle_of_one_radius_elsewhere_is_no_rim`,
`a_body_built_turned_is_the_body_turned`, 13 rows) took 0.24 s in all on
`main` and 2.95 s on the branch.

Where the time goes (callgrind, one full circle): the canonical composite
~50% (`geom_core::spline::blossom::span_restrict` ~35% of the total — the
net is already in Bézier form, full multiplicity at every break, so
restricting a span to its own ends reproduces its row), the enclosing net
itself (`carrier_net::span_bernstein`) ~27%, and
`projected::piece_controls` ~19%, again a span restricted to its own
ends. Candidate cuts: an identity path for a Bézier span read at its own
ends in `CoeffWindow::restrict`/`SpanRow::restrict` and in
`piece_controls` (`Certification::between` already returns the source at
an exact end ratio, but caps its decoration at the weakest of the three,
so the path is not bit-identical in the decoration); a first grid graded
to the band (fewer spans where ε is coarse); or an exact sup for a
circle or ellipse against a quadric, whose residual is a degree-2
trigonometric polynomial (`harmonic::harmonic_sup`), with the net kept
for the spiric and the torus.

## Measured again on the door split (2026-10-26)

Conic unit 1's PR (branch `offset/limb2-for-every-carrier`, the door
split on Ev's ruling on PR 5689) measured the same 13 rows, release with
the workspace profile (`debug-assertions = true`), `--test-threads 1`,
three runs each, on the same box back to back. Before is `main` at
`bc80647eef`; after is the branch.

| row | before (s) | after (s) |
|---|---|---|
| `a_frustum_through_a_plate_with_a_hole_of_its_top_radius_builds` | 0.108 | 2.899 |
| `a_boss_on_a_plate` | 0.127 | 1.673 |
| `a_rim_inside_a_ball_face` | 0.108 | 1.339 |
| `a_boss_beside_a_ring_and_over_a_hole` | 0.090 | 1.332 |
| `a_plug_in_an_annulus_bore` | 0.092 | 1.133 |
| `a_ball_union_built_on_a_turned_plane_is_the_union_turned` | 0.064 | 0.985 |
| `crossing_washers_with_radii_in_band_are_no_rim` | 0.040 | 0.922 |
| `coaxial_equal_cylinders_unite` | 0.052 | 0.583 |
| `every_construction_built_on_a_turned_plane_is_the_construction_turned` | 0.022 | 0.533 |
| `a_boss_on_a_plate_at_interval` | 0.074 | 0.486 |
| `a_boss_over_a_hole_tangent_inside_its_rim` | 0.046 | 0.483 |
| `a_puck_and_its_rounding_ring_unite` | 0.041 | 0.302 |
| `a_closed_edge_in_a_cylinder_or_cone_face_refuses_typed` | 0.021 | 0.191 |
| **all 13** | **0.89** | **12.9** |

About 14.5× in all. A dual pass pays none of it (its door only mints);
an `f64` pass pays it at the attach door and again at rest, where every
at-rest door's check 2 now runs the scalar's edge door. Nothing in the
door split cut it; the candidate cuts above stand.

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

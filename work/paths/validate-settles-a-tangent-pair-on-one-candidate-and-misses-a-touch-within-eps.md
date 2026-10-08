---
id: validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps
kind: issue
title: The simplicity pair pass reads a decided tangency's single candidate and returns no contact for segments that touch within eps (line x arc, arc x arc)
status: closed
opened: 2026-10-07
priority: P0
cost: M
closed: 2026-10-07
refs: [validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc]
---


Found by the review of #4264
(`a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle`)
and pre-existing on `main`.

## The defect

When `crates/profile/src/seg.rs` decides two carriers tangent
(`carrier_line_circle` in `line_arc`, `carrier_circles_external` or
`carrier_circles_internal` in `arc_arc`, each read Zero), it takes one
candidate, the tangency point, and span-checks it alone. A Zero
clearance can be up to ε, and two carriers that close stay within ε of
each other for about `√(2rε)` either side of that point, far longer than
the Kε a span reading resolves. So when one span **definitely** misses
the tangency point while the other definitely holds it, the pair
returns "no contact", although an endpoint of the missing segment can
lie within ε of the other segment. Validation then accepts the pair as
disjoint: a silent wrong answer, not an escalation.

## Reproduction

Pair-level, calling the classifiers directly (the fixtures of
`seg::pair_contact_tests`). The circle is centred (0, 1) with r = 1,
tangent to y = 0 at the origin; offsets are in the run's ε and K. Each
returns `Ok` with no contacts at f64 and at `Interval`, at ε = 1e-9,
1e-6 and 1e-12, and the true distance is 2K²ε² to 4K²ε² (2e-16 to
4e-16 at the default ε), so the segments touch:

| pair | segment 1 | segment 2 |
|---|---|---|
| `line_arc` | line (−1, 0) → (1, 0), holds the foot | arc from angle π, sweep π/2 − 2Kε, ends 2Kε short of it |
| `line_arc` | line (−1, 0) → (−2Kε, 0), misses the foot | arc from angle π, sweep π/2 + 2Kε, holds it |
| `arc_arc` (external) | the arc above, ending 2Kε short | circle (0, −1), r = 1, from angle π, sweep −(π/2 + 2Kε) |

**Measured (the #4264 reviewer):** `line_arc` returns no contacts while
the segments truly cross or touch in 10–435 of each 300 000 random f64
draws at every ε. Every such draw is in the decided-Zero arm, and every
one is also wrong on `main`.

**Whole profile: not yet exhibited.** In every whole profile the
reviewer sketched, validation still refused, because the vertex's other
segment reported the touch. The neighbour catches it when it leaves the
vertex transversally: a vertex within ε of a curve is crossed by any
transversal segment within ε of that vertex. The candidate escape is a
neighbour that leaves the vertex close to tangent to the other curve,
for example an arc whose own tangency with that curve is read off its
span in the same way.

## Band

P0, as a wrong answer from the core of validation that a profile may
already pass. If the first step shows that every whole profile is
caught by a neighbouring pair, this item re-bands to P3 (latent
unsoundness).

## Remedy, sketched

A tangent arm can settle "no contact" from a span miss only once the
miss exceeds the tangency's reach, or by asking the missing segment's
nearer endpoint directly how far it is from the other segment. #4264
keeps the tangent arms on `main`'s rule (escalate on any in-band span
reading) so that it does not widen this hole. Its `joint` short-circuit
applies only to the secant candidates, which are the true crossings.

## Outcome

**Whole profile: exhibited, so the item stays P0.** Two holes in a
square (`rejections::holes_touching_at_a_vertex_between_two_tangent_arcs_are_non_simple`).
Hole 1 runs down the unit circle centred (0, 1) to a vertex
`E = (−3Kε, 4.5K²ε²)`, which stands within ε of the line y = 0, and
leaves `E` on a circle of radius ½ that is tangent to y = 0 further
left. Each of its two arcs misses its own tangency point with y = 0
by more than Kε. Hole 2 lies under y = 0: a rectangle (line × arc) or
a cap of the circle of radius 2 tangent to y = 0 from below (arc ×
arc). The neighbour that should have caught the touch leaves the
vertex nearly tangent to the other edge, so it falls into the same
hole.

Measured on `main` at `9d2b780fe6`, at f64 and at Interval, at ε =
1e-9, 1e-6 and 1e-12: the line × arc variant validates. The arc × arc
variant with the cap tangent at the origin does not; it escalates at
`SegmentPair((1,1), (2,0))` on `arc_span`, because the cap's tangency
with the small circle falls within Kε of `E`. With the cap's tangency
point moved 3.5Kε left, between the two circles' own tangency points,
it validates (it does so anywhere from 2Kε to 5Kε left), and that is
the row's arc × arc variant. Both variants are therefore silent-miss
witnesses on `main`.

**The same hole is in the secant arms.** A crossing at angle φ keeps
the carriers within ε for about ε/sin φ along them. That stretch is
longer than Kε once φ < 1/K. So an end that definitely misses a
shallow crossing can stand within ε of the other segment, and
`joint`'s short-circuit (and, before #4264, `main`'s `?` order) read
no contact there. The widened sweep found this in both
`line_arc`'s Positive arm and `arc_arc`'s secant arm, on genuine
arcs.

**Remedy.** A candidate that a span definitely misses settles only
that point. The pair then reads its segment ends (`seg::end_touches`).
An end touches the other segment when it lies on that segment's
carrier (`chord_side`, or the new `circle_side`) and the span holds
its projection. A definite answer either way settles it before an
in-band one escalates, by the same rule as a candidate's two span
readings.

This is sound by an interval argument. Along each carrier the distance
to the other carrier has one minimum per candidate, so each segment's
points within ε of the other carrier form stretches. The ends of a
stretch that lie inside the zone are segment ends, so two stretches
meet only where an end of one lies in the other.

**Cost.** The end reads are one-sided. An end within ε of the other
segment never reads clear. An end further off reads a touch (a
refusal) or escalates out to Kε from the carrier, and past the span's
end to Kε along a line or to
s* = 2r·asin(sin(θ/4) + Kε/2r) − rθ/2 ≈ Kε/cos(θ/4) along an arc of
extent θ: `arc_span`'s chordal compression. Measured at r = 1, that
is 1.08Kε at a quarter turn, 1.41Kε at a half, 2.6Kε at three
quarters, 14Kε at 6 rad, 490Kε at 6.275 rad and about 4 000Kε at
2π − 10⁻³, at f64 and Interval alike. So with arcs past a half turn
the reads refuse or escalate well past 20ε, which a sweep confined to
±3 rad did not see. `seg::pair_contact_tests::an_end_reads_clear_only_past_the_span_readings_reach`
pins the bound at five extents.

It is a known cost, not a regression. With every non-clear end read
logged, the `ci` suites of profile, sweep, editor-core, topo and pncad
at ε = 1e-9, and of profile, sweep and editor-core at 1e-6 and 1e-12,
plus the tour's tests and the wild corpus's run, read no end past ε
outside the fuzz and the reach probe. The arcs above 6 rad there (6.11,
6.26 and 6.28 rad) read only touches at distance 0, their own shared
vertices.

The tangent arms and the secant arms share `seg::candidates`. No change
touches how declared contact or tangent joints are recorded or
verified, so D10's hold did not bind.

**Rows.**

- `seg::pair_contact_tests`: 15 rows at f64 and Interval, including
  the issue's three fixtures and separated rows that read no contact
  without escalating.
- The two-hole profile above.
- `seg::pair_contact_tests`: band-edge rows, an end ±{0.5, 0.999,
  1.001, 2}ε and {0.999, 1.001, 2}Kε off a line, a circle outside it
  and a circle around it, at both scalars; and the reach row above.
- `seg_reach_fuzz`, a sweep with a closed-form segment-distance
  oracle, over near-tangent and shallow-secant pairs (arcs up to
  10⁻³ short of a full turn, near-concentric internal pairs) and over
  ends placed at the band's edges off another segment at crossing
  angles down to 10⁻⁶.

The 15 pair rows, the two-hole profile and the first sweep were red
on the unfixed code. The band-edge rows and the widened sweep go red
under a mutant that reads an in-band end as no touch, which the first
three pass. The secant-arm sibling and the cost are
measured in the PR.


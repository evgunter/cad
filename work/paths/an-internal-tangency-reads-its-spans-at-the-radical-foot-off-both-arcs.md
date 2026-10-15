---
id: an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs
kind: issue
title: arc_arc's internal-tangent arm reads both spans at the radical-line foot, which near-concentric carriers put far off both arcs, so arcs 1-3 cm apart refuse as a TangentialContact
status: closed
closed: 2026-10-15
opened: 2026-10-07
priority: P2
cost: E
branch: paths/internal-tangency-foot
refs: [decided-tangent-point-is-the-radical-foot, validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps]
---


Found by the review of #4276 (the tangent-pair silent-miss P0), and
identical on `main` at `9d2b780fe6`.

## The defect

`crates/profile/src/seg.rs` `arc_arc`, the `carrier_circles_internal`
Zero arm, takes its one candidate at the radical-line foot
`q = c₁ + û·a`, `a = (d² + r₁² − r₂²)/2d`. When the carriers miss
internal tangency by `g` (`|g| ≤ ε`, so the arm is taken), that point
sits about `r·g/d` off both circles. Near-concentric carriers have
`d ≈ |r₁ − r₂|` small, so the offset is far larger than `g`: at
`d = 17ε` and `g = 0.5ε` it is 0.029·r, about 2.9e7ε at ε = 1e-9.

Both spans are then read at `q` with `arc_span`, whose margin is a
chordal distance from the apex. That margin is exact only for a point
on the carrier. A point 0.029 inside the circle is nearer every apex
that faces it, so an arc that stops a centimetre short of the tangency
direction still reads `q` as held. Both arcs hold it, the candidate is
a `Tangency`, and validation refuses `TangentialContact` for two arcs
that never come within a centimetre of each other.

This is a false refusal, not a silent acceptance. Where an end of
either arc does stand within the band of the other, the end reads
added by #4276 still escalate.

## Reproduction

A 10 × 10 square with two holes. The upper hole is the arc of the unit
circle about the origin from angle `s` through 2 rad, closed by its
chord. The lower hole is the arc of the circle about `(d, 0)` with
radius `1 − d + g` from angle `−2 − s` through 2 rad, closed by its
chord. The two arcs' nearest points are their near ends, `2s` apart.

| d | g | s | arcs apart | f64 and Interval, ε = 1e-9, 1e-6, 1e-12 |
|---|---|---|---|---|
| 17ε | 0.5ε | 0.005 | 1.0e-2 | `TangentialContact { (1,0), (2,0) }` |
| 17ε | 0.5ε | 0.01 | 2.0e-2 | `TangentialContact { (1,0), (2,0) }` |
| 17ε | 0.5ε | 0.014 | 2.8e-2 | `TangentialContact { (1,0), (2,0) }` |
| 34ε | 0.9ε | 0.01 | 2.0e-2 | `TangentialContact { (1,0), (2,0) }` |
| 17ε | −0.5ε | 0.01 | 2.0e-2 | validates (`q` falls outside circle 1) |
| 1000ε | 0.5ε | 0.001 | 2.0e-3 | validates (`q` is 5e-4 off) |

The same results on `main` and on #4276's head.

## Band

P2. It is a typed error that names a contact which does not exist,
millions of ε away, on geometry the public API accepts: two arcs from
different loops drawn on what is meant to be one circle, with centres
drifted apart by more than Kε/2 and less than a few hundred ε. It
fails loud, so it is not P0.

## Remedy, sketched

Read the spans at a point within the band of both carriers. The point
midway between the circles' nearest points on the centre line carries
`g/2` to each (`decided-tangent-point-is-the-radical-foot`), which
keeps `arc_span`'s chordal reading honest. A row should pin the table
above, including the cases that validate, at both scalars.

## Outcome

`crates/profile/src/seg.rs` `arc_arc`. A tangency's candidate, in the
external and the internal arm alike, is the midpoint of the circles'
nearest points on the line of centres, `c₁ ± û·r₁` and `c₂ ± û·r₂`,
which stand `|g|` apart, so it is `|g|/2` off each carrier. Both spans
read it, the recorded contact is it, and the shared-vertex discount
and `end_in_stretch` read it. The external arm needs no measurement
for this: its nearest points are `|g|` apart by construction, and the
radical foot's offsets from the two circles sum to `g`.

The internal arm then asks a second margin, `carrier_circles_chord`:
the radical foot's mean depth inside both circles, the sagitta of the
chord they cut. An internal clearance `g` within ε bounds that depth
only by `ε·r/d`, so near-concentric carriers within ε of tangency can
cross at the ends of a chord whose half-length is `√(2g/d)·r`, 0.24 rad
either side at `d = 17ε`, `g = 0.5ε`. Past the band the pair reads
those two crossings (`Circles2::crossings`, as a proper secant) rather
than one tangency; within it the circles cross, if at all, within
`√(2rε)` of the foot, and the pair reads a tangency; in the band it
escalates.

The review found what reading one tangency there cost: a cusp at the
joint `v` of two adjacent arcs on such carriers, both arcs holding the
crossing 0.24 rad out, recorded its contact at the tangency, which the
shared vertex discounted. The PR's midpoint contact made that silent
at f64 and Interval, every ε, for `g > 0`; `main` refused it only
because its contact sat at the foot, centimetres from `v`, which also
refused every smooth tangent joint on the same carriers. Reading the
crossings refuses the cusp as `NonSimple Crossing` wherever the joint
stands, and the smooth joint validates.

Pinned at f64 and Interval, ε = 1e-9, 1e-6 and 1e-12:

- `rejections::near_concentric_arcs_read_their_tangency_on_each_carrier`:
  the reproduction table above (all six rows validate; the first four
  refused `TangentialContact` on main), and eight pairs within ε along
  the stretch, which refuse: a tangency, a touch, a crossing, or an
  escalation on `arc_span` or `circle_side`.
- `rejections::a_cusp_on_near_concentric_carriers_refuses_its_crossing`:
  eight cusps, `d` from 17ε to 1000ε, the joint at the tangency
  direction or 10ε along from it, refuse `NonSimple Crossing` (the
  first five validated at the review head; the off-direction ones
  validate under a contact at the foot); three smooth joints validate
  (`main` refuses the two with `g > 0` as `TangentialContact`, and a
  contact at the foot refuses the two at the tangency direction).
- `seg::pair_contact_tests::a_tangency_reads_at_the_midpoint_of_the_nearest_points`:
  the recorded point, internal (both orders), external and nested, and
  the chord row's two crossings.
- `seg_reach_fuzz::a_joint_reads_clear_only_where_its_arcs_cross_near_it`:
  adjacent arcs at a joint on carriers within ε of internal tangency,
  judged by `validate::judge_pair` with its discount, against an oracle
  of the arcs' crossings: none more than `3·√(2r·Kε)` from the joint
  may read clear. It finds 1614, 769 and 920 at the review head (ε =
  1e-9, 1e-6, 1e-12) and 165 on `main`.

The fuzz's `near` family draws near-concentric centres from Kε to
10⁻²·r apart, with arcs reaching across the stretch.

Left open: nested near-concentric carriers (`g ≤ 0`) cross nowhere,
but a cusp on them keeps its arcs within ε for a quarter radian and
validates, as on `main`. Filed as
`a-cusp-on-nested-near-concentric-carriers-validates-along-a-long-eps-stretch`
(P0, with the question it carries). The review's NOTE-3 is
`a-vertex-in-an-arcs-gap-within-eps-of-its-carrier-exhausts-the-rays`
(P3).

Sweep and the class row's other sites: appended to
`decided-tangent-point-is-the-radical-foot`. Filed RESTFRONT
`check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` (P3),
whose arm 5 already asks the chord ends at a tangency; check 9 has no
shared-vertex discount (arms 1 and 2 report every vertex contact
between its two loops), so it does not share the cusp's hole.

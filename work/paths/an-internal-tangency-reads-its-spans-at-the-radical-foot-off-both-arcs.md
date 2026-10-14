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

`crates/profile/src/seg.rs` `arc_arc`: a decided tangency's spans each
read the candidate's radial projection onto their own carrier
(`radial_foot`, the projection `touches` already took), the carrier's
point nearest the other circle. The internal arm's contact is the
midpoint of the foot's two projections, `g/2` off each carrier, so
`end_in_stretch`'s `nearer_end` and the shared-vertex discount read a
point beside both arcs rather than centimetres inside or outside them.
The external arm keeps its point `c₁ + û·r₁`, which is on circle 1
and `|g|` off circle 2 (no amplification: over 2·10⁵ draws of radii
0.1–3 at ε = 1e-9 the radical foot's offset is at most 1.02·|g| and
the arm's point 1.03·|g|, both rounding), and reads its spans through
the same projection, so both arms read a carrier point.

The arm cannot read "no contact" for arcs within ε by this change: a
span's definite miss of its own carrier's nearest point still settles
only through the end reads (`end_touches`), and that argument
(`end_touches`'s doc) assumes the candidate is each carrier's point of
least distance to the other, which the projection now is exactly.

Pinned by `rejections::near_concentric_arcs_read_their_tangency_on_each_carrier`
at f64 and Interval: the reproduction table above (all six rows
validate; the first four refused `TangentialContact` on main at ε =
1e-9, 1e-6 and 1e-12) and seven near-concentric pairs whose arcs come
within ε along the wide stretch where such carriers stay within the
band (to about `√(2(ε + g)/d)` rad either side), holding the tangency
direction or not: they refuse `TangentialContact`, `NonSimple Touch`,
or (nested carriers) escalate on `circle_side`, as on main. A mutant
that skips the end reads for an arc × arc tangency turns that row red,
and `seg_reach_fuzz` (1,558 misses at EFFORT = 1). The fuzz's
near-concentric family now draws centres from Kε to 10⁻²·r apart, with
arcs reaching across the wide stretch.

Sweep and the class row's other sites: appended to
`decided-tangent-point-is-the-radical-foot`. Filed RESTFRONT
`check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` (P3).


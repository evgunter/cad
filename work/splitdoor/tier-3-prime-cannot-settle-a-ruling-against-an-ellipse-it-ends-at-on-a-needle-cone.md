---
id: tier-3-prime-cannot-settle-a-ruling-against-an-ellipse-it-ends-at-on-a-needle-cone
kind: issue
title: tier 3′ cannot settle where a seam ruling meets the section ellipse it ends at, on a needle cone cut nearly along its axis
status: open
opened: 2026-10-29
priority: P2
cost: M
---



## What

The needle cone of `crates/sweep/tests/a_plane_near_a_needle_cones_apex.rs`
(half-angle 1° or 3°, revolved whole about `y`, two half-turn faces
between the seam meridians at azimuths 0 and π), split by a plane
through the point `10⁵ε` down the seam ruling from the apex with its
normal tilted `tilt` off the axis toward azimuth `spin`. The split lands
both sides at their closed forms and both pass tier 3, but tier 3′
(`topo::validate_pseudomanifold`) refuses every pair of a seam ruling
and a half of the section ellipse:

    CensusUnsupported { subject: Entity(Edge(..)), cause: ContactLane(NotCertifiable {
        what: "the meeting reading of two curved edges could not settle a count" }) }

Measured at ε 1e-9: `tilt ∈ {1.4, 1.5}` with `spin ∈ {0, π}` on the 1°
cone, and `tilt = 1.5, spin = 0` on the 3° cone; every other pose of the
grid `tilt ∈ {0.3, 0.9, 1.2, 1.4, 1.5} × spin ∈ {0, 1, 2, π, 5}` passes.
Those are the poses where the ellipse's vertices sit on the seam
meridians, so the split leaves two rulings (apex to each vertex) and two
ellipse halves, each half ending on both rulings.

The wider sweep of the same family (α ∈ {1°, 2°, 3°, 5°}, foot
`kε` with `k ∈ {0.3, 2, 10, 100, 10³, 10⁴, 10⁵}`, tilt ∈ {0.05, 0.4,
0.9, 1.2, 1.4, 1.5, 1.53, 1.55}, ten spins) lands 1058 of 2240 splits
at ε 1e-9, every landed tip at its closed form to 1e-6; 105 of them fail
tier 3′ the same way (107 at ε 1e-6, 105 at 1e-12), at `k = 10⁴` and
`10⁵` on the 1°–3° cones and `k = 10³`, `10⁴` on the 5° cone, mostly at
tilt ≥ 1.2, and not only at the seam azimuths. Tier 3 passes on every
one. Only the seam poses had their fault and edge pairs read (above);
the rest carry the same `NotCertifiable` text, unread past it.

The fault, read at `census/curved.rs`'s `interior_meeting`:
`MeetFault::Unsettled(Unsettled::Meter)` for each (ellipse half,
ruling) pair — the residual's rounding, or a located root's slack, is
not definitely inside the band. The meetings are the shared end
vertices, where the ellipse leaves the ruling square to it, so nothing
about the geometry is in band; the ellipse is long and thin (it lies a
few degrees off the rulings), which is where the meter loses the root.

## Where to look

`geom_brep::curve_pair::meet` on an ellipse against a ruling described
as a chart curve, the meter in `geom_brep::roots`. The
`splitdoor/roots-unsettled-split` unit re-cuts `Unsettled`; this row
is a `Meter` case on settled geometry, so read it after that lands.
The pin is the `spin = 0, tilt = 1.5` rows of
`a_needle_cone_cut_off_its_apex_lands_its_closed_forms`, which assert
the refusal as measured.

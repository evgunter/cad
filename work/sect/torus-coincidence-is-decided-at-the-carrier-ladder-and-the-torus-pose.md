---
id: torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose
kind: issue
title: Two tori decided one surface are spelled twice: the carrier ladder's torus rows and the torus pose's tt_coincident
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-10-29
branch: sect/torus-decided-once
---


Found by `sect-toruspose` (branch `sect/torus-pose`) sweeping for every
site that decides a torus pair's tangency or coincidence.

## Measured

`crates/topo/src/boolean/carrier_eq.rs` `torus_data` decides two torus
carriers one surface on four rows (`carrier_torus_axis_parallel`
levered, `carrier_torus_center`, `carrier_torus_major_radius`,
`carrier_torus_minor_radius`), each on its own, then the sum where all
are in band (`declared_reading`). `crates/geom-brep/src/pair_section/torus.rs`
`TorusTorusPose::decide` decides the same question for a coaxial pose on
one row, `tt_coincident` (`|R − R₂| + |r − r₂| + |z₂|` read across the
coaxial slack), and refuses `CoincidentSurfaces` before the gap. Two
spellings, over different extents (the ladder's consumed extent, the
smaller torus's ball), so a pair in band of one can read definite at the
other.

## What a fix has to supply

One decision of "two tori are one surface", read by both the glue door
and the section value, or an argument that the two questions differ.

## Closed — one decision, on the pair, read by the pose and the ladder (#sect/torus-decided-once)

The two spellings asked one question: the ladder's torus reading was
already extent-free in its sum (`t·max(R₁, R₂)`, "whatever the ball"),
and only its undeclared tilt datum read the consumed extent. Measured
on main, about a ring of `R = 5`, `r = 0.06` at the witness band: a
tilt swinging the far ring `0.74ε` was one carrier at the ladder and
TANGENT at the pose (its tilt, levered at twice the ball, read in band,
so it routed to the gap, which read the cores' least distance and the
tube level both within the zero band); a slide of `0.6ε` along and
across the axis was one carrier at the ladder and escalated at the pose
(`tt_coincident` summed `|z₂|` and the offset slack to `1.2ε`).

`geom_brep::tori_coincide` (`pair_section/torus_coincidence.rs`)
decides it once: one where the distance bound (core move plus tube
difference, `TorusDisplacement::upper`, row `tt_coincident`) decides
Zero; apart where a datum alone decides past the band; otherwise
undecided. `TorusTorusPose::decide` asks it first, in every stance; the
carrier ladder's undeclared torus arm returns it, and its declared
reading reads the same `TorusDisplacement`. Rows:
`tori_coincide::the_pose_reads_two_tori_one_where_the_decision_does`,
the oracle search `tori_coincide::a_pair_decided_one_or_apart_stands_where_the_decision_says`,
and `carrier_eq::tests::the_ladder_reads_two_tori_as_their_pose_does`.

**What moved at the ladder, the corner sites included** (`carrier_eq`
undeclared, read by `vtxfac`, `carrier_pair` and `recl`): main decided
two tori one where each datum read within ε, the tilt levered at the
caller's extent; the decision stands on the sum, the tilt levered at
the larger major radius. Measured about `R = 2`, `r = 1` with 1 mm balls
on the equators: `ΔR = Δz = 0.9ε` with a tilt of `1.5ε` at the ring
(inner ball) reads `SameOriented` on main and `Undecided` now; a tilt of
`8ε` (outer ball) `Distinct` → `Undecided`; `15ε` (inner ball)
`Undecided` → `Distinct`. Every move is within the band of a datum or
the sum. Row:
`carrier_eq::tests::torus_data_in_zero_band_summing_past_it_is_undecided_at_every_reader`.
A torus off its ring convention, for which the bound does not hold, is
unreadable undeclared and unsettled declared
(`a_spindle_torus_is_unreadable_at_the_ladder`); a body's validity
refuses it (`ValidationError::DegenerateTorus`), but the ladder also
reads unvalidated bodies.

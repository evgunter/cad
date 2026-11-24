---
id: a-seam-decided-pi-refuses-where-only-its-bend-is-undecided
kind: issue
title: The wedge fold refuses a seam whose wedge is decided π because one station's second-order sagitta lands in band: the bend's undecidedness is the description question's, not the seam's
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by HELDRECORDS' elbow lane (2026-11-22) while routing a pipe's
G1 junction with an elbow (`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`);
filed here because the walk is the must-carry rule's one home, the
description choice SELECT owns (`crates/geom-brep/src/dihedral.rs`).

## The defect, by reading

`geom_brep::dihedral::second_order_walk` reads each station's
second-order sagitta and stops at the first that is not `Positive`:
`Zero`/`Negative` answers `SecondOrderWalk::UnderDetermined`, in band
answers `SecondOrderWalk::InBand(diag)`. The whole-edge fold
(`geom_brep::edge_wedge`, `edge_wedge_over_as`) turns `InBand` into
`WedgeVerdict::InBand(WedgeEscalation::at(WedgeRung::Bend, …))`, so
the fold refuses the WEDGE although every station has already read the
pair smooth (first order) and the seam's wedge is π: only the bend,
which selects the edge's description (`TangentIntersection` where the
jet is determinate, the plain curve where it is not), is undecided.
Tier 3's check 4 and the glue door's rim routing
(`rim_wedge::classify_shared_rim`) both read this fold, so a straight
tube leaving a bend G1 — whose relative curvature passes through zero
at two points of the rim — refuses at a station that lands within ε of
one of those points, though the same pair decided exactly zero there
is a seam with `Bend::UnderDetermined`.

What is owed: the fold reports the seam with its bend undecided, and
the description question (the must-carry rule) refuses alone, typed,
where it needs the bend decided.

## Measured (2026-11-22, `heldrecords/elbow-seams`)

The elbow of `crates/editor-core/tests/a_pipe_unions_with_its_elbow.rs`
with its axis turned 20ε about the cap's normal puts a neutral point
within the band of a station of the seam's must-carry walk. The union
refuses `Escalated { decision: SeamJet, .. }` ("whether the two faces
touching along a seam edge curve apart there or share their curvature
is undecided: margin 1.2e-9"), though the seam's wedge is π. It is
pinned as
`an_elbow_whose_neutral_point_sits_in_band_of_a_station_refuses_on_its_bend`.
At 2ε (nearer) and 10³ε (farther) the same elbow unions.


The paragraph above was written on `heldrecords/elbow-seams`; on
`heldrecords/elbow-ends`, where the junction routing was not yet
reached by that pose, the lane recorded instead that no fixture
reached it. Both stood until the close below.

## 2026-11-23 — Narrowed by PR 7471 (HELDRECORDS), not closed

`geom_brep::dihedral::second_order_walk` now reads on past a station
whose bend is in band, and answers `InBand` only once every station is
read and none decided `Zero`/`Negative`. The rim routing reads the fold
through `geom_brep::rim_routing_wedge`, where an aligned pair whose
bend is in band is the π seam, its bend under-determined, so the
routing reads the seam and a junction glues. Every reader that stores
or judges a description (the must-carry rule, `edge_wedge_over`, tier
3's check 4, the extrude and revolve joins, the blend, the split's
finish) still refuses `InBand` on the `Bend` rung: the description
question refuses alone, typed, as this row owed. Facing each other, the
bend decides between an end and the lamina, so the routing refuses that
too. A station whose first-order wedge is in band refuses as before.

Pinned: `dihedral::bend_rows::a_bend_in_band_at_a_station_leaves_the_seam_under_determined`
(geom-brep), and
`a_pipe_unions_with_its_elbow::an_elbow_whose_neutral_point_sits_in_band_of_a_station_glues_a_junction`
(editor-core): the poses `neutral_off_the_seam(δ)` at δ = 1e-8, 3e-8,
1e-7, whose neutral point sits at a routed rim vertex, glue a junction
and reach the crossing layer; the 20ε pose glues a junction and refuses
`Escalated { SeamJet }` on the seam's description.

## 2026-11-24 — Still open (HELDRECORDS)

The routing half is done: a junction glues where a station's bend is in
band. The union still refuses where it reaches the seam's description:
the elbow turned 10ε to 100ε about the cap's normal refuses typed
`Escalated { SeamJet }` (PR 7471's second review, R2), pinned at 20ε in
`an_elbow_whose_neutral_point_sits_in_band_of_a_station_glues_a_junction`.
What is still owed: those poses build end to end, the seam described
where it rests, or a ruling that the description refusal is the answer.

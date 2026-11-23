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

## Not measured

No fixture reaches it yet: the elbow lane nudged the bend's axis by
2ε and 10³ε about the cap's normal
(`an_elbow_whose_neutral_points_sit_between_the_stations_glues_a_junction`)
and the rim routed π at each, so no routing station landed near a
neutral point; where the routing's stations fall on that rim was not
traced. The finding stands on the code above.

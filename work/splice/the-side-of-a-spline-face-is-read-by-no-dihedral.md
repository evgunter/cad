---
id: the-side-of-a-spline-face-is-read-by-no-dihedral
kind: issue
title: The dihedral side reading has no arm for a spline face: classify_dihedral and implicit_residual are poison on a NURBS surface
status: closed
opened: 2026-10-26
closed: 2026-10-26
priority: P1
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, a-unions-flush-reading-cannot-follow-a-nurbs-carrier]
branch: splice/spline-dihedral
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(fork log row 198), on a designer's finding.

Once a shared rim is glued at the coincidence door, what remains is
which side of the shared edge each wall lies on. `classify_dihedral`
and `implicit_residual` are poison on a spline surface, per the curved
arm's own comment. The reading needs a spline arm built from
`patch_bound`'s signed normal enclosures. Lily wall 8 needs it beside
the boundary-first reading and the two flush rows.

## Closed

`geom_brep::dihedral_side` (`crates/geom-brep/src/dihedral_side.rs`)
reads the side of one face that a second face leaves a shared edge on:
the departure `n × t̂` against the other face's outward normal, through
`enters_material` over the folded arm. An analytic face's normal is
its implicit gradient. A spline face's normal is `S_u × S_v` at a foot
decided on the face within the band, read through the scalar's
`NurbsLane` (`NurbsLane::surface_foot`), with its largest principal
curvature folding the arm. A spline face is given no signed residual.
A tangent dihedral escalates `SideEscalation::Tangent`, and an in-band
one escalates through the lever's rungs.

Witness rows, in `dihedral_side::tests`:
`a_spline_wall_reads_its_side_of_a_plane`,
`a_plane_reads_its_side_of_a_spline_wall`,
`a_spline_wall_and_a_cylinder_read_each_others_side`,
`a_tangent_or_in_band_departure_escalates`,
`the_analytic_dihedral_reads_no_spline_face`,
`a_point_off_the_spline_face_is_refused`,
`a_spline_face_without_a_lane_refuses_typed`.

The boolean does not call the door yet. Its consumer is the sweep's
curved arm, behind the boundary-first reading:
`the-curved-arm-reads-a-spline-faces-side-through-the-dihedral-side-door`.

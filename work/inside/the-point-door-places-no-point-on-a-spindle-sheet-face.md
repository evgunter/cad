---
id: the-point-door-places-no-point-on-a-spindle-sheet-face
kind: issue
title: The curved point door answers Trim(None) for every point on a spindle sheet face, so no station lands on one and a crossing through a sheet is never confirmed
status: open
opened: 2026-11-25
priority: P1
cost: M
refs: [a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine]
---


## Finding

Found by lane `embedwalk-spindle` (2026-11-25), giving a spindle sheet
its window in the injectivity engine.

`topo::contain`'s `curved_interior_placement` reads a cylinder, cone,
sphere and torus face through its carrier's closed form, and answers
`Ok(CurvedPlacement::Trim(None))` for `geom::Surface::Spindle` (the arm
it shares with the plane, which `face_placement` reads before it). So
`topo::cell_region::station_region` places no station on a sheet face,
and the verdict door's verifier (`topo::clearance::embedded`, its
`on_face_stations`) has no pair of stations on a candidate holding one:
a crossing through a sheet is never confirmed. The walk keeps splitting
the pair and refuses on its budget, never `Embedded`.

Reproducer: `sweep::all self_intersection_proves_injectivity::footballs_through_one_another_never_read_embedded`
(two footballs grafted through one another, their sheets crossing on a
circle): `Refused(Budget { stop: Pairs { max_cell_pairs: 1024 }, .. })`
where two stepped rings through one another read
`Embedding::Crosses` (`two_stepped_rings_through_one_another_cross`).
Its assertion is `!Embedded` for that reason; once a sheet station is
placed, the row can ask for `Crosses`.

What is needed: the sheet's arm in `curved_interior_placement`. The
sheet's chart is the torus's formula with `R` signed
(`geom::Surface::Spindle`), and its inverse is in
`geom::chart_point` (`ChartArm::Spindle`), so `torus_face_containment`
with a signed major radius may serve; its region legs decide the ring
convention (`geom::require_ring_torus`, `torus_face_region`) and refuse
a sheet today, so that is where to look first.

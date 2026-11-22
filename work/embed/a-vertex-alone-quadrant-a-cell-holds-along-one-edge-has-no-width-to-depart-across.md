---
id: a-vertex-alone-quadrant-a-cell-holds-along-one-edge-has-no-width-to-depart-across
kind: issue
title: A plane face presented as its quadrant at a vertex alone, in a cell that holds it only along one of its edges, has a rectangle of no width across that edge at every depth, so its pair splits to the floor though a plane parts the wedges
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Two plane faces sharing a vertex alone, each presented there as its
quadrant (`Zero::Point` on both), refuse `Uncertified { Departure }` at
the vertex although `planes_through`'s wedge plane parts them
(`wedges_apart`, `topo::injective`). In every refusing pair one of the
two presented rectangles has an extent of at most `5e-15` along one
axis, at every depth from the first split to the floor, for example
`(0, 6.9e-16) × (0, 0.125)` and `(0, 2.0e-16) × (0, 4.9e-4)` at depth
40. `sign_on`'s `Zero::Point` arm decides each axis's departure as
`∇ψ·Φ_axis·span` at the band, so an axis with no span is decided by no
plane, however wide its margin.

## Evidence

Measured on `embed/vertex-bisector-measure` (main `9bc9d6517` with the
at-rest gate of `embed/gate-measure`, and the wedge plane), with the
gate asking, over the census's vertex plane/plane tests. Four tests
still refuse first at a pair presented `Zero::Point` on both faces:

- `split_through_an_apex::a_split_through_an_apex_closes_each_corner_of_its_section_on_its_own`
  (`topo::all`), faces `3v1`/`14v1`, `7v1`/`14v1`, `10v1`/`14v1`;
- `join_pierce_runs_sweep::a_shared_vertex_fan_reads_its_cuts_from_its_own_first_entry`
  (`sweep::all`), eleven pairs, among them `8v3` against six faces;
- `join_rc_probes::reflex_corner_edge_in_face_poses_build_every_op_undeclared`
  (`sweep::all`), `3v1`/`14v1` and `8v1`/`14v1`;
- `band_mixed_vertex::the_odd_edge_with_one_other_chamfers_at_the_flush_fixture`
  (`sweep::all`), `13v1`/`18v1`.

In each, the wedge plane leans at least `0.10` (cosine) from every one
of the four edge directions (`0.10` to `0.99`), so the plane is not the
cause; the heading plane does not part them either. Without the wedge
plane `split_through_an_apex` and `a_shared_vertex_fan` refuse on the
same first pair (`3v1`/`14v1`, `5v1`/`8v3`), while `band_mixed_vertex`
and `join_rc_probes` refuse earlier on another (`9v3`/`20v1`,
`3v1`/`12v1`), which the wedge plane certifies.

Why the rectangle has no width is not traced. The likely cause is a
cell side through the vertex along one of the face's edges, the face on
the far side of it but for a rounding, so the cell holds the face's
edge and nothing across it; at every split the vertex stays on a cell
side. What would separate that from the next likely cause (the
presentation's affine clip reading the edge's far end) is a print of
the cell and the vertex's chart point at the refusing pair.

## What a fix needs

A reading of a quadrant whose rectangle has no width across one axis:
the face's points in the cell lie on its edge there, so its sign is the
other axis's alone. That is a change to what `sign_on` requires of a
`Zero::Point` at the band, so it is the band's question as much as the
presentation's: settle it with the orchestrator before code.

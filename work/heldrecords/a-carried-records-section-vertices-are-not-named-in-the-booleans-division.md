---
id: a-carried-records-section-vertices-are-not-named-in-the-booleans-division
kind: issue
title: The boolean's division of a face names its section vertices only for a reduction row, not for a carried record
status: closed
opened: 2026-11-25
priority: P2
cost: M
closed: 2026-11-26
---


Found by the `heldrecords/removed-members` lane.

## What

A face the boolean divides names its section cells (the edges and
vertices beside the join's null faces, where the cut ran through it)
among its division's minted cells (`boolean::ops` `Incidence::chords`).
It names the section vertices only for a reduction row on a re-cut
face (`face_pieces` with `Keys::Recut`, the `place_found` path). For an
operand's carried record (`operand_fate`, `Keys::Input`) it names the
section edges alone.

The reason is in `carried`. With the section vertices named, two
records' cells can land on two section vertices that meet at a point
(`a_vertex_through_a_declared_seat::a_wall_vertexs_edges_are_in_the_seat_on_both_sides`).
That landing is a minted meeting, which `carried` decides
(`decide_one` on `uncopied` vertices), and the decision row names a
clone-minted vertex that no input holds. `ops.rs` then panics when
that row leaves the op ("which that input does not hold").

## Consequence

A carried vertex record whose touch the cut moved onto a section
vertex lands on no named cell and refuses `CarryRefused { Carried,
Stale }`, where it should land or be consumed as structure. It fails
loudly, and no test reaches it today.

## Way through

Decide a meeting of two section vertices in terms the decision can
name: the record's own two input cells, or the section's edge in the
reduction's keys. Then name the section vertices on both paths.

## 2026-11-25 — the carried path names the section's edges (`heldrecords/removed-members-red`)

Naming no section on the carried path refused
`three_solids_on_one_line::a_contact_line_beside_a_face_holding_it_builds_in_every_op`
(`Carried(A) Stale`): its edge-in-face record becomes structure only
through the section edge. So the carried path names the section's
edges again, and the vertices alone stay unnamed. The edge's minted
vertex-on-section-edge meeting that this reaches in
`a_pierce_held_in_a_curved_face` refuses `JoinDesync` until the door
decides it (`the-boolean-door-decides-no-minted-vertex-on-edge-meeting`).

## 2026-11-26 — closed: the carried path names the section vertices (`heldrecords/minted-door`)

The carry's door now writes a minted meeting's row on the carried
record's own two input cells, as their meeting, never on a cell the
op's clones minted, two vertices or a vertex inside an edge alike
(`boolean::ops` `minted_meeting`). So no row names a section vertex,
and both paths name the section's vertices (`Incidence::chords` takes
no flag). `a_vertex_through_a_declared_seat::a_wall_vertexs_edges_are_in_the_seat_on_both_sides`
passes, as does
`three_solids_on_one_line::a_contact_line_beside_a_face_holding_it_builds_in_every_op`.

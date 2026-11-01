---
id: an-edge-along-a-partners-ray-through-a-touch-has-no-record
kind: issue
title: An edge along a touching partner's ray on the face leaves its overlap with that ray unrecorded, and tier 3′ refuses it
status: closed
opened: 2026-10-12
closed: 2026-10-16
priority: P3
cost: M
---


Filed by the partner-along-the-face lane
(`a-touching-vertex-beside-a-partner-along-the-face-refuses`).

## What

The plate united with a pyramid lying on its top (`meeting::lying`),
whose edge from `MEET` at 60° lies on the top: the ray. A pyramid with
an edge along that ray, pairing with the lying pyramid's apex at `MEET`
while its own apex touches the top, builds sound in every op
(`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`,
`a_touching_vertex_beside_a_partner_along_the_face_builds_sound_in_every_op`;
tier 3, volumes, material probes, the analytic germ of every row at
`MEET`). Its result's record does not carry the overlap of its edge
with the ray, so tier 3′ refuses
`UnrecordedContact { contact: EdgeEdgeOverlap { .. } }` there.

Measured at rest, on main 8cd31ca1d merged with the lane: `y − x` for
"under the lying pyramid's ray" and "continuing the lying pyramid's
face" (`meeting::along_lying_ray`), witness `(1.575, 1.12990…, 1.0)`,
mid-way along the shared stretch. The other poses are not tallied one
by one; the rows allow the refusal for an overlap of two edges both on
the ray (`three_prime`'s `ray`).

The pair at `MEET` is recorded. The edge-edge germ the vertex-vertex
pass reads along the ray (`recl::recl_edges`, its ray events) decides
the two edges coincide there, and nothing records it.

## The shape to give

Record the overlap the pair decides along a shared ray, as the vertex
pair itself is recorded, or say which door owns the record of an
edge-edge germ through a touching vertex. Then drop the allowance from
`three_prime`.

## Closed

Built. The overlap went unrecorded because the edge-edge event misread
membership, not for want of a door. At a vertex pair whose vertex of
one solid touches a face of the other's, the event read the face's
solid by the wedge at its own vertex alone. Where that vertex lies on
the solid's own face, the face holds the line too, and the half-turn
under it is that solid's material as well. Read without it, `y − x`
dropped the touch along the ray, so `touch_rows` wrote no edge-edge
row.

The reduction now hands each operand's touched face through the pair's
point to `recl_edges` and `resolve_edge_edge` (`recl::HeldFace`). The
face counts where it holds the common line at a decided zero; a
holding in the band refuses typed. The two compose by the side of the
face the solid's own link lies on (`vtxfac::partner_side`): out of it
(a pyramid lying on the plate), a flank is in the solid's material
when it is in the wedge or under the face; in it (a void below the top
whose ray lies on it, the wedge the void's complement), when it is in
both. The mirrored void scene, red on main in `x ∪ y`, now builds and
holds tier 3′ in every op, both slots, every pose.

A touch's two ends must read the material by one rule (`touch_rows`).
Where a solid has an edge and a plane face on the line, one end may
read the face as a held face and the other as a sector of the solid's
own vertex. The contact-line arm read such a face's half-turn blind to
the edge's wedge. It now folds the face into that edge's wedge by the
same side-composed rule, keeping the blind reading between two edges
and for a curved face. That turns `three_solids_on_one_line`'s
subtraction of the 0° prism into a recorded touch: main reads it
Dropped at both ends and tier 3′ refuses its edge-edge overlap. It is
pinned by `edges_on_the_line_recorded`. A curved face holding the
line refuses as the partner beside it does (filed:
`a-line-held-by-a-curved-touched-face-refuses-at-the-edge-edge-event`).
`three_prime`'s edge-edge `ray` admission is gone, and all 12 tests of
`a_vertex_read_by_two_sector_passes` hold tier 3′ at every pose and ε.
The white-box row is
`recl::tests::a_solid_with_an_edge_and_a_face_on_the_line_holds_the_half_turn_under_the_face`.


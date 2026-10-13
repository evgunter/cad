---
id: an-edge-along-a-partners-ray-through-a-touch-has-no-record
kind: issue
title: An edge along a touching partner's ray on the face leaves its overlap with that ray unrecorded, and tier 3′ refuses it
status: open
opened: 2026-10-12
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

---
id: a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses
kind: issue
title: A vertex read by two sector passes refuses VertexReadTwice even where its first pass hangs no strut there; stacking pyramids on a plate at one point refuses
status: closed
opened: 2026-10-07
priority: P1
cost: H
refs: [a-vertex-read-by-two-sector-passes-panics-instead-of-refusing]
closed: 2026-10-07
---


## What

`vtxfac::refuse_sector_rereads` refuses `BooleanError::VertexReadTwice`
wherever a vertex pierces a face of the other solid and is read again,
by a second pierce or by a vertex-vertex pair. That other solid holds
its own contact at the point. The refusal is sound but wide.

The everyday pose it catches: pyramids standing on their apexes at one
point of a plate's top, folded onto the plate one at a time. The plate
and the first pyramid unite into a body that keeps the apex on the top
with no vertex of the top there. The second pyramid's apex then pierces
the top and pairs with the first apex, and the fold refuses
(`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`, "one
standing pyramid"). Uniting the pyramids first and then the plate
builds.

Measured on the merge base with debug assertions off, before the
refusal: wherever the piercing vertex only touches (its link on one
side of the pierced face), no strut is hung at it. Every op built a
body at tier 3, with volumes that add up: the arches, one standing
pyramid, and two blocks in face contact, at every pose. Tier 3′
failed only where the operand's own contact is dropped
(`work/wire/a-boolean-drops-its-operands-own-contact-records.md`). Where
the pierce hangs struts, as for a `meeting::wedge` prism through the
top, the vertex-vertex pass hit `sectors.rs`'s `unreachable!` ("has the
null edge … in its orbit").

## The shape to give

Two routes, to be weighed first:

- **Read every orbit before the first pass writes**, as the
  vertex-vertex lane does. Then decide what the pierce's struts and the
  pair's insertion do together at one vertex.
- **Narrow the refusal** to the reads that follow a written orbit (a
  pierce with runs on both sides). Then prove the touch-only re-read
  sound, rather than measure it.

Done when the plate-first fold of standing pyramids builds in every
member order at tiers 3 and 3′, and the prism row still refuses or
builds sound.

## Closed

The refusal narrows. `vtxfac::refuse_sector_rereads` still refuses a
vertex that pierces two faces. For a vertex that pierces a face and
pairs, it returns the vertex, and the pierce refuses
`VertexReadTwice` only where it would hang struts (it has Out runs),
before it writes. A touching pierce writes nothing, so the pair reads
the orbit as the operand gave it. `vtxfac::partner_side` reads the
partner's link against the pierced face's datum, and refuses unless it
lies strictly on one side. `vtxfac::touch_classes` then reads the
vertex's edge classes against the face and its partners together:
joined with the partners on the Out side, less the voids of the
partners on the In side. So each edge at the vertex gets one class.
The soundness argument is in the PR body.

The plate-first fold of standing pyramids builds in every member order
at every pose, at tier 3 and the closed-form volume. Tier 3′ refuses
only the earlier steps' contacts at `MEET`
(`work/wire/a-boolean-drops-its-operands-own-contact-records.md`).
The strut-hanging prisms (`meeting::wedge`, `meeting::leaned`) still
refuse, and so does a vertex piercing two blocks' faces
(`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`). Filed:
`work/contact/a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone.md`
(P2).

---
id: a-touching-vertex-beside-a-partner-along-the-face-refuses
kind: issue
title: A vertex touching a face and paired with a partner whose link runs along that face refuses VertexReadTwice, though the cells seen build right
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses]
---


Filed by the touch-only re-read lane (PR 4256, from its review).

## What

`vtxfac::partner_side` refuses a touching vertex's pair unless every
bound of the partner reads strictly on one side of the pierced face.
A partner with an edge lying on the face fails that. Example: a pyramid
standing at `MEET` with one base corner on the plate's top, united with
the plate. Every op of a pyramid touching there then refuses
`VertexReadTwice`
(`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`, "a lying
pyramid").

PR 4256's first review removed the refusal as a mutant. All 90 cells
then built right, by material and by naming. So the refusal is caution,
not a known wrong answer.

## The shape to give

The touch reading's premise is that the face and the partner's cone
meet only at the point. Where the partner's link runs along the face,
they meet along an edge, and that edge is an edge-on-face contact of
the other solid's own. Admitting it means proving the layering when
the face and a partner share a ray: which side of the face the
partner's cone lies on, and how an edge of the touching vertex along
that ray is classed.

## Another witness (2026-10-08, PR 4346)

`crates/topo/tests/three_solids_on_one_line.rs`,
`a_prism_whose_face_holds_the_line_builds_or_refuses_typed`. The fixture
is the plate, upright prisms over 0°–50° and 120°–170° about (1.5, 1),
and a third prism over the triangle from 0.3 at 80° and 260° to 0.4 at
350°, whose side face holds the line through (1.5, 1). Three member
orders refuse `VertexReadTwice { operand: B, reads: [Pierce(_), Pair(_)] }`:

- [3, 2, 1, 0] and [2, 3, 1, 0] at step 2;
- [2, 0, 3, 1] at step 3.

Member 0 is the plate and member 3 the third prism. Those orders refuse
identically on main (8fd03fce), so this is not that PR's change. The
row asserts the refusal by order.


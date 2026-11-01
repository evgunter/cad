---
id: a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt
kind: issue
title: A contact line whose ray holds the other solid's subdivision bisector refuses ClassificationInvariant (unbuilt) in four member orders
status: closed
opened: 2026-10-09
closed: 2026-10-13
priority: P2
cost: M
---


Filed by the partner-along-the-face lane
(`a-touching-vertex-beside-a-partner-along-the-face-refuses`), whose
re-baseline adds the fourth order.

## What

`crates/topo/src/boolean/recl.rs`, `recl_edges`: where a ray event
groups coincident edges of both solids and one of the group is a
subdivision bisector (`!m.real`), the pair refuses
`ClassificationInvariant { what: "a contact line's ray holds a
subdivision bisector (unbuilt)" }`. No arm reads it.

Row: `crates/topo/tests/three_solids_on_one_line.rs`,
`a_prism_whose_face_holds_the_line_builds_or_refuses_typed`
(`ON_A_BISECTOR`). The plate (member 0), upright prisms over 0°–50°
(1) and 120°–170° (2) about (1.5, 1), and a third (3) over the
triangle from 0.3 at 80° and 260° to 0.4 at 350°, whose side face holds
the vertical line through (1.5, 1). Left folds of `topo::union`
refuse it at step 3, folding the 0° prism last onto a body holding the
plate, the third and the 120° prism:

- [3, 0, 2, 1], [0, 3, 2, 1] and [0, 2, 3, 1], as on main (db51132ff);
- [2, 0, 3, 1], which refused `VertexReadTwice` at the same step on
  main, its line's foot touching the plate's top beside a partner whose
  link runs along it; with that pair read, the step goes on to this
  refusal.

The row asserts the refusal by order. A refusal typed as a kernel
invariant on valid input is a missing arm, not a defect report.

## The shape to give

Read the bisector's sector as the solid's material across the ray (a
bisector bounds no face, so the wedge it splits is one sector of the
face it lies in), and pair it as `recl_edges` pairs a real edge's
flanks; or say why the arm cannot be built and refuse with the
coincidence it is.

## Closed

Built. In `recl_edges`'s contact-line arm, each solid's material about
the line is wedges, and each of one solid's is paired with each of the
other's by the edge-edge rule (`recl::resolve_edge_edge`, which takes
two `recl::Wedge`s). A real edge's wedge lies between
its flanking faces. A bisector's is the half-turn of the face it lies
in, flanked by that face's two halves. The row builds sound in all 24
member orders. A new row,
`a_prism_against_a_contact_line_on_a_bisector_builds_or_refuses_typed_in_every_op`,
has the 0° prism building sound in every op and slot.

The sibling site, "a solid's coincident edges on one ray lie in a face
of the other (unbuilt)", was reachable: a wedge whose face holds the
line, its apex at the plate's top. It is built the same way: the face
sector the other's mentions were read On against is that solid's
half-turn. Row: `a_contact_line_in_a_face_of_the_other_builds_in_every_op`.

A prism crossing the third's face along the line still refuses,
`VertexReadTwice`, or at the join in one slot:
`a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice`.

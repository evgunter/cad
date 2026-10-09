---
id: a-contact-line-whose-ray-holds-a-subdivision-bisector-refuses-unbuilt
kind: issue
title: A contact line whose ray holds the other solid's subdivision bisector refuses ClassificationInvariant (unbuilt) in four member orders
status: open
opened: 2026-10-09
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

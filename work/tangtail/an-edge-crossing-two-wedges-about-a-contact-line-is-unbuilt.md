---
id: an-edge-crossing-two-wedges-about-a-contact-line-is-unbuilt
kind: issue
title: An edge crossing two wedges about a contact line is unbuilt: two sections along one edge at one vertex pair do not zip
status: open
opened: 2026-10-13
priority: P3
cost: H
refs: [an-edge-crossing-two-wedges-about-a-contact-line-refuses]
---


## What

`crates/topo/src/boolean/recl.rs` `recl_edges` refuses
`BooleanError::CrossingsAlongAContactLine` where an edge of one operand
runs along a contact line of the other (two coincident edges on one
ray of a vertex) and its wedge crosses both of theirs: two edge-edge
germs on one ray of one vertex pair. The recourse it names (apply the
op to the pieces before they are joined, or move the edge off the
line) builds; the shape itself does not.

Row: `crates/topo/tests/three_solids_on_one_line.rs`,
`a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`
("both wedges crossed": the 6 orders that fold the bridging prism
last) and
`an_edge_crossing_both_wedges_refuses_typed_in_every_op_and_its_recourse_builds`
(every op, both operand slots, 36 refusals).

## What was tried

Each germ placed on its own record (distinct sector pairs, so
`place_germ` takes both), measured over the 6 refusing union orders
(each op fires the event twice: at the plate's top and at the 120°
prism's top corner, both ends of the doubled section):

- the shared edge (the bridging prism's) folded Out in both germs, as
  `Reversed` folds a pinch's: every order panics in
  `zip.rs` `glue` (`proven` on a half-edge an earlier joint killed);
- no fold: every order refuses `Join(Euler(NotSameFace))`;
- the contact-line side folded Out, or both sides: the same
  `NotSameFace`.

The two sections run along one edge between the same two vertex
pairs (both pieces' faces are dropped there; the result's edge pairs
the outer face of one piece with the outer face of the other), so the
bridging solid's two null edges along its edge bound a zero-width
strip that the zip has to collapse, joining one piece's kept face to
the other's across it. `recl` alone cannot place that; the insertion
(`insert::plan_null_pairs`, `reconcile_shared`) and `zip::glue` need a
rule for two seams sharing one edge.

## The coincidence door

As the parent row: the edge lying on the contact line's ray is
`bool_ee_collinear`'s margined verdict, read from values; no
declaration is consulted.

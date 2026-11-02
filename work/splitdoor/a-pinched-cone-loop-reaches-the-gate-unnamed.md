---
id: a-pinched-cone-loop-reaches-the-gate-unnamed
kind: issue
title: A cone face pinched at its apex reaches the at-rest gate as VolumeUncomputable (props_cone_loop_vertex) rather than ResultPinchedAtConeApex
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by SPLITDOOR unit B stage B2 (`splitdoor/cone-apex-edge-face`),
pinning a box edge through a cone's apex
(`crates/sweep/tests/a_cone_apex_on_an_edge.rs`,
`an_edge_along_the_axis_holds_a_quarter_of_the_cones_top`).

## What

The 60° wrap-tipped cone (`common::wrap_tip::one_wrap_edge`) against the
box `[0, 2] × [0.55, 1.6] × [0, 2.1]`, its edge along `y` the cone's
axis, turned `0` about it: `cone ∪ box` refuses

`ResultInvalid { errors: [VolumeUncomputable { source: Face { face, source:
ConeLoop { what: "props_cone_loop_vertex" } } }] }`

At 30° and 45°, and at other turns, the same pose's remainder ops either
build or refuse `ResultPinchedAtConeApex` by name
(`boolean::ops::pinched_at_cone_apex`).

`props_cone_loop_vertex` (`geom_brep::props::curved`,
`require_cone_loop_vertices_distinct`) refuses a cone loop with two
non-adjacent vertices that coincide in space. The named reading refuses
a loop that starts two of its EDGE half-edges at one vertex key decided
the apex. So this loop meets itself at the apex in a way the named
reading does not see. Two readings are open:

- the loop passes the apex vertex through a member the named reading
  skips (not an `Edge` start), and the refusal only needs naming;
- two distinct vertices sit at the apex, the box edge's split vertex
  not identified with the cone's apex vertex. That would be a
  topological defect of the result, which the gate refuses but which
  the vertex pair should never leave.

## Where to look

Build the result short of the gate (the at-rest gate refuses it) and
list the cone face's loop vertices at the apex: their keys tell the two
readings apart. The pin accepts this refusal for `∪` and `cone ∖ box`
only, beside the named pinch.

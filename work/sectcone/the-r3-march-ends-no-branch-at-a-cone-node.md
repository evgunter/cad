---
id: the-r3-march-ends-no-branch-at-a-cone-node
kind: issue
title: The ℝ³ march ends no branch at a node, so a cone pair's arcs at its apex node or a crossing tangency are classified and not carried
status: open
opened: 2026-11-22
priority: P1
cost: H
---



Filed by the cone × sphere / cone × cone door lane (PR 7457).

A cone pair's section lists a node at the apex where it lies on the
partner (`pair_section/cones.rs` `ConeRuledPose::apex_stance`, the
`Node` stance) and at a crossing tangency (`ruling::Crossing`), with
each arc's end there named (`NodeEnds`). The march has no node exit in
ℝ³: `ssi::march_seeds` ends a branch only at the slab or a ball
(`march::SlabExit`, `march::BallExit`), and `pair_section.rs`
`FittedCarrier::component` refuses a `BranchEnd::Node` from an ℝ³
march. So `cones::cone_ruled` gives every arc that ends at a node
refused at the march: `cones::cone_ruled` keeps every arc that ends at a
node the general rung's (`ClassCarrier::General`, its `NodeEnds` and
`NodeSet` listed), and `pair_section.rs` `march_over_the_ball` refuses a
cone pair's section that lists a node, `CarryRefusal::Unminted(NoCarrier::ConeNode)`,
before any march.
the value for the same reason (`pair_section/cone_torus.rs`
`cone_torus`).

C2 already states the chain's node end (`crates/geom-brep/README.md`,
"A chain may end at a node of the section (C5)"), built for the 1×2
lane only (`ssi/node.rs` `node_box`, `march::RectExit`'s node boxes).
At a cone's apex the cone's own gradient vanishes, so the ℝ³ system's
Jacobian drops rank there and the node box is the apex's closed form,
not a Krawczyk zero of `∇φ`.

The work: an ℝ³ exit into a node's box, the arc's carrier fitted on to
the node with the named ray as its end tangent, and limb 3's chain
ending in the node's box, for the apex node and for a crossing
tangency. Witnesses: `crates/geom-brep/tests/cone_pair_section.rs`
`a_nodes_arcs_refuse_the_carry_by_name` flips to a carry. The tube's half
of the same gap is `work/ssiedge/an-r3-tube-chain-has-no-node-end.md`.

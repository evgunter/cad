---
id: an-r3-tube-chain-has-no-node-end
kind: issue
title: An ℝ³ tube chain has no node end, so an analytic pair's arc ending at a node (a cone's apex on the partner) refuses its tube
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Filed by SECTCONE (`sectcone/cone-tube-arm`, the tube's cone arm). Its
spec asked how a rung-3 ℝ³ chain ends on a node today, and said to
refuse typed and file the gap if no ℝ³ node end exists. None exists.

## What

C2's node end is the chart chain's alone.
`crates/geom-brep/src/ssi/certify.rs` `node_ends` reads a chart
chain's ends (`SsiTube::Chart`'s `nodes`, through `ssi::node::node_box`
over the wall's chart), and only the analytic × NURBS arm of limb 3
calls it. The analytic × analytic arm, `probe_tube_analytic`, reads
`box_graph_margin` over every box of the chain, the end boxes
included. Its first refused box makes the whole rung
`Rung::unmeasured`.

At a cone's apex the gradient vanishes, and the tube's cone arm
(`ssi::enclose::implicit_gradient_enclosure`, gated off the apex)
refuses every box that holds it. So an analytic pair's arc that ends at
C5's node, a cone's apex on the partner, refuses its tube at every
rung: `SsiError::TubeProbeSilent { rungs, refused: rungs }`, and at the
edge door `AnalyticRung3Refusal::Tube(TubeRefusal::ProbeSilent { .. })`.
The refusal is typed and honest, but such an arc cannot certify.

`ssi::certify`'s `a_tube_whose_enclosures_are_refused_answers_no_rung`
pins it: a carrier along a 45° cone's ruling into the apex, against
the plane holding that ruling. A tangency node of two smooth analytic
surfaces ends no better: there `∇f₁ × ∇f₂` vanishes, so the end box's
margin straddles zero and the tube refuses as a straddle.

## What it needs

An ℝ³ twin of `node_ends`: a chain end that is a node ends in a box
over which the claim is the node's own reading (C5: its Hessian on
`S`'s tangent plane, the apex's closed-form margin), not the
transverse enclosure. It waits on a pair door that mints an arc ending
at a node. `cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door`
(SECTCONE) is the first. The cone × torus door
(`pair_section/cone_torus.rs`, PR 7395) classifies an apex on the torus
as a node or a touch and refuses it typed (`ConeTorusPose::unmarched`);
it becomes a second consumer once a chain can end at a node.

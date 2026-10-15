---
id: node-of-a-rod-crossing-an-equal-bore
kind: issue
title: The node of a rod crossing a bore of its radius refuses NodeUnbuilt: A's Out sectors and B's In sectors are different ones, which a pierce ring cannot pair
status: closed
opened: 2026-10-14
closed: 2026-10-15
branch: tang/node-rod-bore
---


Found by TANG U1's node unit (PR 5111's review, MAJOR 1).

## What happens

Two equal walls whose axes meet build every boolean at their nodes when both faces bound their material on the same side: two rods, or two bores (`crates/sweep/tests/bisector_nodes.rs`, `two_crossing_bores_build_every_operation_at_the_closed_forms`). A rod crossing a bore of its radius refuses `NodeUnbuilt { why: OppositeSenses }` in every op, both orders (`a_rod_against_a_bore_refuses_its_node_typed`, `crates/sweep/tests/bisector_nodes.rs:336`), raised at `crates/topo/src/boolean/vtxfac.rs:1729`.

## Why

Round a node, on A's wall the sectors about `±â₁` lie outside B's cylinder and those about `±â₂` inside it; on B's wall the same, the other way round (`crates/geom-brep/src/intersect.rs`, `TwoEllipses::nodes`). `mint_node` builds the node as A's ring vertex piercing B's face: A's struts span A's sectors outside B's material, and B's ring (`mint_ring`) spans B's sectors inside A's, the same runs. For two rods both are the sectors about `±â₁`; for two bores both about `±â₂`. For a rod against a bore, A's sectors outside B are those about `±â₂` while B's inside A are those about `±â₁`: two different germ pairings of the same four arcs, which a `RingPlan` (one set of runs for both sides) cannot express.

Measured before the gate: with A's runs about `±â₂` the join refuses `SingleSiteSectionLoop { count: 4 }`, with `±â₁` `JoinDesync` ("every chord arc separates a loose scaffolding pair").

## What building it needs

A node whose two rings pair the four germs differently on the two sides: A's struts across one pair of opposite sectors, B's across the other. That is a sibling of `RingPlan` with per-side runs, and the join must then accept a germ whose A half and B half sit in struts of different runs. The bodies are ordinary at rest: e.g. a rod through a brick bored to its radius, ∩ = the rod in the brick less the bicylinder.

## Closed

Built on `tang/node-rod-bore`; no new ring vocabulary was needed. The runs are the same four arcs on both sides: B's sectors inside A, the ones about `±â₁` when A is a rod and about `±â₂` when it is a bore. B's ring spans them as before. What differs by sense is only how A reads each run. A's normal at the node is B's when the walls are the same kind and B's reversed when they differ, so in A's own turn the run starts at B's end germ and lies in B's material, Below rather than Above (`mint_node`, `crates/topo/src/boolean/vtxfac.rs`). So the "What building it needs" above was wrong: how A reads the runs follows the senses, not which runs there are. A probe minting the struts that way built every op at its closed form, so the null-edge pairing contract (slot k is the same germ in both) holds unchanged.

`NodeWhy::OppositeSenses` is gone. `sweep/tests/bisector_nodes.rs::a_rod_against_a_bore_builds_every_operation` builds ∪, ∩ and both differences, both orders, at 90°, 60°, 35° and 150° and a re-posed 90°, against the closed form ∩ = 4π − 16/3 at 90° and inclusion–exclusion elsewhere, with the mesh volume as an independent oracle.

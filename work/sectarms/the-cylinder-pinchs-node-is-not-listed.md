---
id: the-cylinder-pinchs-node-is-not-listed
kind: issue
title: The cylinder pinch's node is not listed on the section value
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Filed by the cylinder × cylinder stage 3b-ii lane (branch
`sectarms/cylinder-node-listing`), per the orchestrator's ruling D1.

## What

C5 (`crates/geom-brep/README.md`, the node text Ev ratified in PR 6398):
"The value lists each node with the surfaces' common normal, its two
tangent lines and the side of each other the surfaces lie on in the
sectors between them … Each arc end at a node names its line and the
sense it leaves along."

The bisector pose lists its nodes so (`pair_section/cylinders.rs`,
`bisector_nodes`). The pinch (`CcNode::Pinch`: the thinner wall inside
the fatter, touching it at one node) lists **no node**: its two lobes are
`ClassCarrier::Unminted(PINCH)`, with no node ends
(`ComponentEnds::default()`, cut where the region straddles).

Listing it needs a new assignment: which lobe leaves along which two of
the node's four rays. The lobe on the `+a₂` side of the thinner wall
takes the two rays that lean `+a₂` (the π turn about the common
perpendicular swaps the lobes and maps each ray to its opposite); the
frame itself is closed form off `Q = II₁ − II₂`, as the bisector's is.
Nothing reads it until the pinch's arcs are built, so it waits for them.

## Where the absence is read

- `topo::boolean::section_cert` (`of_pair_section`): `unbounded` would
  read a straddled pinch (no listed node, every lobe cut) as a tangent
  ruling; it excludes a section holding a `PINCH` lobe by its note,
  which listing the node retires.
- `geom-brep/tests/cylinder_pair_section.rs`
  `the_pinch_lists_no_node_and_refuses_to_carry` pins the absence.

## Done when

The pinch lists its node, its frame and its lobes' four ends, and the
`PINCH` exclusion in `section_cert` is gone.

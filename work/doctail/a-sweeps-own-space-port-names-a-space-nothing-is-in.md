---
id: a-sweeps-own-space-port-names-a-space-nothing-is-in
kind: issue
title: an extrude, revolve, sweep or loft of a profile drawn on a frame mints a space port that nothing is in, and a user can place into it
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Found by a VGEOM designer weighing free-moving a free profile
(2026-11-25), by inspection only; not driven.

`Node::outputs` (`crates/editor-core/src/node.rs`) gives every extrude,
revolve, sweep and loft a `space` port unconditionally. But where the
profile it reads is drawn on a frame, the body is in that frame's space:
`SpaceWalk::node`'s `Extrude | Revolve | Sweep | Loft` arm answers
`self.var(profile).or_else(own)` (`crates/editor-core/src/doc.rs`). So
the node's own `space` port is then a minted space that holds nothing,
and nothing stops a user placing into it or mating against it.

Check by building a profile drawn on a face's frame, extruding it, and
placing a second body into the extrude's `space` port: what is that
space, and what does the placement mean? The likely repair is that the
port exists only where the body is born in a space of its own, or that
it aliases the frame's space; which is the question.

Signed (VGEOM orchestrator).

---
id: deleting-a-space-strands-the-placements-into-it
kind: issue
title: Deleting a new_space() strands the placements whose target reads it; the viewer's free-move then dies with NoSuchSpace
status: open
opened: 2026-10-22
---


Found by the D′1 lane's viewer port (`a-placement-names-its-target-space`).

A `Node::Space` (`new_space()`) defines one `Space` output and reads
nothing; a placement reads it in its `target` slot. Deleting the space
node takes its output with it, as any delete does (DM7): each placement
whose target read it is left reading a dead variable, reported as a
strand, and refuses at evaluation. Nothing cascades: the copies in that
space are not deleted or re-targeted.

The viewer keys a free-move frame by the space's variable
(`crates/viewer/src/display.rs`); once the space is gone the gesture
dies with `AdmissionFault::NoSuchSpace` (`display.rs:259`).

Open: whether deleting a space should take the placements into it (as
deleting a body does not), re-target them into the world, or stay a
strand the author repairs.

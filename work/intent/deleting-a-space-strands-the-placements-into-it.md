---
id: deleting-a-space-strands-the-placements-into-it
kind: issue
title: Deleting a new_space() strands the placements whose target reads it; the cascading delete missed them
status: closed
opened: 2026-10-22
priority: P1
parent: a-placement-names-its-target-space
closed: 2026-10-22
---


Found by the D′1 lane's viewer port (`a-placement-names-its-target-space`).

A `Node::Space` (`new_space()`) defines one `Space` output and reads
nothing; a placement reads it in its `target` slot. Deleting the space
node is a delete like any other (D10, DM7): the door admits it, each
placement whose target read it is left reading a dead variable,
reported as a strand, and refuses at evaluation. Nothing cascades on
its own: deleting a body does not take its readers either, and a
re-target into the world would be an edit the author did not ask for.
The author repairs the strand, or deletes with the cascade.

The cascade did not see them: a placement depends on its target by
name alone (`Doc::read_ports`), so `cascade_delete_order` answered
nothing for a `new_space()`, which is scheduled nowhere, and missed the
placements into a construction's own space when the construction went.
It now follows the spaces a node reads, and puts a `new_space()` last.

The viewer drops a free-move frame whose space is gone when the
document moves (`DisplayState::prune`, `AdmissionFault::NoSuchSpace`,
`crates/viewer/src/display.rs`).

Done when: `intent_dprime1_target_space::deleting_a_space_strands_the_placements_into_it`
passes — the delete strands and cascades nothing, the cascade takes the
placement first and the space last.

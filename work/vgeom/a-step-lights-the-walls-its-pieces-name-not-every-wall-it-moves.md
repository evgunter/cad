---
id: a-step-lights-the-walls-its-pieces-name-not-every-wall-it-moves
kind: issue
title: A profile step lights the walls its pieces name, not every wall its numbers move
status: open
opened: 2026-11-25
priority: P2
cost: E
---


`marks::step_focus` (`crates/viewer/src/marks.rs`) lights the walls over
the pieces the profile attributes to a step. The profile attributes a
run to the earliest step whose piece lies on it
(`crates/profile/src/structure.rs`, `Piece`'s "drawn as one segment with
an earlier piece on the same carrier"). So on the profile crate's
filleted square, the four fillet steps light 3, 2, 2 and 1 walls
(`crates/viewer/tests/focus_highlight.rs`,
`each_fillet_step_lights_its_own_arc`, which pins those counts). Editing
the radius of the last fillet moves both tangent walls beside its arc,
and lights neither of them.

The question: should a held step light every wall its numbers move (its
arc and both trimmed runs, which a radius edit changes), or the walls its
pieces name (what the names say it drew)? The second is what the names
carry today and cannot light a wall the step did not touch. The first
reads as "what this edit changes" and needs the runs a fillet trims,
which no name records per step. It is a viewer-behaviour choice; the
pinned counts go red either way it moves.

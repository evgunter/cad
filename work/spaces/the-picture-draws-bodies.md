---
id: the-picture-draws-bodies
kind: unit
title: the picture draws bodies, not placements: a body is drawn unless an operation reads it as a body
status: dispatched
opened: 2026-11-01
priority: P0
cost: H
branch: spaces/picture-draws-bodies
refs: [6209]
---


## Spec

The contract is `crates/viewer/README.md`, *The picture draws bodies*
(Ev on PR 6209, fork row 230). In short:

- **The rule.** A body is drawn unless an operation reads it as a body:
  an operator's operand (`∪ − ∩ ⊔`), a one-input operation's input, or
  a placement's source wherever the copy lands. A read of member k of a
  family takes over member k alone. Mates, selections, measures and
  assertions hide nothing. A `⊔` is drawn as its pieces.
- **Drawing.** Each drawn body is drawn in its own space through
  `editor_core::mate::solve::in_space`, the one composing door. The
  picture today iterates `world::drawn_copies` only (`pickindex.rs`,
  `scene.rs`); a construction or an operator result must be drawable
  and pickable as itself, and a pick on it seats its own operation.
- **Shown spaces** are the world and every space holding a drawn body.
  A drawn body the product slot does not reach is tinted as outside the
  product.
- **Free-move** admits any shown space under a pick except the world
  (`AdmissionFault::InTheWorld` stays; `display::instance_check`'s
  instance-only kind gate goes). Display state (probe, hide) is keyed
  by space (and by body for hide), not by instance.
- `world.rs`' vocabulary (`shown_spaces`, `drawn_copies`, `placed_var`,
  `is_drawn_copy`, `placements_of*`, `made`) is re-keyed from placements
  to drawn bodies; what no longer means anything goes.

Out of scope: what gestures insert (`a-new-body-lands-unplaced`), the
product door, and the View pane's toggles (their own units), but this
unit's display state must give U4 a per-space and per-body hide to
drive.

**Done when.** A construction nothing places is drawn and pickable at
once, tinted; a fillet of it draws only the fillet; a family placed
into an instance's space draws the members and not the instance;
member k read alone hides member k only; a body placed into the world
leaves its own space's picture while a sibling nothing reads stays;
free-move opens on a construction's space and refuses the world. Rows
for each in `crates/viewer/tests/`. Carries
`the-viewer-does-not-show-a-constructions-own-space` and
`a-family-placed-into-an-instances-space-hides-that-space` (the latter's
done-when "the instance drawn" is superseded: member 0 stands in for
it).

Review tier: **dual** (concurrent, H): the drawing rule is an
architectural decision every later gesture and the product door build
on, and it is easy to get subtly wrong (member granularity, same-space
vs cross-space reads, picks seating the right node).

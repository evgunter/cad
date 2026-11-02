---
id: a-family-placed-into-an-instances-space-hides-that-space
kind: issue
title: A family placed into an instance's own space hides that space from the picture
status: closed
opened: 2026-10-28
priority: P1
cost: M
refs: [6209]
parent: the-picture-draws-bodies
closed: 2026-11-01
---

Found in INTENT stage 3 G (PR 5954), disclosed there for review.

**What happens.** `shown_spaces` (`crates/viewer/src/world.rs:63`) shows a `Node::Space` only when no placement reads a source body in it (`:65-79`). An instance's own space is such a space. A family whose members are placed into it reads the instance's body there, so the space counts as read. The picture then draws the space only through the family's copies, and the instance standing in it is no longer drawn on its own.

**What it should do.** A space a family is placed *into* is that family's target, not a source the family copies. Being targeted should not hide it. Placements that read a body in a space should be told apart from placements that target the space, and only the first kind should hide it.

**Done when.** A row in `crates/viewer/tests/` places a family into an instance's own space and asserts the space is in `shown_spaces`, with the instance and every member drawn.

## Closed

By `the-picture-draws-bodies` (PR 6265): a body no operation reads as a body is drawn in its own space; a family placed into an instance's space reads the instance's copy, so the members are drawn and member 0 stands in for the instance.

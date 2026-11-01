---
id: the-viewer-does-not-show-a-constructions-own-space
kind: issue
title: viewer: a construction's own space is not shown (DI5's default set) because the viewer draws placements alone
status: open
opened: 2026-10-22
priority: P1
refs: [5811, 6209]
cost: H
parent: the-picture-draws-bodies
---


Found by the D′1 lane's viewer port (`a-placement-names-its-target-space`).

`docs/INTENT-STAGE3-SPEC.md` §5′.1 (The viewer): by default the viewer
shows the world and every space no placement reads a source body of,
and "a new construction appears at once as its own shown space, tinted
as outside the product, and once placed its own picture goes".

`world::shown_spaces` (`crates/viewer/src/world.rs:63`) shows the world
and every `new_space()` no placement copies a body out of, but not a
construction's own space: the viewer draws placements alone, so a
construction nothing places has no picture. A placement into a
construction's space is drawn through the copy of that construction.

Showing a construction's own space needs the picture to draw a body
that no placement copies, in its space's coordinates: the composing
door of stage E (§6).

The author's show and hide is not built either. The same §5′.1
paragraph shows "by default the world and every space no placement
reads a source body of, then the author's show and hide": a space the
author hides leaves the picture and its free-move frame, and a space
the default leaves out (one a placement copies) can be shown on its
own. The viewer has a per-instance hide
(`SessionOp::SetInstanceHidden`), which is not a space's. Both halves
are display state (DI5) and both need the same composing door to draw
a space nothing places.

Done when: a new construction is drawn at once as its own shown space,
tinted as outside the product, and goes once placed; and the author can
show and hide any space, the choice kept as display state.

**Evidence, 2026-10-26 (D′3, PR 5636).** D′3 makes the product a slot, and `docs/INTENT-STAGE3-SPEC.md` §5′.3 ("The viewer's contract after D′3", (c)) states the picture this row builds: the product slot's body plus the shown spaces, with a body the product does not read tinted as outside it. The viewer at D′3 still draws the world's placements and tints nothing. The INTENT orchestrator ruled that this row builds (c), with E's drawing. The wording "a space's bodies that nothing in it reads" is the sink-set rule that rows 80 and 85 retired, and is not the rule here.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to CHROME's charter (viewer coverage of what a document holds). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).

---
id: the-viewer-does-not-show-a-constructions-own-space
kind: issue
title: viewer: a construction's own space is not shown (DI5's default set) because the viewer draws placements alone
status: open
opened: 2026-10-22
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

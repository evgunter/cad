---
id: circle-image-envelope-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: geom-core's bounds census is red on main: pcurve_cache's circle_image_envelope (PR 3733) is a sole-bracket bound door with no roster line
status: closed
opened: 2026-10-02
priority: P0
cost: E
closed: 2026-10-02
---


## What

`geom-core`'s `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
fails on main:

> arrived or moved (owe a roster line WITH its disposition):
> `("crates/geom-brep/src/pcurve_cache.rs", "circle_image_envelope")`

The door arrived with PR 3733 (`pcert/general-circle-fitted-route`),
and no roster line was added to `crates/geom-core/tests/bounds_census.rs`.
Every PR whose diff touches geom-core runs this test and goes red on
it; TANG's PR 3752 was the first seen (run 36967200256), and it
reproduces locally on a tree that does not touch `pcurve_cache.rs`.

## The fix

Add the roster line with the door's disposition. The disposition is
the owner's call: what the bracket bounds, and whether it is sole.
Found by TANG, 2026-10-02.

## Closed

Fixed on main by commit 467d4b42f ("bounds census: roster circle_image_envelope"), landed by another lane; `crates/geom-core/tests/bounds_census.rs` carries the line.

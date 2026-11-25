---
id: unmoved-batches-of-two-spaces-merge-on-body-frame-carried-spans
kind: issue
title: PickIndex::pick_for merges unmoved batches of different spaces on spans the kernel certified on body-frame-carried rays, ungrown
status: open
opened: 2026-11-25
priority: P3
cost: E
---


## Finding

`crates/viewer/src/pickindex.rs`, `PickIndex::pick_for`: spans are
grown to hold on the caller's ray (`on_callers_ray`) only where a
display-moved group meets another. Unmoved batches of different spaces
(one `pick_face` call per space) are still merged on the kernel's
spans as answered, and a posed copy's span there is certified on the
ray carried into its body frame — the caller's only to that carry's
rounding. The narrowing is deliberate: `pick3_acceptance::
the_door_answers_the_exhaustive_walk_and_keeps_every_aimed_vertex`
pins the door's spans bit for bit against `FlatReference`, an
exhaustive walk over every part flattened, which carries the same
frames; growing there moves 3081 of its 23496 rays' spans without
changing what the row is about (the early-out prunes nothing).

The flip needs two spaces' copies drawn coincident to within a body
frame's translation rounding, at a translation that dwarfs the local
geometry and the eye — inside the session box (10³) that is a far
smaller margin than the display case, where a free move or a
placement draft can cancel a pose's translation outright. P3.

## Next step

Either grow every group's spans whenever groups meet (and teach
`FlatReference` the same growth, or compare it on survivors rather than
bits), or a witness first: `moved_copy_pick.rs`'s twin row with the two
copies in two spaces and no display map.

Sibling in the kernel's own merge:
`work/doctail/pick-face-orders-targets-on-rays-each-carried-by-its-own-body-frame.md`.

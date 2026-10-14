---
id: split-cylindrical-feature-box-copy-of-projectbox-has-drifted
kind: issue
title: split_cylindrical_feature_box.rs says it copies projectbox::build but still builds square bosses sunk 1/16 with pockets
status: open
opened: 2026-10-02
---


## What

`crates/sweep/tests/split_cylindrical_feature_box.rs`, `project_box`
(:60-90), says it is "a copy of `projectbox::build` in
`demos/tour/src/projectbox.rs` ... A change to that body is a change to
this one." It is not a copy any more:

- the bosses are square `slab`s, `(0.4375, 0.8125)` x ... from
  z = 0.1875, so they are sunk 1/16 into the floor;
- the four features after them are blind `pocket` subtracts, from
  z = 0.5625 to 1.0625.

`projectbox::build` builds round bosses (`BOSS_R = 0.1875` on
`BOSS_AXES`) standing on the floor (`BOSS_Z = (FLOOR_TOP, 0.875)`),
each union declaring its flush contacts, then four through-bores
(`BORE_R`) down the boss axes and out through the floor.

Nothing keeps the two in step: the tour is a detached cargo root that
this crate cannot call, so the doc's "a change to that body is a change
to this one" is a promise with nothing enforcing it.

## Do

Decide what the test is for. If it is the tour's body, re-copy it
(round bosses, declared unions, bores) and check that what the test
asserts still holds. If it is a standalone fixture, say that, and drop
the claim that it is a copy.

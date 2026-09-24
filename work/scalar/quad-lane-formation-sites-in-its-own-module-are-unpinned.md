---
id: quad-lane-formation-sites-in-its-own-module-are-unpinned
kind: issue
title: QuadLane is formed in the module that owns its private field and nothing pins those sites against a literal
status: open
opened: 2026-09-24
priority: P3
cost: D
---

## What

`topo::QuadLane`'s one field, `cut_face`, is private to `props.rs`'s
module, and `props.rs` forms the door in production:
`mass_properties` (~305, `Some(QuadLane::certified())`) and
`classify_shells_of` (~1905, the same). `wiring_rows`'
`holds_the_certified_quadrature` pins what `QuadLane::certified()`
holds; nothing pins that these two sites CALL it. Because the module
can write the private field, a `QuadLane { cut_face: … }` literal at
either site would compile, hold whatever it names, and leave every
wiring row green.

This is the class LANE-4P kept a comparison for on the shell door
(`props.rs` `at_rest_policy_tests::certifying_arms_are_the_doors`
compares each certifying `AtRestPolicy::shell_door` arm with
`ShellDoor::certified()`, because the arms sit in the parent module
that owns `ShellDoor`'s private field). `QuadLane`'s formation sites
have no such row. Not a regression: the base had the same two sites.

`RegionLane`'s formation sites are in `census.rs`, a different module,
so its private fields cannot be written there and it is not exposed
the same way. `crates/topo/src/validate.rs` (~3677) forms
`crate::props::QuadLane::certified()` from outside the module and is
likewise not exposed.

## Proposed

Either a row per site comparing the lane the site hands on with
`QuadLane::certified()` (as the shell door's policy rows do), or a
source row asserting that no `QuadLane {` literal appears in
`props.rs` outside the constructor. LANE-4's fold of the fifth door
(`FittedLane<T>`) should settle which, since its formation sites will
have the same question.

## Found by

LANE-4P's single review (PR 3165), style finding S3.

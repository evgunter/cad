---
id: point-in-solid-summary-has-no-caller
kind: issue
title: PointInSolidError::summary has no caller, and its doc says the census reads it
status: open
priority: P4
cost: E
refs: [tier-three-renders-every-carried-curved-containment-refusal-as-not-yet]
opened: 2026-10-06
---

## What

`boolean/solid_contain.rs` `PointInSolidError::summary` is `pub` and
its doc says it is "the one vocabulary a consumer that carries a
`&'static str` reads (the census's containment arm names the witness
it could not decide with it)". Nothing calls it: a grep for
`summary()` over `crates`, `demos`, `tools` and `benches` finds no
call. The census's containment arm reads a refusal through
`census.rs` `Undecided::of_point_in_solid` instead, which places each
arm by name.

It is also a second, unmaintained rendering of the same refusals: its
`Self::Loop(_)` arm says "the in-plane region walk ... refused" for a
corrupt loop and an in-band walk alike, and its wording ("ε",
"schedule ray", "material witness") is the kernel's, not a user's.

## Direction

Delete it, with its doc's claim; or, if a consumer wants it, give it
that caller and match `Loop` by arm the way `of_point_in_solid` does.

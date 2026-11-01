---
id: main-red-the-pseudomanifold-gate-door-is-off-the-tier1-mutation-roster
kind: issue
title: Main red: Certifier::validate_pseudomanifold is a public mutation path off the tier-1 roster
status: closed
opened: 2026-10-26
priority: P1
cost: E
closed: 2026-10-27
pr: 5824
branch: quad/main-red-triage-1027
---

**Main red.** Found by the CLEAVE lane on `cleave/flush-partners-at-a-slit-build`
after merging main `2f9e6f9e39`; the lane does not touch `props.rs`.

## What

`topo` `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
(`crates/topo/src/review_m1_pr5_internal.rs`) fails:

> public mutation path(s) that neither declare the tier-1 debug
> postcondition nor open a surgery scope nor appear on this test's
> allowlist: [`crates/topo/src/props.rs::validate_pseudomanifold`]

The door is `Certifier::validate_pseudomanifold`, which
`c6dc46433c` (the quad/measure-c fix pass, merged with
`6511057368`) put in `props.rs` (`props.rs:2370`): a `pub fn` taking
`&mut self` that keeps the validator's certificate on the certifier
and writes nothing to the body it borrows. The roster reads every
public `&mut self` door as a mutation path.

## Done when

The door declares the tier-1 postcondition, or goes on the roster's
allowlist with the argument for why tier 1 survives it, and the roster
test is green on main again.


## Closed

PR 5824. `validate_pseudomanifold` is on the roster's `ALLOWED` list. The certifier holds its body by shared reference and writes only the certificate it keeps, so no arena is written. The roster row and `the_two_door_tables_cover_the_same_surface` are green.

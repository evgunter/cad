---
id: ssi-enclose-carries-a-private-two-sum-beside-geom-core-exact
kind: issue
title: geom-brep's ssi/enclose.rs carries a private two_sum beside the public geom_core::exact::two_sum
status: closed
closed: 2026-10-26
opened: 2026-10-12
priority: P3
cost: E
refs: [4830]
---

## What

`crates/geom-brep/src/ssi/enclose.rs` defines a private `fn two_sum`,
an error-free addition. `crates/geom-core/src/exact.rs` already exports
`pub fn two_sum` with the same contract. That makes it one primitive
spelled twice. Found by a designer on FLUX's `[ev]` PR 4830.

## Fix

Call `geom_core::exact::two_sum` from `enclose.rs` and delete the
private copy, after checking the two are the same algorithm.

## Closed (2026-10-26, tracker sweep)

**Superseded by `work/issues/double-double-arithmetic-is-spelled-in-five-places.md`**,
which lists this row's copy as its fifth instance and owns the one fix
(a shared double-double type in `geom_core::exact`). The citation has
moved since: on main at `bb25fa4f` `ssi/enclose.rs` has no `two_sum`;
the private copy is `crates/geom-brep/src/ssi/compensated.rs:44`
(`two_sum`, the same algorithm as `geom_core::exact::two_sum`,
`crates/geom-core/src/exact.rs:45`), beside `fast_two_sum` (:52),
`two_prod` (:57, `two_product`'s FMA form) and the `Dd` type they
serve.

---
id: ssi-enclose-carries-a-private-two-sum-beside-geom-core-exact
kind: issue
title: geom-brep's ssi/enclose.rs carries a private two_sum beside the public geom_core::exact::two_sum
status: open
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


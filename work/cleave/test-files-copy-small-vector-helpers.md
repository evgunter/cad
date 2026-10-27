---
id: test-files-copy-small-vector-helpers
kind: issue
title: Test files copy dot, cross and sub helpers over [f64; 3] instead of sharing one
status: open
opened: 2026-10-27
priority: P4
cost: E
---

Found by PR 5812's style review.

## What

`dot`, `cross` and `sub` over `[f64; 3]` are written out again in
several of `crates/topo/tests/`'s files: `contact7_touch_sweeps.rs`, `sliver_shell_role.rs`,
`the_fin_crown_winds_as_a_solid.rs` and
`union_flush_onto_edge_contact.rs`, each its own copy.

## Owed

One home (`geom_core::Vec3`, which the files can already name, or a
shared test helper), and the copies gone.


---
id: reporting-margin-door-fixtures-write-real-looking-paths-that-collide-with-the-allowlist
kind: issue
title: reporting-margin-door's selftest plants write over real-looking source paths, so each collides the day its path is allowlisted
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), from the review of that PR.

## What

`scripts/gates/reporting-margin-door.sh`'s selftest plants write with `>` to fixed, real-looking source paths: `crates/topo/src/props.rs` (10 plants), `crates/step-import/src/error.rs` (4), `crates/sweep/src/blend.rs` (2), `crates/topo/src/lib.rs` (2), `crates/geom-core/src/predicate.rs`, `crates/topo/src/extra.rs` and `crates/topo/src/samples.rs`. The clean fixture plants each allowlisted path first. When a real change allowlists one of those paths, the plant overwrites what the clean fixture put there, and the selftest goes red for a reason unrelated to what it tests. That already happened once: PR 6820 allowlisted `crates/topo/src/validate.rs` as a mint site, the clean fixture's prose file overwrote its mint, and the nightly went red. GREEN made that one write append (PR 6921).

## Owed

Plants write to paths no allowlist can name (a fixture-only crate or file name), or append, and the clean fixture refuses a plant path that is also an allowlist entry, so that the collision is diagnosed rather than met.

---
id: main-red-cone-torus-pose-matches-unit-apex-variants-with-struct-patterns
kind: issue
title: Main red: clippy -D warnings fails on geom-brep's cone_torus_pose.rs, which matches the unit variants ApexContact::Node and ::Touch with struct patterns
status: open
opened: 2026-11-23
priority: P1
cost: C
---


Found by HELDRECORDS' elbow lane (2026-11-23) running
`cargo clippy --workspace --all-targets --all-features -- -D warnings`
on `heldrecords/elbow` after merging `origin/main`. The branch has no
diff from `origin/main` in `crates/geom-brep` (sources or tests), so the
red is main's.

## Measured

`crates/geom-brep/tests/cone_torus_pose.rs:229-230` matches
`ApexContact::Node { .. }` and `ApexContact::Touch { .. }`, but
`ApexContact` (`crates/geom-brep/src/pair_section/cone_torus.rs:144`)
declares both as unit variants. `clippy::unneeded_struct_pattern`
fails the build under `-D warnings` (geom-brep's `all` test target).

## The fix

Drop the struct patterns: `ApexContact::Node` and `ApexContact::Touch`.

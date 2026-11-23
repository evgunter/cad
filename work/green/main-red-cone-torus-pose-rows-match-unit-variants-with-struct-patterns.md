---
id: main-red-cone-torus-pose-rows-match-unit-variants-with-struct-patterns
kind: issue
title: main is red at clippy: cone_torus_pose's rows match ApexContact's unit variants with struct patterns
status: open
opened: 2026-11-23
---


`cargo clippy -p geom-brep --all-targets --all-features -- -D warnings` fails on `origin/main` at `ffb8d91ae5`. `clippy::unneeded_struct_pattern` fires twice in `crates/geom-brep/tests/cone_torus_pose.rs`:

- `:229`: `KtStance::Apex(ApexContact::Node { .. })`;
- `:230`: `KtStance::Apex(ApexContact::Touch { .. })`.

`ApexContact::Node` and `ApexContact::Touch` are unit variants. The arms were last written by `cac8518856` ("sectcone fix pass"). The fix is `ApexContact::Node` and `ApexContact::Touch` with no braces.

Found by `oneround/lever-sites-topo` (PR 7228), which does not touch the file. The workspace clippy (`--workspace --exclude viewer --all-targets --all-features -D warnings`) stops on it there as on main.

---
id: a-crossing-germ-re-decides-the-faces-parallelism-the-fan-decided
kind: issue
title: topo: insert::germ_dir decides the two faces' parallelism again (bool_germ_line) on the margin fan::meeting already decided (bool_faces_parallel)
status: open
opened: 2026-11-09
priority: P3
cost: E
---



Found by SEAL's `a-corner-reads-only-its-own-sectors` sweep (corner sites that decide a plane-parallel predicate again).

## What

`insert::germ_dir` (`crates/topo/src/boolean/insert.rs:2522`) decides `bool_germ_line` on `Margin::levered(|n_a × n_b|, min(arm_a, arm_b))` (`insert.rs:2533`). Its own comment calls this "the same margin `pair_search` read as definite before it sent this pair down the crossing path". That earlier reading is `fan::meeting`'s `bool_faces_parallel` (`crates/topo/src/boolean/fan.rs:205`), a magnitude decide on the same norm at the same arm.

A Zero there is a `ClassificationInvariant` ("surviving crossing record on coplanar sector faces", `insert.rs:2537`), and an in-band reading is an escalation. Both are the second decision of one fact on one datum, each logged under a different name.

## Shape

`fan::meeting` hands `Meeting::Rays` the decided margin, or the line it read, and `germ_dir` takes it rather than deciding it again. The invariant then goes.

`vtxfac`'s pierce `bool_germ_line` (`held_normal × plan.normal`, `crates/topo/src/boolean/vtxfac.rs:2427`) was not checked against an earlier reading.

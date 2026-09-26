---
id: on-pool-thread-pool-helper-has-six-homes
kind: issue
title: the explicit-width rayon pool test helper (on_pool / ThreadPoolBuilder) is spelled in six places
status: open
opened: 2026-09-25
priority: P4
cost: E
---


## The finding

A test that reads a result at an EXPLICIT rayon width builds a
`rayon::ThreadPoolBuilder` pool of `n` threads and `install`s the
closure on it. That helper, the same five lines each time, is spelled
at six sites across four crates:

- `crates/editor-core/tests/fixture/mod.rs`, `on_pool` (also mounted by
  `crates/viewer/tests/` through its `fixture` symlink);
- `crates/sweep/tests/common/mod.rs`, `on_pool`;
- `crates/mesh/tests/errors.rs`, inline in a row;
- `crates/mesh/tests/d9_mesh_goldens.rs`, a local helper;
- `crates/mesh/tests/k_funnel_composition.rs`, a local helper;
- `crates/topo/src/props.rs`, inline in the `face_walk_composition_tests`
  unit module (`refusal_at`).

`gather/parallel-node-map` (PR 3145) folded one copy
(`m10_sym_drive_memo_interval.rs`) into editor-core's fixture. That made
editor-core's tests single-homed but left the class across crates as it
was.

## What a fix is

One `on_pool(threads, run)` in `crates/test-utils`, called from all six
sites. The catch is that `test-utils` is a dependency-free leaf by
charter ("A dependency-free leaf crate named by [dev-dependencies]
only" in each manifest's comment), so this helper would give it its
first dependency, `rayon`, at the workspace pin. That is a charter
question for whoever owns `test-utils`. The fallback is a helper module
per crate, which is what already exists.

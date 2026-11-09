---
id: main-red-rustdoc-ops-link-and-demo-tour-since-the-finished-body
kind: issue
title: main red: rustdoc rejects topo's boolean/ops.rs link to ValidationError::VolumeUncomputable and demos/tour no longer compiles (AtRestBody), since PR 6371
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`:

- `crates/topo/src/boolean/ops.rs` (the module doc above `face_kinds`, line 5703): `unresolved link to ValidationError::VolumeUncomputable`. PR 6371 (`onehome/finished-body`, merge `80a5227c5a`) removed the file's top-level `use crate::validate::{… ValidationError …}`, so the bare path no longer resolves there.
- `demos/tour` does not compile (the doc gate's `demos/tour/Cargo.toml` pass):
  - `src/bodies.rs` `bud_rim`: `fillet_edges(…).body` is now an `AtRestBody<S>` where the fn returns `Body<S>` (E0308). `AtRestBody` reached the blend's output in PRs 6371 and 6584 (`onehome/finished-tail`).
  - `src/probe.rs` (`let boxbody = acc.body.clone()`): E0282, type annotations needed.
  - `src/probe.rs` (`geom_core::UNNAMED_PREDICATE`): E0433, `geom_core` is not a dependency of the tour. This one was already red on 11-07 (run 42342466395).

Not BOXES'. PRs 6371 and 6584 merged in the window after the 11-07 nightly, and the 11-07 rustdoc job did not report the `ops.rs` link or the `bodies.rs` error.

(BOXES nightly-reds triage, 2026-11-09)

---
id: split-and-mirror-take-a-point-the-plane-kind-forgets
kind: issue
title: SplitPlane and Reflection still take a point the plane kind forgets
status: open
opened: 2026-10-23
priority: P3
cost: M
---

Found by the designer pair on `a-plane-poses-foot-outgrows-the-coincidence-doors-budget` (record on `analysis/design-fork/msolve-plane-foot-sym-budget`), and filed by its implementer on PR 5624.

A plane pose's value is `{ normal, offset }` (`PoseValue::Plane`, `crates/editor-core/src/pose.rs`). The kernel's split and mirror still take a point and a normal: `topo::SplitPlane { origin, normal }` (`crates/topo/src/splitting/mod.rs`) and `topo::Reflection::across_unit(point, normal)` (`crates/topo/src/mirror.rs`). So `plane_of` in `crates/editor-core/src/verbs/split.rs` and `wire_mirror` in `crates/editor-core/src/eval/wire.rs` hand them the plane's foot, `offset · normal`.

Taking `{ normal, offset }` would match the value. The kernel's margins would read `p · n − c` instead of `(p − o) · n`. At `Sym` that is one factor of the normal fewer than `(p − n c) · n`.

The designers split on whether this pays on its own. The section face's carrier still stores an origin, and PR 5624 makes rung 1 prove the split's own rows without the replay.

**Done when** the two take the plane equation, or a measurement shows it buys nothing and the item closes on it.

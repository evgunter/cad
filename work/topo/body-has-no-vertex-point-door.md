---
id: body-has-no-vertex-point-door
kind: issue
title: the vertex-to-point chain vertices().filter_map(get_vertex).filter_map(get_point) is re-spelled by a demo and the sweep suites; Body has no door for it
status: open
opened: 2026-10-01
priority: P3
---

Found by `linalg/doors` (PR 3710) while rewriting
`demos/tour/src/diechamfer.rs` to compare clouds by nearest match.

## The chain

```rust
body.vertices()
    .filter_map(|(k, _)| body.get_vertex(k))
    .filter_map(|v| body.get_point(v.point))
```

Hits at the PR's base (grep `filter_map(|v| .*get_point(v.point))`
over `crates demos`):

- `demos/tour/src/diechamfer.rs` `vertex_points`
- `crates/sweep/tests/verbs_chamfer.rs` (around :34)
- `crates/sweep/tests/sf2a_r1_head.rs` `centroid` and `sorted_points`
- `crates/sweep/tests/sf2a_r1.rs` (around :103)
- `crates/sweep/tests/review_chamfer_r1_probes.rs` (around :75)

The pattern cannot see the same walk spelled with `map(|(_, v)| …)`
over `vertices()` directly, or with `unwrap`/`expect` in place of
`filter_map` (e.g. `review_ring_clearance_r1_probes.rs`,
`shell8_dump.rs`); those are the same question.

## What a door would be

A `Body` readback such as `vertex_points(&self) -> impl Iterator<Item =
(VertexKey, &Point3<T>)>` in `crates/topo/src/body.rs`, beside
`get_vertex`/`get_point`. The `filter_map` spelling also silently drops
a vertex whose point key dangles, which a door could refuse instead.
A demo is an outside consumer, and it re-spells this because no door
exists.

`crates/topo/src/body.rs` is unowned ground in this program's
keep-out; a unit that mints the door draws that fence.

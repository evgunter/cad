---
id: a-turn-is-still-spelled-outside-its-carriers-period
kind: issue
title: A turn is still spelled outside its carrier's period: edge_join's own TAU and kind match, on_carrier's domain, bare tau arithmetic in topo, geom-brep and mesh, mesh's unwrap_tie integer, certify's conic-into-line arm
status: open
opened: 2026-11-10
priority: P3
---

Filed from PR 6776's review: the spellings of a turn that are left.

## What

- **F1:** `crates/topo/src/boolean/edge_join.rs:1206-1214` spelled `T::from_f64(TAU)` and matched `Circle | Ellipse | Spiric` itself, then ran `t1 − period`, rather than asking `Curve3::period`. At `Interval`, `from_f64(TAU)` is a point, not τ's enclosure.
- **F2:** `crates/geom/src/curves/on_carrier.rs:633`, `domain()`, restated which kinds repeat.
- **F3: bare `T::tau()` / `TAU` arithmetic.**
  - topo: `splitting/join.rs:751, 780`, `boolean/fate.rs:902-904`, `boolean/rim_wedge.rs` (five `T::from_f64(TAU)` points), `boolean/trace.rs:266`, `boolean/region.rs:1022, 1058`, `boolean/reduce.rs:4038`, `boolean/section_cert/ruling.rs:869`;
  - geom-brep: `props/curved.rs:3044, 4472, 5813`, `half_angle.rs:836`.
- **F4:** `crates/mesh/src/walk.rs:384`, `unwrap_tie`, read a whole-period integer off a value (`k.round()`, `k.floor()`, `k.ceil()` of `(prev − raw)/τ`). C4 says no whole-period integer is read off a value.
- **S1:** `crates/geom-brep/src/certify.rs:4026-4031` routed `(None, Circle | Ellipse | Spiric)` into the line arm, so a conic answering `None` would have been certified as a line.
- **S2:** where a site holds a `Curve3`, it asks the carrier, not `conic_period()`.
- **S4:** `topo::pcurves::branches_astride` took a bare length, not a `Period`.

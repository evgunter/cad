---
id: a-turn-is-still-spelled-outside-its-carriers-period
kind: issue
title: A turn is still spelled outside its carrier's period: edge_join's own TAU and kind match, on_carrier's domain, bare tau arithmetic in topo, geom-brep and mesh, mesh's unwrap_tie integer, certify's conic-into-line arm
status: closed
opened: 2026-11-10
priority: P3
closed: 2026-11-10
branch: onecopy/turn-spellings
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

## Resolved (2026-11-10)

- **F1:** edge_join's closed join asks `carrier.period()`. A carrier with none takes the old `JoinCarrierUnsupported` arm.
- **F2:** `on_carrier`'s `domain()` asks `Curve3::period()`.
- **F3, sites that hold a carrier, ask it:**
  - `splitting/join.rs`, through a new `SectionConic::period()` that reads its carrier's;
  - `reduce.rs:4038`;
  - rim_wedge's edge-arc angle;
  - step-import's `normalize.rs` rim seam.
- **F3, a circle with no `Curve3`, states a circle's period** through `geom::conic_period`: the trace's circle, region's arc, and rim_wedge's rim (a private `Rim::period()`, replacing five `T::from_f64(TAU)` points, so at `Interval` the rim now reads τ's enclosure).
- **F3, angles by construction, read `Period::angle()`:** fate's turning, the ruling folds, `props/curved.rs`'s three turns about an axis, and half_angle's walk.
- **F4:** mesh's `unwrap_tie` reads every copy through `Period::near`: about `prev`, and in the tie zone about half a turn either side. It reads no whole-period integer off a value. Each copy is `raw + times(k)`, the same product as before, so no bits move.
- **S1:** certify's `(None, Circle | Ellipse | Spiric)` is `unreachable!`, and the line arm takes lines only.
- **S2:** the step-import site, which holds a `Curve3`, asks it. The sites that hold only a `&dyn Conic` (`conic_quadric::through_the_apex`, `pose::window`) read `geom::conic_period`, the conic kind's own statement.
- **S4:** `branches_astride` takes a `Period` and reads its `half()`.

**Measured:** the ci profile on geom, geom-brep, topo, mesh, step-import, sweep and editor-core passed 12983 of 12983 at default ε, at 1e-6 and at 1e-12, so no row moved at `f64`.

**Not folded:** rim_wedge's `τ·floor(x/τ)` reductions, the same C4 class as F4, move bits under `at_or_above`. They are filed as `rim-wedge-reduces-angles-by-a-floor-of-the-value`.

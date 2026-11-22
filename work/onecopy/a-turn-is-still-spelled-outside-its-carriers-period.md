---
id: a-turn-is-still-spelled-outside-its-carriers-period
kind: issue
title: A turn is still spelled outside its carrier's period: edge_join's own TAU and kind match, on_carrier's domain, bare tau arithmetic in topo, geom-brep and mesh, mesh's unwrap_tie integer, certify's conic-into-line arm
status: closed
opened: 2026-11-10
priority: P3
closed: 2026-11-10
branch: onecopy/turn-spellings
pr: 6788
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

**A carrier states its own period, by type.**
- `geom::Circle::period` and `Ellipse::period` are total. `Curve3::period` is built from them and `spiric_period`, so which kinds repeat is decided once.
- Sites that hold a matched circle read the circle's period, with no `Option`: step-import's rim seam, reduce's `boundary_meets_circle_only_at` (which now takes the `Circle`), rim_wedge's edge arc, `implicit::circle_curvature_bound` and `certify`'s self-loop.
- certify's no-period arm branches on `Curve3::period()` being `None`, and restates no kind list.
- `cover/readings`' `Curve3::cover` reads the period, not a kind list.

**Sites that hold only a `Conic`, or a circle with no `Curve3`**, read `geom::conic_period`: trace, region, rim_wedge's rim (the private `Rim::period` is dropped, S3), `boolean/mod.rs`'s rim locus, `slit::even_points`, `separating`, `implicit::conic_arc_residual_range` and `splitting/join` (through `SectionConic::period`, which reads its carrier). `edge_join` and `pair_section`'s closed components ask their `Curve3`.

**Angles by construction read `Period::angle()`** (through a `use`, S4): fate, the ruling folds, `props/curved.rs`, the half-angle walk (one `turn` for its cut and its close), and certify's revolution.

**mesh's `unwrap_tie`** reads every copy through `Period::near`, with no integer off a value. This moves values in two places, pinned by `unwrap_tie_reads_its_zone_edge_as_the_period_reads_it`:
- At the quarter-turn zone edge, `Period::near`'s copy decides the zone where the old `(prev − raw)/τ` integer did. Within an ulp of the edge the answer can move by a whole turn: at `raw` −13.837166941154068, `prev` 0.3, `anchor` 2.3 it reads −1.2708 where the fold read 5.0124.
- A signed zero keeps its sign: −0.0 about 0.0 reads −0.0, where the fold read +0.0.

**`branches_astride`** takes a `Period` (S4). chart_iso's `pub const TAU` is retired.

**Value changes besides `unwrap_tie`:**
- At `Interval`, a `from_f64(TAU)` point became τ's enclosure (edge_join, rim_wedge, slit, `boolean/mod.rs`, `pair_section`).
- At `Sym`, `T::tau()` is the node `2·π` where `from_f64(TAU)` was a literal.
- At `f64` nothing else moves: the ci profile on geom, geom-brep, topo, mesh, step-import, sweep and editor-core is green at all three ε.

**The gate:** `scripts/gates/period-spelling.sh`, table `period-spelling.tsv`, counts `from_f64(TAU)`, `2.0 * PI` and bare `TAU` in production code outside the homes, per (file, function) with a class. The `owed` rows, the C4 integer reads, and the `T::tau()` sites a carrier is owed are filed as `a-turn-read-off-a-value-or-a-literal-remains-at-f64`. rim_wedge's floors, with the Interval straddle (finding 3), are `rim-wedge-reduces-angles-by-a-floor-of-the-value`.

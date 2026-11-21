---
id: a-place-is-an-f64-argument-answered-on-the-value-channel
kind: unit
title: A place is an f64 argument to the evaluator, answered on the value channel
status: spec
opened: 2026-11-20
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

## Scope

- `Real` gains an associated `Value`, the scalar with its tangent removed (`f64` at `Dual<f64>`, `Interval` at `Dual<Interval>`, the scalar itself at `f64`, `Probe`, `Interval`, `Sym`), and a read down to it; curves and surfaces gain a view on it (the generic form of the `map_scalar(T::value_channel)` copy `topo`'s `SplineWall` keeps).
- Evaluators take a place as `f64` and answer on the value channel. About 55 calls: `curve_face/composite.rs`, `curve_face/spline_face.rs` `settle`, `ring_path/spline_wall.rs` `paths`/`piece`, `pcurve_cache/spline_incidence.rs`, `sweep` blend `reach` cell corners, `geom` `project_seed`, `spline_region::carrier_walk`, `chart_region::interior_witness`, `periodic_nurbs.rs`, and the box centres and patch splits of `spline_trim` (`bezier_cells`, `Patch::split`, `refined`) and `spline_incidence::Net::of`.
- A module whose only exits are decisions (`spline_trim`'s `Patch<T>` ends in `Side`/`Reading`) runs whole at `T::Value`.
- `edge_nurbs` `Schedule::station` holds `T` ends and interpolates in `f64`; a fixed fraction between `T` ends is computed at `T`, as `spline_incidence`'s scheduled samples are.

## Not this unit

A foot or root the body is built on: that is the solved-value question, open with Ev. A site here that turns out to be built on is reported, not forced.

## Measure first

22 of these sites were traced to a decision or a parity read; about 9 were not. Trace the rest before the sweep and report any that keeps what it evaluates. Whether the view on the value channel allocates at a certifying scalar.

## Decides

The view's shape (borrowed where `Value = Self`). Whether `Sym` needs a computed place opaque inside the evaluator.

## Review

Letter H, tier dual: a new associated type on `Real` and a new evaluator surface, which every later place site will build on.

## Note from `a-bound-to-decide-against-never-becomes-a-t`

`topo::pcurves_rings` `Ring::side` keeps `T::from_computed(room[1])` for the `v` window `v ± rv` that the chart's arms are read over: a place, so it belongs to this unit. The room decision beside it now uses `Margin::metered_bound`.

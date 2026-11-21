---
id: a-place-read-inside-a-reading-held-at-t-needs-the-reading-on-the-value-channel
kind: issue
title: A place read inside a reading held at T: the reading has to run on the value channel first
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Found by `a-place-is-an-f64-argument-answered-on-the-value-channel`. Each site below is a place by DL5.2 (`crates/geom-core/README.md`): an `f64` the code chose to evaluate at, whose reading ends in a decision. Each still enters as `T::from_computed`, because what it evaluates is handed to a reading that runs at `T` beside values with real tangents, so handing it to a place-taking evaluator (`geom`'s `eval_at`, which exists only at a scalar that carries no tangent, after the geometry's `value_view()`) means moving that whole reading there, not the site.

- `topo/src/ring_path/spline_wall.rs`, `SplineWall::outward` (two calls) and `SplineWall::piece` (three): the outward normal becomes the island reading's `left` (`chord_join.rs`, `arriving`) and the iso pieces become `Path<T>` beside a quadric's, both read by the ring parity at `T`. Neither is stored in a body: the foot is a solved value only evaluated at, so these are the place row's (DL5.2, the paragraph after the table), not the solved-value door's. The foot read in `SplineWall::paths` (`on_wall`) is moved.
- `topo/src/pcurves_rings.rs`, `Ring::side`: the window `v ± rv` the chart's arms are read over; the arms go to `ray_parity_as` with the chain's `T` pieces.
- `topo/src/chart_region.rs`, `interior_witness`'s stage 2 (two calls): the candidate `(x, y)` is a plane point handed to `contain::contfp` on the bodies at `T`. The bound is `Decide + Bounds`, which admits a dual, so `eval_at` (bounded `Value = T`) is not available. A bound that excludes a dual and states `Value = T` would make it so; `Certifies` as PR 7190 declares it (`Decide + Sealed`) does not state the equation.
- `sweep/src/blend/reach.rs`, `Cell::new` and the two `corner` closures in `band_reach` (three calls): box-arithmetic cell corners, read into margins `face_bound` and `metered` return as `T` and the band reach subtracts at `T`.
- `geom-brep/src/pair_section/ruling.rs`, `Cell::new` (two calls): a cell's end angles, at `T: Real`, where the cell's cosine and sine are read. Traced: every use of a cell is a decision (`Ruling::reads`, `escalate` and `past` over `discriminant` and `meeting`, `ruling.rs` about lines 771 to 1004), so the angles are places, read by a ruled reading held at `T`.

Whether any of these readings keeps what it evaluates was traced by reading only (consumers named above); none was found stored in a body.

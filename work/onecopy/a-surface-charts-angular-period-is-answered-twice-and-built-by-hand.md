---
id: a-surface-charts-angular-period-is-answered-twice-and-built-by-hand
kind: issue
title: A surface chart's angular period is answered at two doors (one gives a plane τ) and built by hand at about thirty sites
status: open
opened: 2026-11-10
priority: P3
cost: M
---



Filed from `onecopy/carrier-period`, the unit that made a curve carrier state its own period (`geom::Curve3::period`) and made every curve-span reader ask it. That unit's sweep of `Period::new(T::tau())` (and its `Interval::`/`f64::` spellings) over `crates/*/src` found 44 production hits outside `geom_core::period` itself. The curve-carrier ones were ported. What is left is mostly the surface side.

**Two doors answer a chart's `u` period** (`crates/topo/src/pcurves.rs`):
- `chart_period` (~`:5048`) decides by kind: `τ` for every surface of revolution, a closed spline chart's knot domain, and `None` for a plane;
- `chart_u_period` (~`:1800`) answers `τ` for a plane too. Its sibling's doc says so, and a reader that must tell a plane from a periodic chart is told to ask `chart_period`.

**Built by hand, a surface's or an angle's turn:**
- topo: `chart_region.rs:2964`, `:2977`; `chart_window.rs:455`, `:559`; `pcurves.rs:1808`, `:1825`, `:2020`, `:2024`, `:8941`; `boolean/solid_contain.rs:1257`; `offset_axial.rs:2932`; `shell.rs:5297`; `boolean/conic_quadric/mod.rs:550`;
- geom-brep: `pcurve_cache.rs:322`, `:6439`, `:9022`, `:9061`, `:11290`, `:12057`; `pcurve_cache/projected.rs:1401`, `:2989`; `props/curved.rs:2107`, `:3282`, `:4613`, `:4747`, `:5433`, `:5472`; `edge_nurbs.rs:1084`; `chart_singularity.rs:1329`; `half_angle.rs:876`;
- curve roots read as an angle: `curve_face/circle_roots.rs:381`, `circle_torus.rs:706`, `plane_crossing.rs:501`;
- other crates: `geom/src/azimuth.rs:377` (an azimuth's own `near`); `profile/src/seg.rs:967` and `path/verbs.rs:427`; `sweep/src/revolve/mod.rs:1113` and `tube.rs:337`.

Plus `Period::new(tau)` over a local `tau = T::tau()`: `chart_region.rs:1694` and `chart_window.rs:381`.

**The question.** Is the right home a surface kind stating its chart's periods, one door the way `Curve3::period` is for a curve, which the chart readers ask? Then `chart_u_period`'s plane answer retires. For an angle that is no carrier's parameter (an azimuth, a revolve angle, a root's angle), is it a `Period::turn()` constructor in `geom_core`, so that a turn is one name rather than a constructor call with `τ` written at each site? Each site above needs a disposition against the answer: a surface chart's, an angle's, or a curve root's, which could ask its curve.

---
id: a-surface-charts-angular-period-is-answered-twice-and-built-by-hand
kind: issue
title: A surface chart's angular period is answered at several doors and built by hand at about forty sites
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
pr: 6776
---



Filed from `onecopy/carrier-period` (PR 6754), the unit that made a curve carrier state its own period (`geom::Curve3::period`, delegating to `Conic::period` and `geom::spiric_period`) and made the curve readers that hold a carrier ask it. Line numbers are as of that PR's last fix pass.

**The doors that answer a surface chart's analytic periods by kind.** They agree today:
- geom-brep `pcurve_cache::angle_periods` (~`:6474`, private): `(u, v)`, each `τ` where the channel is an angle;
- topo `pcurves::chart_u_period` (~`:1819`) and `chart_v_period` (~`:1836`): `u`'s and `v`'s, plus a net's knot length where the net is decided closed;
- topo `pcurves::ChartPeriods::stated` (~`:2031`, `:2035`): both, band-free;
- topo `pcurves::chart_period` (~`:5119`): `u`'s by kind. Its doc says `chart_u_period` answers `τ` for a plane. It no longer does (it returns `None` there), so the two answer alike for every kind;
- topo `pcurves::polar_arm`: whether `v` is an angle, as its radius;
- `topo::chart::Chart::v_periodic`, which answers a different question under the same word (whether the domain wraps). `pcert/chart-channel-periods-are-spelled-in-several-places` holds that half, and this row overlaps it on the doors above.

**Built by hand, a surface chart channel's turn** (`Period::new(τ)` with `τ` written at the site):
- geom-brep:
  - `chart_singularity.rs:1329` (`sum_walk`);
  - `pcurve.rs:292` (`ellipse_pcurve_on_cylinder`);
  - `pcurve_cache.rs:322` (`angle_rate`), `:9057` (`phase`), `:9096` (`turns`), `:11536` (`torus_oblique_circle`), `:12300` (`spiric_chart_pcurve`);
  - `pcurve_cache/projected.rs:1400`, `:2988`;
  - `props/curved.rs:3279`, `:4610`, `:5430`, `:5469`;
  - `edge_nurbs.rs:1084` (a rim arc's turn on a spline chart).
- topo:
  - `chart_region.rs:1747`, `:3017`, `:3030`;
  - `chart_window.rs:381`, `:455`, `:559`;
  - `solid_contain.rs:1260`;
  - `offset_axial.rs:2929`;
  - `shell.rs:5314`.
- mesh: `walk.rs:1125`, `:1154`, `:1175`, `:1187`, `:1289`, `:1306`, `:1343` (`loop_polygon`'s chart unwraps).
- step-import: `chart.rs:285` (`unwrap`), `entities.rs:2090` (`loop_u_wrap`), `normalize.rs:224` (`chart_direction`).

**Built by hand, a curve root's angle** (the root of a circle's or conic's parameter; each could ask its curve where it holds one):
- geom-brep `curve_face/circle_roots.rs:369`, `circle_torus.rs:687`, `plane_crossing.rs:501`;
- geom-brep `half_angle.rs:832`.

**Built by hand, a bare angle** (no carrier's or chart's parameter):
- `geom/src/azimuth.rs:377` (an azimuth's own `near`);
- geom-brep `props/curved.rs:2107`, `:4744`;
- profile `seg.rs:967`, `path/verbs.rs:431`;
- sweep `revolve/mod.rs:1110`, `revolve/tube.rs:337`.

**The question.** C4 (`crates/geom-brep/README.md`) already says every whole-period question uses the channel's own period, which the chart owns. So, first, is the one home a `geom::Surface` method stating each analytic channel's period, which every door above asks and every surface-chart site above reads? Second, for a bare angle, is there a constructor in `geom_core` (`Period::turn` is taken: it decides a turn), so that `τ` as a period is one spelling?


## Resolution (`onecopy/surface-chart-period`)

Both questions are answered as code. Each applies C4 (`crates/geom-brep/README.md`: "every whole-period question uses the channel's own period, which the chart owns"); no design text changes.

- **One home for an analytic chart's periods: `geom::Surface::angle_periods`.** It is matched by kind and returns `None` on a spline chart, whose periods are decided at a band. geom-brep's private `angle_periods` retires into it. topo's `chart_u_period`, `chart_v_period`, `ChartPeriods::stated` and `polar_arm` ask it, and each answers exactly as before for every kind (by reading). `chart_period`, which answered alike, retires into `chart_u_period`. `Chart::v_periodic` becomes `Chart::v_domain_wraps`, named for its own question.
- **One spelling of an angle's turn: `geom_core::period::Period::angle()`.** `geom::conic_period`, `geom::spiric_period` and `Surface::angle_periods` state it. Every production `Period::new(τ)` in the lists above reads it; test fixtures keep their spelled inputs.
- **The kind-generic chart readers ask the chart.** `chart_window`'s `run_azimuth_images` and `nearest_branch` take the azimuth's period from `Surface::angle_periods`, refusing typed where the chart has no azimuth. `edge_nurbs::arc_rim_measured` reads its circle carrier's `Curve3::period`.
- **The rest read `Period::angle()` inside one kind's own code**, where the channel is an angle by construction: a torus's or a spiric's chart pcurve, the projected lane, the `props` charts, mesh's and step-import's chart unwraps, the curve roots, and the bare angles. Asking the chart there would add a second statement of the kind the site already matched.

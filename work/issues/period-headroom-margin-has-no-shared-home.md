---
id: period-headroom-margin-has-no-shared-home
kind: issue
title: The period-headroom margin Margin::levered(T::tau() - span, lever) is spelled at ~18 sites across 6 crates under ~12 predicate names with no shared home
status: open
opened: 2026-09-06
---


The decide "this span does not definitely exceed one period" —
`Margin::levered(T::tau() - span, lever)` classified at the linear
band, `Zero | Positive` admit, `Negative` refuses typed, the ambiguity
band escalates — is certification's `interval_span_winding`
(`crates/geom-brep/src/certify.rs:1496`), and it is re-spelled by
hand wherever a span is read that certification did not bound: a
reconstructed span, a chart-inverted pair, a hand-built loop, a
boolean trim window. Every re-spelling carries its own predicate name,
its own `Margin::levered(...)` expression and its own `match` on the
three dispositions, and nothing ties them together: a change to the
shape at one site (the lever, the sign convention, which of `Zero` /
`Positive` admits) is invisible to the others. MESH-12 (PR 1617)
added two more of them for the sphere parse and, in its fix pass, a
third for the forward half `0 < Δt` — which is the second half of
the same certification premise, `interval_span_forward`
(`certify.rs:1488`). Of the sites below, `certify.rs`,
`props/curved.rs:1638` and `fold_chain` (`props/curved.rs:2288`)
decide both halves; whether each of the others does was not read for
this filing, and at any that does not, a reversed span reads as
headroom `τ + |Δt|`, definitely `Positive`, and is admitted — the
defect MESH-12's fix pass closed at the sphere parse.

The sites, each line read at this head (merge base `f729fbaf4`,
PR 1617's fix-pass tree), with its predicate name:

| site | name | lever |
|---|---|---|
| `crates/geom-brep/src/certify.rs:1496` | `interval_span_winding` (circle) | carrier radius |
| `crates/geom-brep/src/certify.rs:1515` | `interval_span_winding` (ellipse) | minor semi-axis |
| `crates/geom-brep/src/pcurve_cache.rs:2630` | `pcurve_azimuth_period` | `arm` |
| `crates/geom-brep/src/pcurve_cache.rs:3335` | `pcurve_azimuth_period` | `u_arm` |
| `crates/geom-brep/src/props/curved.rs:1647` | `props_meridian_span_winding` | sphere radius |
| `crates/geom-brep/src/props/curved.rs:2296` | `props_meridian_pieces_winding` | torus minor radius |
| `crates/topo/src/boolean/reduce.rs:2120` | `bool_split_span_period` | radius |
| `crates/topo/src/boolean/solid_contain.rs:892` | `bool_cone_trim_period` | `max(|v0|, |v1|)` |
| `crates/topo/src/boolean/solid_contain.rs:1118` | closure over `name` (two callers) | `lever` |
| `crates/topo/src/boolean/solid_contain.rs:1567` | `bool_wall_trim_period` | `lever` |
| `crates/topo/src/boolean/solid_contain.rs:1873` | `bool_sphere_trim_meridian_span` | radius |
| `crates/topo/src/boolean/solid_contain.rs:1966` | `bool_sphere_trim_period` | radius |
| `crates/topo/src/boolean/contain.rs:498` | `bool_contact_arc_span` | radius |
| `crates/topo/src/boolean/contain.rs:639` | `bool_curved_contain_period` | radius |
| `crates/sweep/src/revolve/mod.rs:707` | `revolve_angle_headroom` | `r_max` |
| `crates/sweep/src/revolve/tube.rs:434` | `tube_window_headroom` | `arm` |
| `crates/topo/src/chart_region.rs:1621` | `chart_region_cyl_wrap` (spelled as the excess `span − τ`, sign reversed) | radius |
| `crates/topo/src/chart_region.rs:2741` | `chart_region_seam_span` (the excess form again) | radius |

Eighteen sites in ten files across three crates (`geom-brep`, `topo`,
`sweep`; the two reviews that found the class counted six, by module).
Beside them, not decides but the same quantity read raw:
`crates/sweep/src/blend/surgery.rs:2346` compares
`(T::tau() - (st1 - st0)).lo() <= 0.0` on an interval directly, and
`crates/topo/src/chord_join.rs:2849` halves `TAU - width` in `f64`.
The sweep that produced this list matched the spelling
`tau() -` / `- T::tau()` / `TAU -` in non-test sources; a headroom
formed through a named helper or as `period - span` with a local
`period` binding would be missed.

What a shared home would be: one function on `Decide` — the span, the
lever, the band, a predicate name — returning the three-way
disposition (or the typed `Result` with the caller's error), with the
forward half decided in the same call so a site cannot take one half
without the other; `predicate-dimension-audit.md` then documents the
helper's key once and each site's row cites it. The names stay per
site (they are what the funnel records), the arithmetic does not. The
MESH-12 fix pass is the immediate evidence for the cost of the status
quo: the forward half was missed at the sphere parse because the
winding half was copied without it, and the same omission stands at
the twelve `bool_*` / `chart_region_*` / `*_headroom` sites above
until each is read.

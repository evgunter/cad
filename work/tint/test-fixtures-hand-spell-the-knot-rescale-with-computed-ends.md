---
id: test-fixtures-hand-spell-the-knot-rescale-with-computed-ends
kind: issue
title: Six test fixtures hand-spell the knot rescale with computed ends instead of the on_domain door
status: open
opened: 2026-09-12
priority: P3
cost: E
---

## Finding

Filed by the SCALAR lane for `D290`, whose sweep pattern — a `Vec<f64>`
of knots derived from another `KnotVector`'s knots by an affine map,
then `KnotVector::clamped` — has no remaining hit under `crates/*/src`
but six under `tests/`, each a fixture builder spelling the map by
hand with COMPUTED ends (`t0 + (t1 - t0) * k`, so the fixture's last
knot is `t0 + (t1 − t0)`, which is not `t1` in `f64` in general — an
ulp off either way — while the carrier the fixture is paired with is on
`[t0, t1]` exactly):

- `crates/geom-brep/tests/pcurve_general.rs` (the sphere chart-image
  builder, `.map(|k| t0 + (t1 - t0) * k)`)
- `crates/topo/tests/m6_3_chart_completion.rs` (same shape)
- `crates/topo/tests/review_ssiflat_r1_probes.rs` (same shape)
- `crates/topo/tests/review_ssiflat_r2_probes.rs` (same shape)
- `crates/sweep/tests/m8_4_intersection_iso.rs`, `fn rescaled` (the
  general `[a, b] → [lo, hi]` form, ends computed)
- `crates/geom-brep/tests/review_m5_pr7b_ssi.rs`, the `stretched`
  fixture in the rung-3 row (`.map(|k| k * 2.0)` then `clamped`) — the
  same class, spelled as a scale. Its purpose is a pcurve whose domain
  DISAGREES with the carrier's, so the door's spelling is
  `pb.on_domain(0.0, 2.0)` on the same net; the scale by two is exact
  at every knot, so nothing about that row rides on the ulp.

`crates/geom/tests/curves/compose.rs`'s `on_unit_interval` is the
inverse map used as a CHECK, not a builder, and is not in the class.

D290 landed the door these are hand-copies of: `KnotVector::on_domain`
and `NurbsCurve2/3::on_domain` (ends assigned exactly, interior affine,
count unchanged, typed refusal). The four curve fixtures are one call to
it each — `fit.on_domain(t0, t1).expect(..)` — which also removes the
`NurbsCurve2::new(..)` re-validation of a net that was already valid.
**`m8_4_intersection_iso.rs`'s `rescaled` is not**: it is applied to a
SURFACE's `knots_u`/`knots_v` (no surface-level door exists), and its
expression associates as `((hi − lo)·(x − a))/(b − a)`, which rounds
differently from the door's `(hi − lo)·((x − a)/(b − a))` in general, so
a converter there — `n.knots_u().on_domain(0.0, wide)` per axis, then
`NurbsSurface::new` — re-baselines the fixture's interior knots by an
ulp rather than restoring anything. Whether any row's outcome depends
on the ulp is not known: none of the six asserts its fixture's domain,
so a fixture an ulp short of the carrier's `t1` is not something these
suites can see today. Not D290's ground (its fence is the four source
files plus their tests), so filed here rather than converted.

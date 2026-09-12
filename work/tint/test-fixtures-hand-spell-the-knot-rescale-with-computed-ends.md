---
id: test-fixtures-hand-spell-the-knot-rescale-with-computed-ends
kind: issue
title: Five test fixtures hand-spell the knot rescale with computed ends instead of the on_domain door
status: open
opened: 2026-09-12
---

## Finding

Filed by the SCALAR lane for `D290`, whose sweep pattern — a `Vec<f64>`
of knots derived from another `KnotVector`'s knots by an affine map,
then `KnotVector::clamped` — has no remaining hit under `crates/*/src`
but five under `tests/`, each a fixture builder spelling the map by
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

`crates/geom/tests/curves/compose.rs`'s `on_unit_interval` is the
inverse map used as a CHECK, not a builder, and is not in the class.

D290 landed the door these are hand-copies of: `KnotVector::on_domain`
and `NurbsCurve2/3::on_domain` (ends assigned exactly, interior affine,
count unchanged, typed refusal). Each fixture is one call to it —
`fit.on_domain(t0, t1).expect(..)` — which also removes the
`NurbsCurve2::new(..)` re-validation of a net that was already valid.
Whether any row's outcome depends on the ulp is not known: none of the
five asserts its fixture's domain, so a fixture an ulp short of the
carrier's `t1` is not something these suites can see today. Not D290's
ground (its fence is the four source files plus their tests), so filed
here rather than converted.

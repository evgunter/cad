---
id: literal-k-where-the-runs-k-belongs
kind: issue
title: a literal 10 (and one DEFAULT_K) stands in for the run's K at five band thresholds
status: open
opened: 2026-09-04
refs: [1732]
---


## The finding

`Band::linear` scales the coincidence threshold by the run's K
(`Tol::k()`, `CAD_AMBIGUITY_K`, default `DEFAULT_K` = 10). Five band
constructions instead scale by a **literal 10**, and one by the compiled
`DEFAULT_K` constant. Each therefore states a band that coincides with
the run's only while K is at its default, in a repo whose whole point
here is that ε and K are run configuration.

Found while enumerating the 143 `Band::new` call sites for
`band-linear-spelling-not-swept`; that unit's hit list described these
sites accurately and then treated the description as the disposition,
which is the gap this item closes.

- `crates/sweep/tests/common/approx.rs:428` — `Band::new(eps, eps * 10.0)`
  over a parameter ε. Note it sits ~370 lines below this same file's
  `pub fn band()` (line 61), which IS the run's band, with no sentence
  saying why this one is not that.
- `crates/sweep/tests/sf2b_r1_probes.rs:329` — `Band::new(e, 10.0 * e)`
  over an ε ladder.
- `crates/geom-brep/src/ssi/certify.rs:930` — `Band::new(zero, 10.0 * zero)`.
- `crates/geom-brep/src/ssi/march.rs:995` — `Band::new(zero, 10.0 * zero)`.
- `crates/geom-brep/tests/pcurve_p1a_meter.rs:30` —
  `Band::new(ROW_EPS, 10.0 * ROW_EPS)`.
- `crates/geom-brep/tests/tcost_k1_budget_exit.rs:65` —
  `Band::new(eps, DEFAULT_K * eps)`: the same decision spelled with the
  named constant. Structurally forced (see below) and now documented at
  its site; listed here so the class is complete.

A seventh instance, `crates/geom-core/tests/band_tolerance.rs`, was
repaired rather than filed — it asserted `20*eps` definite and `3*eps`
in-band, correct only for 3 < K < 20, inside the very row that pins
`Band::linear` to (ε, K·ε).

## What the two `src/` sites are, precisely

Both are **lib tests** under `#[cfg(test)]`, not production paths, and
in both the multiplier is structurally inert: the assertions read only
`band.zero()` (`march.rs` compares `MarchTol::from_band(band).meters()`
against `band.zero()`; `certify.rs` checks ladder rungs against a floor
derived from `zero`), and neither block reads `escalate()` at all. The
literal 10 exists only to satisfy `Band::new`'s `zero < escalate`
invariant. So these two are **not** wrong answers under a non-default K
— they are a misleading spelling, which is a real but smaller defect
than their location in `src/` suggests.

## Why `tcost_k1_budget_exit.rs` cannot simply be rewritten

`Band::linear` takes only a `Tol` witness and derives ε from the run;
`from_zero_threshold` is private. There is therefore **no door** for
"an explicit ε with the run's K", which is what a suite stating its own
tolerance would need. That is the structural reason its inline form
stays, and it is worth asking whether that door should exist — it is
the same gap any suite pinning ε but wanting K would hit.

## The decision this needs

Per site: either the literal is genuinely arbitrary (as the two `src/`
lib tests appear to be), in which case it wants a sentence saying the
escalate edge is unused rather than a number that reads as K; or it
means "the run's K", in which case it should consult `Tol`. That is a
judgement per site, not a sweep — which is why this is filed rather
than swept.

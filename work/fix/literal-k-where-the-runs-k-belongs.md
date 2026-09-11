---
id: literal-k-where-the-runs-k-belongs
kind: issue
title: a literal 10 (and one DEFAULT_K) stands in for the run's K at five band thresholds
status: review
opened: 2026-09-04
refs: [1732]
branch: fix/literal-k-sweep
pr: 2346
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

## What landed

Per site, decided rather than swept, with the K-dependence executed
before it was claimed.

**Consults the run's K now** (the escalate edge is read downstream, and
the ε is the caller's, so the band wanted "this ε, the run's K"):

- `crates/sweep/tests/common/approx.rs` `reattach_certifies_at` —
  `Band::new(eps, Tol::witness().get().k * eps)`. The band reaches
  `EdgeCurve::certify`, whose `decide` classifies a residual in
  (ε, K·ε) as an escalation rather than a certification, so the width
  is part of the answer the door reports.
- `crates/sweep/tests/sf2b_r1_probes.rs` `r1p5` — `is_axial`'s verdict
  is `Err` exactly when a margin lands in the ambiguity band, which is
  what the row's `escalations == 0` asserts.

**Arbitrary, and now says so** (the escalate edge is never read; the
number existed only to satisfy `Band::new`'s `zero < escalate`, and a
literal 10 read as K):

- `crates/geom-brep/src/ssi/certify.rs` — `tube_ladder` reads
  `band.zero()` and nothing else.
- `crates/geom-brep/src/ssi/march.rs` — `MarchTol::from_band` is
  `Self(band.zero())`.

**Deliberately pinned on both edges, and now says so:**

- `crates/geom-brep/tests/pcurve_p1a_meter.rs` — the rows assert that a
  cone seam drifting `0.98 ε` escalates because the collapsed meter
  reads `d·sec α` = 1.1316 ε. Deriving the upper edge from the run's K
  reds two of the four rows at `CAD_AMBIGUITY_K=1.05`, a legal run
  (the only floor is K > 1): executed, not reasoned.
- `crates/geom-brep/tests/tcost_k1_budget_exit.rs` — unchanged; its
  site already carries the sentence, and `crates/geom-brep/tests/shared/tol.rs`'s
  census already names it.

**Two defects the shape sweep turned up, both repaired here:**

- `crates/editor-core/tests/dsc_checks.rs` — the in-band slab was
  `10ε` thick, so `V/A = 5ε` and the escalation it asserts happens only
  for K > 5. It was RED on the merge base at `CAD_AMBIGUITY_K=3` and at
  `1.05`. Now `(1 + K)·ε`, in band for every K > 1.
- `crates/sweep/tests/bool3_torus_doors.rs` — the shell law was
  measured at the run's band and compared against a law computed from a
  literal `10·ε`, so it accused the discriminant metering of moving at
  `CAD_AMBIGUITY_K=100` when nothing had. Now `band().escalate()` —
  the measuring band's own upper edge. Same repair in
  `crates/sweep/tests/bool3_r1_probes.rs`, where the figure is printed
  rather than asserted.

The structural gap the item names — `Band::linear` takes only a `Tol`
and `from_zero_threshold` is private, so there is no door for "an
explicit ε with the run's K" — is untouched here: `crates/geom-core/src/`
is PROPS' ground. Four sites now want that door, not the one the item
named; reported to the orchestrator rather than filed across the fence.

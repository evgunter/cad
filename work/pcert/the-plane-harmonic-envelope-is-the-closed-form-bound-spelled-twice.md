---
id: the-plane-harmonic-envelope-is-the-closed-form-bound-spelled-twice
kind: issue
title: A plane chart's harmonic envelope is the stated closed-form bound spelled a second time, uncentred
status: open
opened: 2026-11-12
priority: P3
cost: C
---

Filed by `pcert/closed-form-image-bounded` (PR 6845), from its review's
fix list SS5.

## What

One bound has two homes. `pcurve_cache.rs` `run_harmonic_checks`, the
closed-form door's plane arm, bounds a harmonic image's miss from its
carrier as `‖Δc‖ + σ(Δa, Δb) + ‖Δl‖·max(|t₀|, |t₁|)`, the coefficient
differences of the image mapped through the affine chart against the
carrier's. `pcurve_cache/closed_bound.rs` `harmonic_residual` and
`Residual::bound` bound the same residual on every analytic chart,
centred on the span's midpoint (`Σ hʲ·sup|cₖⱼ cos ks + dₖⱼ sin ks|`).
On a plane the two are the same statement, the second tighter.

## Why not merged in PR 6845

The plane arm certifies every derived plane row at every door, the
symbolic scalar included. Routing it through `closed_bound` changes the
forms the M10 documents' walks build (the plate's ledger digests in
`editor-core/tests/m10_sym_profile_interval.rs`) and the envelope every
stored plane certificate carries (`pcurve_mirror_v`'s "the same
certificate to rounding"), so it is a re-pin of its own, not a drive-by.

## Fix direction

Make the plane arm of `run_harmonic_checks` read
`closed_bound::harmonic_residual(..)?.bound()` over the span's outer
ends (`carrier_net::outer_ends`), delete its own spelling, and re-pin
the symbolic ledgers with that reason. The periodic charts' arm
(fidelity against a re-derived image, plus frame and incidence) is a
different statement and stays. The projected door's fidelity term
(`projected.rs`, the stored carrier through the inverse chart's
Lipschitz bound) is a different statement too, not a third home.


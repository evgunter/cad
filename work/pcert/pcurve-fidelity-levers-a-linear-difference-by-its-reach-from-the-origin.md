---
id: pcurve-fidelity-levers-a-linear-difference-by-its-reach-from-the-origin
kind: issue
title: pcurve check 4: fidelity reads a linear-term difference as |Δl|·max(|t0|,|t1|) plus |Δc| separately, so a window far from t = 0 is hundreds of times loose
status: open
opened: 2026-10-14
priority: P3
cost: E
---


(PCERT implementer, from review 5041's R1 probe
`review_5041_a_definite_loose_envelope`, on `review/5041-r` @ 0d89aa292.)

## What

`crates/geom-brep/src/pcurve_cache.rs` `fidelity` reads each channel as
`|p0 − q0| + σ(pa − qa, pb − qb) + |pl − ql|·reach`, with `reach =
max(|t₀|, |t₁|)` (`periodic_envelope`'s `let reach`). The constant and
the linear difference are bounded separately, each about `t = 0`. But a
stored image whose difference is `Δl·(t − t_mid)`, a drift centred in
the window, has `Δc = −Δl·t_mid`, and the two cancel to at most
`|Δl|·w/2` over a window of width `w`. Read separately they sum to
`|Δl|·(|t_mid| + reach)`.

Measured (cylinder r 0.5, a rim, `l = 1e-8`, `w = 0.1`): the true miss
is `5e-10` (0.5ε), and the term reads about `2·10⁻⁶` at `t₀ = 100` and
`2·10⁻⁷` at `t₀ = 10`, 200–4000× loose. Both refuse definitely, as
`EnvelopeTerm(FidelityV)`. PR 5041's
`a_refused_envelope_reads_as_a_bound_at_rest` is built on exactly this
refusal (it now reads as a bound, not a stored contradiction), but the
certificate still refuses a stored image it should accept.

The same `|Δl|·reach` shape is in the plane chart's affine envelope
(`‖Δl‖·T` with `T = max(|t₀|, |t₁|)`, `EnvelopeStatement` docs) and in
the incidence `Line` term (`radial(l).norm() * reach`).

## Direction

Expand each channel's linear part about the window's midpoint: read
`Δc + Δl·t_mid` as the constant and `|Δl|·w/2` as the linear term.
Over the window, `Δc + Δl·t = (Δc + Δl·t_mid) + Δl·(t − t_mid)`, and
`|t − t_mid| ≤ w/2`, so this is exact and never looser. Then apply the
same to the plane chart's `‖Δl‖·T` and to the incidence `Line` term.
Row: R1's drift at `t₀ = 100` certifies.

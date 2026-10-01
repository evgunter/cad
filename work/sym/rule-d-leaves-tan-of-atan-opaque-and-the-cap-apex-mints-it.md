---
id: rule-d-leaves-tan-of-atan-opaque-and-the-cap-apex-mints-it
kind: issue
title: Rule D folds sin/cos of q·atan X but not tan, and the cap apex's tan(|Δθ|/4) mints that atom: 104 plate freezes on the coefficient bound
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [the-negative-arm-lost-its-document-consumer]
---

## What was measured (LINALG's merge of `main` into `props/sign-hull`, 2026-10-01)

The plate's walk ledger (`m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`)
on `props/sign-hull` before the merge froze 0 nodes on its
`Early/Assertion` and `Door/Decision` lines; rule G had taken them from
104 to 0. Merged with `main`, they freeze 104 each again. The plate's
receipt is `[811, 0, 140, 462]` either way, so no decision moves; what
moves is the walks' frozen column.

**Cause:** `main`'s `4a8cfe8da` ("sweep: the cap apex is chord-scale
(mid − n̂·σ·(len/2)·tan(|Δθ|/4)), read off the sweep"), in
`sweep::swept::arc_apex`. For the plate's unit-bulge circles the
sweep is `4·atan(1)`, so the sagitta carries `tan(1·atan(1))`. Rule D
(`geom_core::sym::trig::fold`, under `trig_of_atan`) folds `sin` and
`cos` of `q·atan X` in closed form; `Tan` goes to `atom1`, so the value
`1` stays an opaque atom. A profile probe listed all 208 freezes as a
`Powi` over a form in that atom, each refused on the COEFFICIENT bound
(coefficients at `2^186` scale), e.g.
`(…·tan(1·atan(1)) + …·2^62·tan(1·atan(1))·hole_a_r + …) / (…)`.

**Confirmed by a probe, then reverted.** I routed `SymOp::Tan` through
rule D's arm as `closed.sin / closed.cos` (declining when the `cos`
polynomial is zero). Both lines went back to frozen 0, and the plate's
receipt did not move.

## What a fix would be

Fold `tan(q·atan X)` as the quotient of rule D's own closed forms: the
shared denominator cancels, and at `q = 1` the fold is `X`. That is the
bulge-rational apex the sagitta had before `4a8cfe8da`, read through
the sweep. Open points, not measured: the pole (`cos kψ`'s polynomial
vanishing over a box where `tan` has no value); the `half-π` arm's
`k mod 4` table for `tan`; and the re-measure of every pinned receipt
and ledger (the apex is in every cap plane of every arc document).

## Home

SYM: rule D's arm. The apex spelling is PATHS's, and it is deliberate
(`arc_apex`'s doc: chord-scale, one tangent and not the half-sweep's
`sin`/`cos`, which cost `r1_annulus` its ceiling).

---
id: a-rotation-can-refuse-an-approx-face-that-certifies-near-eps
kind: issue
title: a rigid map moves an Approx face's hull_sup by up to 5%, so a face certified within that margin of eps refuses ApproxRecertify after a rotation
status: open
opened: 2026-09-26
priority: P3
cost: E
---


## Measured (ENCL tight-ε lane, PR 3272's sitting, 2026-09-26)

`topo::transform`'s `map_approx` re-derives a mapped face's offset
certificate from scratch. On `bowed()` at `d = ±0.05`, each fit was
re-derived under 92 rigid maps (4 axes × 23 angles, plus a translation)
with `certify_offset_at`:

| fit ε | stored `hull_sup` | worst mapped | drift | maps over ε |
|---|---|---|---|---|
| 1e-6 | 4.39e-7 | 4.61e-7 | +5.0% | 0 / 92 |
| 1e-9 | 9.26e-10 | 9.50e-10 | +2.6% | 0 / 92 |

The worst map in both cases is axis (1,1,1)/√3 at 1.08 rad. None refused
here, but the fit stops refining as soon as it certifies, so a face can
land anywhere under ε. One that lands within a few percent of ε would
refuse `ApproxRecertify` after a rotation: moving a valid body would make
it invalid. That is inferred from the drift sizes, not reproduced.

## What is open

Whether a rigid map should be able to refuse a face the kernel just
minted. Possible directions include minting with headroom (certifying
under ε by the certificate's worst rigid-motion drift) or re-fitting on
refusal, but the choice is the taker's. The first step is a fixture that
reproduces the refusal. The probe sources are not committed.

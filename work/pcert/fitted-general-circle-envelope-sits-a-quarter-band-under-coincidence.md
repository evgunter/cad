---
id: fitted-general-circle-envelope-sits-a-quarter-band-under-coincidence
kind: issue
title: "pcert: the fitted general-circle image is refined to a quarter of the band, so its certified envelope (pcurve_envelope_hermite) keeps only 4x of headroom against the band at every eps; confirm the target or widen it"
status: open
opened: 2026-10-02
priority: P3
cost: M
---



## Measured

Found by the `reach/tilted-sphere-pair` lane, once lily wall 7's carve
runs at the probe scalar. `pcurve_envelope` is in the committed M7 era
(640 rows per ε, all zero-classified at ~1e-15), and a fresh sweep
records it on 23 shapes. Only `demo/lily_walls` flags: 8 rows per ε,
the fitted general-circle rows (PR 3733's route) of the tilted section
on the lantern's zone sphere.

| ε | margins (m) | `|m|/ε` |
|---|---|---|
| 1e-6 | 1.9421e-7, 2.2900e-7 | 0.194, 0.229 |
| 1e-9 | 2.1237e-10, 2.4087e-10 | 0.212, 0.241 |
| 1e-12 | 2.4704e-13, 2.4980e-13 | 0.247, 0.250 |

The margin is ε-coupled by construction.
`pcurve_cache::sphere_circle_image_lane` refines the Hermite image
"until every span's certified bound is a quarter of the band"
(`hermite_image(…, 0.25 * band.zero())`). The certificate then
re-derives the envelope at just under ε/4 and classifies it `zero`.
k-lint rule (2) flags any zero-classified margin above `ε/10²`.

**What the lint does with it now** (PR 3817). `run_fitted_checks`
records a circle carrier's envelope under its own name,
`pcurve_envelope_hermite`, so the closed-form lanes' envelopes keep
`pcurve_envelope` and stay under the metre rules. `tools/k-lint` gained
rule (5), `CONSTRUCTION_COUPLED`. The rule judges a name's `zero` rows
against the target its construction refines to: here a quarter of the
band, with a 1.2× ceiling of 0.30·ε. It flags any definite row as a
refused certificate. `tools/k-lint/tests/construction_coupled.rs`
re-reads the target out of `sphere_circle_image_lane` (`0.25 *
band.zero()`), so the ruling goes red if the lane's target moves.

The rule records the design as built. It does not settle whether a
quarter band is the right target: the certificate has 4× of headroom
against the band. At the interval scalar at 1e-12, the same rows already
escalate their loop-continuity check
(`fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar`).

## What a fix owes

PCERT should confirm that a quarter band is the lane's intent, or choose
a target with more headroom. Quintic Hermite error falls as `h⁶`, so a
target of `ε/10²` costs about 1.7× the spans. If the target moves,
`CONSTRUCTION_COUPLED`'s entry and its pin move with it.

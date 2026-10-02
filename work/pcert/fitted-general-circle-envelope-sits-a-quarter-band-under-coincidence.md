---
id: fitted-general-circle-envelope-sits-a-quarter-band-under-coincidence
kind: issue
title: pcert: the fitted general-circle image is refined to a quarter of the band, so its certified pcurve_envelope sits within 4x of the coincidence threshold at every eps and k-lint rule 2 flags it
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

**Why the ε-coupled roster cannot take it.**
- `lint_sample` judges a `zero` row by rule (2) for EVERY family,
  rostered or not ("a zero classification is a decision against ε
  itself"). Putting the name on `EPS_COUPLED_PREDICATES` would therefore
  not touch these rows.
- The name's other sites are closed-form residuals at ~1e-15, which are
  ε-independent.
- The name records no definite rows, since a definite envelope is a
  refusal, so rule (4) has no draw to cut over.
- `EPS_COUPLED_UNRULED` gates on every row of the name, and every
  sweep has 640 of them.

So the flag is a true rule-(2) statement: the fitted lane leaves its
certificate 4× of headroom against the band. At the interval scalar at
1e-12, the same rows already escalate their loop-continuity check
(`fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar`).

## What a fix owes

One of the following:
- A fit target with the headroom the rule asks for. Quintic Hermite
  error falls as `h⁶`, so a target of `ε/10²` costs about 1.7× the
  spans.
- Or the reason a quarter band is the right target, recorded where
  k-lint reads it.

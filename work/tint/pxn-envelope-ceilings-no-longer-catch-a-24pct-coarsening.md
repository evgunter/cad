---
id: pxn-envelope-ceilings-no-longer-catch-a-24pct-coarsening
kind: issue
title: r1_pxn_probes: the envelope ceilings no longer catch a 24% coarsening of PXN_FIT_SAMPLES
status: open
opened: 2026-10-02
cost: E
priority: P2
---


`crates/geom-brep/tests/r1_pxn_probes.rs`
`the_certified_sup_bounds_the_dense_sampled_true_sup` keeps a ratio
ceiling of 1.005 and an additive one of `ENVELOPE_FLOOR = 1e-13`. Both
were chosen so that coarsening `PXN_FIT_SAMPLES` from 33 to 25 turned
the row red. The meet of the convex and lerp insertion forms
(`work/props/convex-insert-widens-near-equal-point-nets.md`) tightened
the envelope, and that coarsening now passes. Measured at default ε,
certified over sampled truth:

| samples | a = 0 | a = 1e-13 | a = 1e-12 | a = 1e-11 | a = 1e-10 |
|---|---|---|---|---|---|
| 33 | 3.553e-15 | 1.064x | 1.000581 | 0.999993 | 1.0000021 |
| 25 | 9.326e-15 | 1.251x | 1.002420 | 1.0000038 | 1.0000022 |
| 17 | 3.553e-15 | 1.354x | 1.004091 | 1.0000283 | 1.0000024 |
| 9 | 9.770e-15 | 1.959x | 1.013100 (red) | — | — |

The coordinator ruled that the ceilings stay in the PR that tightened
the envelope, so the row's docs now state what the ceilings catch:
nine samples, through the ratio arm only. The additive arm catches
none of the three coarsenings. Ceilings that would restore the guard
at 25 samples, from these numbers:

- a ratio ceiling of about 1.0015 (33 reads 1.000581, 25 reads
  1.002420);
- `ENVELOPE_FLOOR` at about 2e-14. Over truth, a = 1e-13 reads
  6.5e-15 at 33 samples and 2.51e-14 at 25, and a = 0 reads
  3.6e-15 and 9.3e-15.

Both margins are under 2x on the red side, so they need a check at
every ε row before they land.

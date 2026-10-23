---
id: the-teapot-spouts-frustum-oracle-misses-its-measured-volume-at-1e-12
kind: issue
title: At eps 1e-12 the teapot spout now measures a volume, and the straightened-frustum oracle sits outside its pad
status: open
opened: 2026-10-23
---


## What

`demos/tour/tests/eps_regression.rs` `tour_runs_green_at_eps_1e_12` is
red on PR 5635 (CI run 40026352236, `demos (tour + wild)`). It panics at
`demos/tour/src/teapot.rs` (the spout's `Ok(p)` arm of `stops`):

```text
the canal's V = 0.000055052648797962526 vs the straightened frustum's
0.000055052533158981754: they differ by 1.156389807722329e-10 m^3,
OUTSIDE the certified enclosure's own half-width 8.094490616656182e-12
```

**Not PR 5635's routing.** Reproduced locally at 1e-12. Reproduced
again with `constant_weight` mutated to answer only for `1`, which
restores every unit-weight gate PR 5635 moved, and with `skin.rs`
given a local constancy test. The panic was the same to the bit. PR
5635 only triggers the demos job, by touching
`demos/tour/src/skinned.rs`. Main's PR gates run that job only for
diffs under `demos/`, so the red was most likely already on main, and
PR 5629 (`skin_on`'s per-row lane, which changes how the spout's
constant-`w` control rows are interpolated) is the likeliest cause.
That attribution is unverified: no pre-5629 build was run.

**What changed in the reading.** The scene's docs say the spout's
measurement refuses `QuadratureBudget` at 1e-12 (the chase stops at
1.244e-9 m against a 1.024e-9 m target), so the scene takes the
`TargetUnreached` arm, whose bracket holds the frustum. Now the
measurement meets the target, and the `Ok` arm compares the frustum
against a pad of 8e-12. The scene's own comment says the frustum is the
closed form of the thing the cubic skin APPROXIMATES ("there is no
elementary volume for a cubic skin through seven annuli"). A relative
miss of 2·10⁻⁶ is then the skin's fitting error, not a soundness
failure, and the oracle asks the bracket to hold an approximation's
value. Owed: confirm that the measured enclosure is sound (for example
against a refined skin, or a dense quadrature of this skin), then
re-state the `Ok` arm's oracle and the `SPOUT_ARCS` docs' claim that
1e-12 refuses.

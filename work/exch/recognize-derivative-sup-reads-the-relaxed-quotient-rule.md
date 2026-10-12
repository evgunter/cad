---
id: recognize-derivative-sup-reads-the-relaxed-quotient-rule
kind: issue
title: step-import's derivative_sup bounds a rational patch's speed through the triangle-relaxed quotient rule over the whole net, where geom-brep's patch_bound reads the joint per-cell maximum
status: open
opened: 2026-10-12
---



## Found (ENCL `encl/one-chart-speed` §5 sweep, 2026-10-12)

`crates/step-import/src/recognize.rs` `derivative_sup`, read by
`enveloped_residual_sup`, bounds `|S_u|` and `|S_v|` as
`(sup|A_u| + sup|S − c|·sup|w_u|)/w_min` over the whole control net,
in `f64`. That is the triangle relaxation of the joint
`max_{i,j} ‖k_i − g_i·q_j‖/w_min`. Here `k_i` is the `i`-th
coefficient of `A_u` about `c`, `g_i` the matching coefficient of
`w_u`, and `q_j = P_j − c`. Each term is affine in `S − c`, so its
norm is largest at a control point. The joint maximum is never
larger, and it keeps the cancellation between `k_i` and `g_i·q_j`
that the relaxation drops.

ENCL put the surface meters and `ssi::enclose`'s chart speeds on the
joint reading
(`crates/geom-brep/src/patch_bound.rs` `QuotientForm::speed_sup`,
read through `pair_norm_sup`). On the meters' fixtures the relaxed
reading was up to `1.125`× the joint one. `derivative_sup` is not a
certified reading: its sums and norms are rounded to nearest, and its
`sup` folds use `f64::max`. So the certified form does not drop in
as it is, and whether the bound must be certified is this row's
question first.

## What is open

Decide whether `enveloped_residual_sup` needs a certified speed. If
it does, read `derivative_sup` through the certified form, which also
answers the rounding. If it does not, take the joint maximum in
`f64`. The whole-net reading, with no cells, is a separate looseness
from either.

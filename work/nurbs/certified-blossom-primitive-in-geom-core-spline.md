---
id: certified-blossom-primitive-in-geom-core-spline
kind: issue
title: geom_core::spline: host one certified knot-insertion / blossom primitive; the ssi cut box is a third interval de Boor ring in a consumer
status: open
opened: 2026-10-02
priority: P3
cost: M
---


Found in review of the `ssi/dome-tube` lane (PR 3903), 2026-10-02.

## What

The tree has three interval-arithmetic de Boor / Boehm recurrences:

- `CurvePlan::apply_certified` in `crates/geom-core/src/spline/algebra.rs`;
- `insert_once_ring` in `crates/geom-core/src/spline/compose.rs`, which
  that file already files as a duplication of the first;
- `bezier_on` in `crates/geom-brep/src/ssi/enclose.rs`. It blossoms a
  span's homogeneous coefficient line to a sub-interval's Bézier block,
  written in the lerp form `d₀ + α·(d₁ − d₀)` on 4-channel lines, so
  that `NurbsBoxes::deriv_box` can cut a span cell to a tube window.

The third lives in a consumer because the spline module offers no
primitive that restricts a span's piece to `[a, b] ⊂ [t_s, t_{s+1}]`
in certification arithmetic. Each copy carries its own index ranges
and its own rounding argument.

## Next

Host one certified primitive in `geom_core::spline`, covering:
- restricting a coefficient line to a sub-interval of a span (blossom
  or double knot insertion to full multiplicity);
- generic over the channel count;
- in the lerp form, so that equal point coefficients stay exact.

Then route `bezier_on` through it, and the other two where their
schedules fit. `TensorNet::refine_u`/`refine_v` are the tensor
bookkeeping a surface consumer would reuse.

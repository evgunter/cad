---
id: certified-blossom-primitive-in-geom-core-spline
kind: issue
title: geom_core::spline: host one certified knot-insertion / blossom primitive; the ssi cut box is a third interval de Boor ring in a consumer
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-10
branch: nurbs/monotone-step-and-blossom
pr: 4551
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

## Closed

`CoeffWindow::blossom(args)` is the one certified primitive
(`geom_core::spline::blossom`): `p` enclosure arguments, each met with
the span's closure (an argument wholly outside refuses typed,
`BlossomError::OutsideSpan`), every level through
`Certification::between`. `CoeffWindow::restrict((a, b))` is the
Bernstein row on `[a, b]`; a bare Bernstein row mints
`KnotVector::bezier(p, (s0, s1))`. `compose`'s Bézier decomposition is
`restrict` per span and piece, and `insert_once_ring` and `sub_segment`
are retired. Routed through it: `ssi/enclose::bezier_on`,
`ssi/section::sub_piece`, `props/quad`'s `de_boor` (inside the span's
closure) and `bezier_bisect`, and `pcurve_cache/projected::piece_controls`
through `SpanLocate::restrict_window`. The per-site table, including
the sites not routed and why, is in PR 4551's body.
`apply_certified` and `TensorNet::refine_*` keep their Boehm schedules
and step through the door.

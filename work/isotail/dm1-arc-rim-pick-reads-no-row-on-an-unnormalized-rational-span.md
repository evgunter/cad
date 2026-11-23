---
id: dm1-arc-rim-pick-reads-no-row-on-an-unnormalized-rational-span
kind: issue
title: the arc-rim pick holds no row of dm1's l-bracket wall, inferred to be its unnormalized end weights 1 and 2
status: closed
closed: 2026-11-04
opened: 2026-10-29
priority: P3
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot]
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

## Measured

Since the import splits a foreign chart at each interior C⁰ knot, `tests/fixtures/wild/stepcode/dm1-id-214.stp` refuses at every band with:

```text
pcurve certification: the iso-line lane refuses this class — no boundary row of this chart, in
either u direction, holds the circle at the probe — not an arc rim of this chart
```

The refusal comes from `topo::pcurves::nurbs_iso_derive`'s arc-rim arm, through `geom_brep::pick_arc_rim`. It sits at the same door that refused `MapResidual` before the split: the arc-rim mint on the l-bracket wall `#382`.

Wall `#382` states its circle as four rational quadratic spans whose control polygon is a square, with weights `(1, 1, 2, 1, 1, 1, 2, 1, 1)` along `u`, so each span's end weights are 1 and 2. That is a circle under a non-standard (unnormalized) parameterization. Its rim circles are three-arc rationals, promoted to `CIRCLE`. After the split each sub-patch is one such span.

A rational half cone with normalized weights, split the same way, adopts its CIRCLE rims on the sub-patches through the same pick (`a_rational_wall_splits_where_its_rims_carry_no_knot`).

## Inferred, not proven

`arc_rim_measured` reads the row's turn from its interior weight as `w = cos(h/2)` (`arc_rim_turn`), which holds for end weights of 1. A span with end weights 1 and 2 states the same arc with interior weight `1·√(1·2)·cos(h/2)` after normalization, so the turn read off it is wrong and no candidate holds the circle at the probe. Normalizing a row's end weights before the read (a weight reparameterization, which keeps the locus) is the likely repair. Confirm the cause before building it.

## Closed (2026-11-04)

The PCERT main-reds lane (`pcert/main-reds-e2-e5`) confirmed the cause
and fixed it at the pick and at the image. Measured on dm1's own bits:
the sub-patch's row at `v₁` has weights `(1, 1, 2)` on
`u ∈ [0, 0.27027…]`, and it holds the quarter rim exactly (zero radial
residual in exact rationals over 101 samples). At `u = ½` it sits at
53.13° where the standard form sits at 45°, so `arc_rim_turn` read
`w₁/w₀ = 1`, no turn, and the pick answered `Ok(None)`.

The repair keeps the file's parameterization rather than normalizing the
chart. `Pcurve::IsoArc` carries the row's end weights (`ends`), and its
map applies each sub-arc's Möbius change `σ = λs/(λs + 1 − s)`,
`λ = √(eₖ/eₖ₊₁)`. The certificate compares `ends` to the row's on the
bits and decides `wₖ/√(eₖ·eₖ₊₁) = cos(h/2)` per span, and the pick reads
the turn at unit end weights. The l-bracket now imports, and full dm1
refuses one door later
(`work/importgate/dm1s-touching-instances-refuse-at-the-aggregate-gate-unrecorded.md`).


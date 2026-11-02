---
id: dm1-arc-rim-pick-reads-no-row-on-an-unnormalized-rational-span
kind: issue
title: the arc-rim pick holds no row of dm1's l-bracket wall, inferred to be its unnormalized end weights 1 and 2
status: open
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

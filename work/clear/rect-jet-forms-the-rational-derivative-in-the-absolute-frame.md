---
id: rect-jet-forms-the-rational-derivative-in-the-absolute-frame
kind: issue
title: rect_jet forms a rational net's derivative from separately hulled position and slope in world coordinates, so its width grows with the body's distance from the origin and never shrinks with the cell: rational walls never certify against themselves
status: dispatched
opened: 2026-11-01
priority: P0
cost: M
---


## Finding (the PRICE designers, 2026-11-01, three independent reports)

`NurbsSurface::rect_jet` / `restricted_jet` (`crates/geom/src/surfaces/nurbs.rs`)
forms the rational derivative `S_u = (A_u·w − A·w_u)/w²` on the cell's
hulled homogeneous net. `A` is the hull of absolute homogeneous
positions, so `A·w_u` carries the face's distance from the origin times
the cell's weight slope. The excess width does not shrink with the cell.

- **Measured on main**, the `S_u` hull against `S_u`'s true range over a
  cell: lily walls are 2.8–5.4× wide at the median and 56–130× at the
  tip; Klein walls are 13–29×.
- **Position dependence.** A quarter-circle wall of radius 0.25 placed at
  (3, 3) has a jet 7.7× wider than the same wall at the origin. Moving the
  lily wall by (10, 10, 10) drops its certified cells at 8×32 from 167 to
  0.
- **Polynomial walls** (`w_u = 0`) pay nothing, which is why the arc-spine
  lofts, `s_duct` and `twisted_loft` hold.

**Prototyped by two designers, not committed:** recentre each homogeneous
control point, `A_i − c·w_i`, before taking the hull. Recentring the hulls
afterwards changes nothing.
- Worst slack falls from 109× to 3.7×.
- The lily walls against themselves hold in 0.3–1.3 s, where before they
  ran out of budget after 46 s.
- The Klein walls against themselves hold in 4.6–6.1 s.
- Whole bodies: `lily_leaf_a` takes 12k pairs, and the Klein loop 80k pairs
  in 43 s. What remains is cap-corner Departure (ZEROSIDE) and the
  near-parallel walls.

One designer argues the home is the convex-combination form already in
`geom_core::spline::derivative` (the numerator over the weight squared,
bounded by control ratios), which would also leave residual slack below
3.7×. Then the monotone arm could test the numerator's control matrices
(the least eigenvalue is concave).

**The same class (width grows with translation), already filed:**
- `nurbs/nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` (P2);
- `quad/quad-rational-patch-cross-grows-with-translation` (P2);
- `encl/patch-bound-offset-fit-recentring-origins` (P1).

One home for a rational derivative over a cell, which cannot depend on
where the body sits, closes the class.

Blocks `injectivity-price-on-tour-lofts-exceeds-the-pair-budget`.

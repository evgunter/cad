---
id: a-uniformly-scaled-rational-patch-refuses-the-quadrature-as-a-straddling-weight
kind: issue
title: A rational patch whose weights all sit near one end of the f64 range refuses the quadrature as a weight hull straddling zero
status: open
opened: 2026-10-12
priority: P3
cost: E
---


Found by ENCL (branch `encl/subnormal-weight-face`) while sweeping for
the shape that row fixed in `patch_bound::rational_cells` and
`offset_fit`'s `Composite::build`: an enclosure that forms powers of a
surface's weights, on a face whose weights all sit near one end of the
`f64` range.

Measured with `props::quad::nurbs_patch_face` on the two-span half
cylinder `review_r1_rational_probes::diag_uniform_weight_twins` builds,
weights `1, 1, s, s, …` (`s = cos π/4`) times one power of two:

- `2⁰` and `2⁻¹⁰⁰`: `QuadratureBudget` (width 1.30e-6 against target
  1.02e-6), the same answer at both scales.
- `2⁻³⁰⁰` and `2³⁰⁰`: `QuadratureUnsupported` "a rational patch
  enclosure refused (a weight hull straddling zero, or a non-finite
  net)", the refusal at the end of the rational lane in
  `props::quad` (near `nurbs_patch_face`).

The surface is the same at every scale, so the refusal names a lever
the caller does not have: no weight straddles zero, and the net is
finite. The `w³`/`w⁴` the rational integrand divides by
(`n_dd / w.powi(3) − …/w.powi(4)`, and the per-column `w³`) underflow
or overflow at those scales.

The remedy is `geom_core::spline::weight_scale::unit_scale_exact`:
apply it to the weight slice before the lane forms its enclosures.
It returns the weights scaled by the one power of two that carries the
largest into `[1, 2)`, or `None` when that would drop a bit (read the
weights as described). The scaled net is the same described surface,
so this is a fix rather than a recourse. `patch_bound::rational_cells`,
`offset_fit`'s `Composite::build` and `ssi::section`'s
`Pieces::enclosed` already read their weights through it. Decide which
arm reads the net on the described weights, before the rescale: a net
of weights all one power of two becomes all ones.

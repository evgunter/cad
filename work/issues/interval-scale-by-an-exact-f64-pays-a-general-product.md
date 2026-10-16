---
id: interval-scale-by-an-exact-f64-pays-a-general-product
kind: issue
title: An interval times an exact power of two pays a general interval product: the quotient halving's one scale per output
status: closed
branch: nurbs/interval-scale-exact
closed: 2026-10-14
opened: 2026-10-14
---


## Measured

Since `nurbs/quotient-products-once`, every quotient piece
(`geom_core::spline::derivative`: the cell meters `cell_quotients`,
`cell_second_quotients`, `polynomial_area_pieces`, and the curve
meters `speed_pieces`, `accel_pieces`) is halved from products formed
once: de Casteljau's sums unscaled, then each output scaled once by an
exact `2⁻ʳ` (`Grid::halved`, `Vector::scaled`). At `Interval` that
scale is `self * T::from_f64(k)`: a general interval product (four
products, outward rounding) against a point, where the product by a
positive power of two is exact in the normal range and needs no
rounding at all (`[lo·k, hi·k]`, a subnormal result aside).

Callgrind on `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian`
on that branch: the halving is about 49% of instructions, the piece
readers (`QuotientPiece::coordinate_range`, `sup_norm`, the divides)
30–40%. The scale is part of the halving's half.

## Lever

A scale-by-exact-`f64` door at `Real` (`interval.rs` and
`interval-transcendentals` have no program's territory today, so it
needs an owner), reachable without a new `Real` bound
(`scripts/gates/no-extra-real-bounds.sh`). Estimate: at most 25% off
the face bound (about 1.3× there), less on the SSI walk, where the
pieces are a smaller share. Fewer pieces (`RATIONAL_CERT_SPLITS`, 16)
is the other lever, and it is a measurement before it is a change:
the count moves cell granularity, the census and the offset goldens.

## Resolution (NURBS build A, 2026-10-14)

The door is `Real::scale_pow2(k)`. It is a defaulted method on `Real`
itself, so it needs no extra bound: the default is `self * from_f64(k)`,
and only `Interval` overrides it, through `DInterval::scale_pow2` in
`interval-transcendentals`. The override:

- returns `[lo·k, hi·k]` unpadded when `k` is a positive normal power
  of two and both results are normal (or zero);
- falls back to the general product otherwise.

Against the general product it is:

- bit-identical wherever that product is exact (`|x·k| >= 2^-960`, the
  backend's 2Prod floor);
- strictly tighter between `2^-1022` and `2^-960`, where the general
  product pads an exact product one step;
- the general product itself everywhere else.

`Vector::scaled`, the halving's one scale per output, reads it.

Measured on `nurbs_cert::tests::rational_face_bound_dominates_sampled_hessian`
(callgrind, opt-level 1), against main `8096a51b`:

| | main | branch |
|---|---|---|
| test total | 3.757 G | 3.260 G (−13%) |
| halving (`Raised::halved`, inclusive) | 1.835 G | 1.338 G (−27%) |

`r1_lane0_e2e::the_f64_seam_answers_every_public_door` went from 73–78 s
to 71 s (`--test-threads 1`, two runs each).

The other lever named above, fewer pieces (`RATIONAL_CERT_SPLITS`), is
not taken here.

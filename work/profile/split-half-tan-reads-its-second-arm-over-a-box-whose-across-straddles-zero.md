---
id: split-half-tan-reads-its-second-arm-over-a-box-whose-across-straddles-zero
kind: issue
title: split_half_tan's second arm divides by across over the whole box, unbounded where a wide box of along lets across reach zero
status: open
opened: 2026-11-24
---


Found by `boxfinish/wall-roots`'s sweep (the class of
`geom_brep::quartic::quadratic_roots`: a value selection whose arm is
bounded only on its own side of the selector, evaluated over the whole
box at `Interval`). Read from the code, not run.

`profile::sugar::split_half_tan`: `(−along).select_le_zero(
split_half_tan_near(..), (len − along)/across)`. The second arm is the
accurate one where `along < 0`; where `along` straddles zero it is read
over the whole box, and `across` is an independent input. For a narrow
`along` about zero `across ≈ len` is clear of zero and nothing is lost;
a box of `along` wide enough (an arc's turn of order a quarter or more)
lets `across` contain zero, and the hull is the whole line where the
half-tangent is bounded. Holding `along` to its side does not help,
since `across` is not computed from it. Not established: that any
caller passes a box that wide.

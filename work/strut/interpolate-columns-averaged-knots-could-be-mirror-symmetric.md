---
id: interpolate-columns-averaged-knots-could-be-mirror-symmetric
kind: issue
title: interpolate_columns' averaged knots could be minted mirror-symmetric when the parameters are
status: open
opened: 2026-09-15
priority: P1
cost: D
---


## Finding

`crates/sweep/src/skin.rs` builds a lofted surface's v-direction
structure by handing its section parameters to
`geom::curves::fit::interpolate_columns`, whose averaged-knot
construction (`averaged_knots` in `crates/geom/src/curves/fit.rs`,
Book Eq. 9.8) computes each interior knot as
`(Σ params[j..j+degree]) / degree` — a left-to-right sum divided once.

When the parameter vector IS mirror-symmetric about `[0, 1]`, the
knot vector it produces need not be, because that sum is not evaluated
symmetrically: `params[j] + … + params[j+p-1]` and the mirror window's
sum accumulate different roundings. Averaging from BOTH ends — pairing
each window with its mirror and combining them so the two sides are the
same arithmetic — would mint a mirror-symmetric knot vector by
construction whenever the parameters are, at no cost to any other case.

## Evidence

`NurbsSurface::reversed_v` / `reversed_u` (SCALAR's VREV door,
`crates/geom/src/surfaces/nurbs.rs`) reverse a chart exactly when the
reversed direction's knot vector is its own reflection. On the kernel's
own producer — `k` equally spaced sections through
`sweep::loft_body` — that holds for `k ≤ 6` and FAILS at `k = 7` and
`k = 8`, although the chord parameters are symmetric in every one of
those cases:

- `k = 7`: `knots_v = [0, 0, 0, 0.25, 0.41666666666666663,
  0.5833333333333333, 0.75, 1, 1, 1]` — the two interior middles do not
  sum to 1 exactly (the door names indices 4 and 5).
- `k = 8`: `knots_v = [0, 0, 0, 0.21428571428571427,
  0.3571428571428571, 0.5, 0.6428571428571428, 0.7857142857142857, 1,
  1, 1]` — the outermost interior pair misses by one residual.

So a user who lofts seven equally spaced sections and asks for the
reversed wall gets a typed refusal from a door that is right to refuse:
the structure it was handed is asymmetric. The refusal is the door
reporting the producer's rounding, and fixing it upstream is what makes
the door say yes.

`crates/sweep/tests/vrev_acceptance_set.rs`'s
`a_lofted_walls_v_knots_are_symmetric_up_to_six_sections` pins the cut
as it stands; a lane that mints symmetric knots here moves that row's
boundary and should say so rather than restore it.

## Scope note

The construction is `geom`'s (PROPS' `crates/geom/src/curves/fit.rs`)
and the parameter choice is `skin.rs`'s (this program's). Whichever end
takes it, the claim to establish is the same: symmetric parameters in,
symmetric knots out, exactly.

## Was

Filed by SCALAR's VREV fix pass (PR 2627), which met the `k = 7`
refusal while writing the door's acceptance set down.

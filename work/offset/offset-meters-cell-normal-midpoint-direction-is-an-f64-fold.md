---
id: offset-meters-cell-normal-midpoint-direction-is-an-f64-fold
kind: issue
title: offset_meters: cell_normal's assembly-B direction norm is an f64 fold used as an upper-bound divisor
status: open
opened: 2026-09-15
priority: P1
cost: H
---


## The finding

`cell_normal`'s assembly B projects the normal enclosure `m` onto the
midpoint direction `d̂` and divides by a certified upper bound on
`‖d̂‖`, which is what makes the projection a bound at all — the
comment beside it says so: *"the division by a certified upper bound
on `‖d̂‖` is what keeps the projection a bound when `d̂` is unit only
to rounding"*.

That bound is assembled in raw `f64`:

```rust
let dn = sqrt_up(dv[0].mul_add(dv[0], dv[1].mul_add(dv[1], dv[2] * dv[2])));
```

Two `mul_add`s and one multiply, each rounded to NEAREST, then one
`next_up` from `sqrt_up`. `next_up` covers one ulp of the square root;
it does not cover the up to ~1.5 ulp the fold below it can lose
downward. Where the fold lands low, `dn` is not an upper bound on
`‖d̂‖`, the quotient `(proj / dn).lo()` is not a lower bound on the
projection, and `floor` — the regularity floor the whole offset door
rests on — is over-reported on that cell.

The same file already owns the sound spelling: `norm_sup`, which
squares and sums IN THE RING and then rounds the square root up. It is
twenty lines above and assembly C's own `sup` already calls it. The
fix is to give `dv` a ring reading, or to take the midpoint direction's
norm through `norm_sup` on point intervals.

## How large

Measured on the sibling site this was found from — `offset_fit`'s
`m_sup`, the same shape on the same kind of hulls — the `f64` fold sat
below the ring reading on **306 of 308 cells** of a real grid, worst
deficit 4.70e-16 relative (`d = 1e-6`, quarter cylinder; the
in-module row `the_normal_divisor_is_the_rings_reading_not_an_f64_fold`
still counts it). Assembly B's operands are midpoints rather than
magnitudes, so the count will differ, but the mechanism is identical.

## Why it is filed rather than fixed

Found by PROPS's mignitude-floor sweep (PR #2469), which fixed the two
instances inside `crates/geom-brep/src/offset_fit.rs` — PROPS's ground.
`offset_meters.rs` is SHELL's (`work.py territory`). The sweep's other
finding, the same shape with no `sqrt_up` at all, is filed on TRIM's
slate as `ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up`.

That PR does touch this file, by announced seam and for one thing
only: `norm_sq` and `norm_sup` become `pub(crate)` so `offset_fit` can
call them instead of re-spelling the fold. No behaviour in this file
moves.

## The class

*An upper bound on a norm assembled by an `f64` fold of ring
endpoints, used as a divisor.* The three members found are this one,
`offset_fit::Composite::cell_bound`'s `m_sq`/`y_sq` (fixed at #2469)
and `ssi::certify`'s `stretch`.

## Still live after RATE-PAIR (2026-09-15)

Re-read at `37dce8287`, after RATE-PAIR typed this file's chart speeds
(`PatchRegularity::speed_u`/`speed_v` are `SupSpeed<f64>` now) and
re-worded `CellNormal`'s unit docs. Neither touched assembly B: `dn`
is still `sqrt_up(dv[0].mul_add(dv[0], dv[1].mul_add(dv[1], dv[2] *
dv[2])))`, and the finding above stands verbatim.

Worth saying because the rate pair is the natural place a reader would
look for this: `dn` is NOT a rate. It is the norm of a direction
vector, so no `SupSpeed` tag applies and the pair's doors do not reach
it. What it wants is the ring's fold, which this file already owns.

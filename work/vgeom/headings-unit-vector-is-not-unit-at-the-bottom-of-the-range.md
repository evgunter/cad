---
id: headings-unit-vector-is-not-unit-at-the-bottom-of-the-range
kind: issue
title: heading promises a unit vector and a subnormal separation answers one of length 1.414
status: dispatched
opened: 2026-09-21
priority: P4
cost: E
branch: vgeom/deletions
---


## Finding

Found by the review of `vgeom/sketch-infinity`, against a claim that
PR made rather than against code it wrote.

`crates/viewer/src/sketch.rs`, `heading`. Its header says **a unit
vector**, or `None`. The guard is
`length.is_finite() && length > 0.0` over `length = dx.hypot(dy)`,
and that bounds each quotient to `[-1, 1]` — no component exceeds a
finite length — which is all the guard was ever asked for and all its
comment claims.

It does not make the answer unit LENGTH. Executed:

| `dx = dy` | `length` | answer | `\|answer\|` |
|---|---|---|---|
| `5e-324` | `5e-324` | `[1.0, 1.0]` | `1.4142` |
| `1e-320` | `1.414e-320` | `[0.70719…, 0.70719…]` | `1.0001287` |
| `1e-310` | `1.4142135623731e-310` | `[0.70710678118655…, …]` | `1.0000000000000084` |
| `1e-300` | — | — | `1.0` |

At the bottom of the subnormal range the division has no precision
left to divide with, so the quotient is not the direction cosine the
header names. The error is `1.29e-4` at `1e-320` and 41% at the
minimum subnormal.

## Reachability: a negative result about a SEARCH

Four separations were driven through the public `preview` door and
the driver refuses the degenerate junction two steps earlier every
time, at `Tol::witness()` and at `1e-12`. **A document tolerance
below `1e-12` was not tried.** So nothing here says the door cannot
be reached; it says the shapes that were tried do not reach it.

That is also why this is P4 rather than a guard: the recourse
`heading` has is `None`, and refusing a separation the driver already
refuses would be a guard whose only witness is a call no caller
makes. What a fix has to decide first is whether the header should
promise a unit vector at all, or promise the same thing
`camera.rs`'s ray direction does — where unit-ness is argued from the
CALLER rather than from the guard, which is the honest form and is
already in this crate.

## Fence

`crates/viewer/src/sketch.rs` — VGEOM's, under the standing double
claim with CHROME, VIEW and author.

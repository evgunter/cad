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

## Closed

Taken by `vgeom/deletions`. **The header stops promising a unit
vector and says what the guard buys instead.** The row's own question
— promise unit-ness, or argue it the way `camera.rs` does — is
answered against the precedent rather than around it: `ray_through`
keeps its promise because it can ARGUE it (*"`forward` is a unit
vector and the offsets are perpendicular to it, so the length is at
least 1 for every finite cursor"*), and `heading` is `pub` in a `pub
mod` with `points` supplied by the caller, so no such argument exists
to make. What is left is to state the property as a function of the
input.

### The table, re-derived by executing it

| `dx = dy` | `length` | answer | `\|answer\|` |
|---|---|---|---|
| `5e-324` | `5e-324` | `[1.0, 1.0]` | `1.4142135623730951` |
| `1e-322` | `1.4e-322` | `[0.7142857142857143, …]` | `1.0101525445522108` |
| `1e-320` | `1.414e-320` | `[0.7071977638015374, …]` | `1.0001286688480588` |
| `1e-315` | `1.41421356e-315` | `[0.7071067817978808, …]` | `1.0000000008645558` |
| `1e-310` | `1.4142135623731e-310` | `[0.7071067811865536, …]` | `1.0000000000000084` |
| `1e-308` | `1.414213562373095e-308` | `[0.7071067811865475, …]` | `0.9999999999999999` |
| `1e-300` | `1.414213562373095e-300` | `[0.7071067811865476, …]` | `1.0` |
| `1.0` | `1.4142135623730951` | `[0.7071067811865475, …]` | `0.9999999999999999` |

The row's figures reproduce. Two things it did not say, and both
shape the header:

- **The largest `dx = dy` with a non-unit answer is `1e-281`**, at an
  error of `1.1e-16` — one rounding step. So the promise was never
  EXACT even for ordinary separations; what the subnormal range does
  is take a 1-ULP approximation to 41%, which is a different claim
  and is the one worth writing down.
- **The DIRECTION survives where the length does not.** Both
  components are divided by one length and that length's own
  rounding is a common factor. Over every separation whose two
  components are the first 400 multiples of `5e-324`, the worst angle
  error is `2.2204e-16` rad — one rounding step — while the worst
  length error is `0.41421`. That is why the header now states the
  defect rather than refusing: `None` is this door's only other
  answer, and the one consumer
  (`pane/viewport.rs`'s tip marks) scales a screen mark by the pair,
  so a subnormal separation costs a mark up to 41% long and pointing
  the right way, against no mark at all.

### Reachability: a tolerance far below `1e-12` was tried

The row says a document tolerance below `1e-12` was not tried. It was
now, through `Tolerance::init(Tolerance::with_eps(1e-300))` — **three
hundred orders of magnitude, not twelve** — and the answer does not
move. Driven through the public `preview` door with that ε committed,
across four shape families:

- a junction at a tiny vertex, `d` from `5e-324` to `1e-200`:
  **refused**, every one, *"this junction is tangent at any precision
  you could care about (turn margin -0 m on a 0 m arm)"*. `d = 1e-100`
  draws and answers a heading of length exactly `1.0`.
- an arc of subnormal RADIUS (`1e-320`, `1e-310`, `1e-300`), whose
  flattened points would sit subnormally apart: **refused**, same
  rule.
- a collinear continuation of `5e-324` and `1e-320` off a 1 m leg:
  **refused**, same rule, *"turn margin 0 m on a 1 m arm"*.
- an OPEN chain whose last leg is the tiny one: draws down to
  `1e-100` and refuses at `1e-200` and below. Note why the ones that
  draw are unit: at `1.0 + 1e-100` the offset is swallowed by the
  coordinate, so `dx` is exactly `0` and the separation is normal.

**The refusal is not ε-driven**, which is the finding rather than the
count: the turn margin underflows to `±0` and the tangent rule fires
at any ε, so the diagnostic's own advice — *"or lower the
tolerance"* — does not open this door either. Lowering ε by 288
orders of magnitude changed nothing.

**What that still is: a negative about a search.** The shapes were
authored through `preview`; `heading` is `pub` in a `pub mod`, so the
population of callers is not the population of gestures, and a
downstream consumer of this crate can hand it any `&[[f64; 2]]`. That
is exactly why the deliverable is the header and not a guard.

**Where**: `crates/viewer/src/sketch.rs`, `heading` — the summary
line, a new paragraph on what the guard does and does not buy, and
the in-body comment that claimed the guard made the answer unit.

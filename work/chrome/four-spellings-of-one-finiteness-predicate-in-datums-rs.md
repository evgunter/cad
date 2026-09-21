---
id: four-spellings-of-one-finiteness-predicate-in-datums-rs
kind: issue
title: datums.rs hand-spells one finiteness predicate four times, past a named home
status: open
opened: 2026-09-15
priority: P1
cost: E
---


## Finding

`crates/viewer/src/datums.rs` asks "is this a positive finite number"
four times, in four spellings, none of them sharing a home:

| site | spelling |
| --- | --- |
| `View::metres_per_pixel_at` | `scale.is_finite() && scale > 0.0` |
| `View::screen_metres_at` | `span.is_finite() && span > 0.0` |
| `grid_pitch` | `if !wanted.is_finite() \|\| wanted <= 0.0` |
| `rule_patch`'s `rule` closure | `![first, last, lo, hi].iter().all(\|b\| b.is_finite())` |

The first two are the same question on the same quantity one call
apart; the third is its negation, spelled the other way round; the
fourth is the same question over four values with the positivity half
dropped because it does not apply to a coordinate.

**The sweep that closed `metres-per-pixel-swallows-a-nan-depth` added
the third and fourth of them**, which is what turns a pre-existing
duplication into something worth filing: the next lane to touch this
file will add a fifth unless there is a door to route through.

## The named home, and why it may not fit

`crates/geom-core/src/real.rs` has `is_finite_length`, and
`topo::query` routes through it — five doors listed in its own
rustdoc. Reading it, the fit is **not obvious and that is part of the
work**: `is_finite_length` is generic over `Real`, asks its question
through the scalar's poison channel (`x - x`), and its contract is
about a DECIDED length paired with a norm witness. `datums.rs` is a
display module over plain `f64` whose quantities are a scale, a span
and two lattice bounds — not norms, and never decided. So the
candidate outcomes are: route through `is_finite_length` after
checking it means what these sites need; give `datums.rs` one private
predicate of its own and use it four times; or argue that two of the
four are different questions and say which.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the
territories table; reading `crates/geom-core/src/real.rs` is a read,
not an edit.

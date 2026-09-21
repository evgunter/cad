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

**Measured against the tree on 2026-09-21 the census is EIGHT sites,
not four** — `grep -n is_finite crates/viewer/src/datums.rs` returns
eight lines and ten occurrences. The row's own prediction, that the
next lane to touch this file would add spellings unless a door
existed, is what happened. The title's number is kept as the count
the row was FILED at.

`crates/viewer/src/datums.rs` asks "is this a positive finite number"
three times, in three spellings, and "are these numbers" four more,
in three more, none of them sharing a home:

| site | spelling |
| --- | --- |
| `View::metres_per_pixel_at` | `scale.is_finite() && scale > 0.0` |
| `View::screen_metres_at` | `span.is_finite() && span > 0.0` |
| `grid_pitch` | `if !wanted.is_finite() \|\| wanted <= 0.0` |
| `rule_patch`'s `rule` closure | `![first, last, lo, hi].iter().all(\|b\| b.is_finite())` |

| `seen_region`, the edge heights | `!(ha.is_finite() && hb.is_finite())` |
| `seen_region`, the crossings | `!(a.is_finite() && b.is_finite())` |
| `seen_region`, the region bounds | `bounds.iter().all(\|b\| b.is_finite())` |
| `datum_view` | `!value.is_finite()`, raising `CameraError::NotFinite` |

The first two are the same question on the same quantity one call
apart; the third is its negation, spelled the other way round. The
next four drop the positivity half because it does not apply to a
coordinate, a height or a lattice bound. The last is not a predicate
at all: it is a TYPED refusal that names which side of the viewport
failed and carries its value.

**A ninth site asks a different question and is not a member**:
`View::viewport_px` tests `width.is_nan() || height.is_nan()`, because
`f64::max` would quietly answer with the other operand against a
`NaN` while an INFINITE side must pass through to the door that
refuses it. Its rustdoc says so at the site.

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

## What was done

Two doors, plus the typed one left where it is.

- `all_finite<const N: usize>([f64; N])` — the finiteness half, taken
  by array so a call site names its quantities. The four `seen_region`
  / `rule_patch` sites route through it.
- `positive_length(f64) -> Option<f64>` — finiteness AND sign,
  answering with the value so a caller holds a length rather than a
  permission to re-read the number. `metres_per_pixel_at`,
  `screen_metres_at` and `grid_pitch` route through it. **Built ON
  `all_finite`**, so the finiteness half has one spelling.
- `datum_view` keeps its own `is_finite`, with a comment saying why:
  the predicate doors answer yes or no about a SET, and the answer
  that door owes names the member.

`geom_core`'s `is_finite_length` was read and **declined**, with the
reason written beside `all_finite`: it is generic over `Real` and asks
through the scalar's poison channel because its callers may carry an
interval or a dual; its rustdoc's roster is a hand-kept claim about
doors that DECIDE a length against a norm witness; and the recourse it
names — rescale the geometry — is not one a display module can offer.
Routing display code through it would widen that roster with sites it
is not about and would still leave the sign half hand-spelled.

`crates/viewer/src/datums.rs` now holds two raw `is_finite` calls —
one inside `all_finite`, one inside `datum_view` — and the two
`is_nan` calls of `viewport_px`.

**The same predicate is hand-spelled six more times elsewhere in the
crate**: `positive-finite-predicate-has-six-homes-outside-datums-rs`.

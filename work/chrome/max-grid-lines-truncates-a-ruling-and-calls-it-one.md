---
id: max-grid-lines-truncates-a-ruling-and-calls-it-one
kind: issue
title: MAX_GRID_LINES returns a truncated ruling in the shape of a complete one
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

`crates/viewer/src/datums.rs`, `rule_patch`'s `rule` closure:

```rust
let count = ((last - first) as usize)
    .saturating_add(1)
    .min(MAX_GRID_LINES);
```

**`MAX_GRID_LINES` is 512, not 96** — measured against the tree on
2026-09-21. The defect is unchanged; the reachability arithmetic below
was taken at the wrong number and is re-derived at the end of this
row. When the patch genuinely holds more lattice
lines than the cap, the closure rules that many of them and returns — and the
caller receives a ruling indistinguishable from a complete one. There
is no flag, no count, no shortened patch: a partial grid in the shape
of a whole one, which is the class the sweep that closed
`inclusive-rule-range-draws-a-line-on-a-nan-count` and
`metres-per-pixel-swallows-a-nan-depth` was sent to close, at a site
that sweep read as a backstop and left standing.

The const's own doc says why it reads as a backstop: *"Not a budget
the design expects to spend"*. That argument is
about REACHABILITY and not about honesty: the cap fires exactly when
the arithmetic has gone somewhere the design did not plan for, which
is the case where the caller most needs to know the answer is partial.

## What the disposition might be

Three shapes, and choosing between them is the work:

- **Refuse.** A patch that cannot be ruled inside the backstop is a
  patch this view cannot rule, and the module's answer everywhere else
  to "I cannot compute this" is to draw nothing. Consistent, and
  costs a grid at exactly the views where a coarse grid might still
  orient a reader.
- **Rule what fits and say so**, by shrinking the patch to what 96
  lines cover rather than truncating the line list — the drawing is
  then complete for a smaller patch, which is a true statement.
- **Keep it and argue it at the site**, the way `grid_pitch`'s
  rustdoc argues its refusal: name the consumer, weigh a truncated
  grid against no grid. The project's standard is not "never
  substitute"; it is "weigh the two failures where you do".

## What is asserted about it today

Nothing. No row in `crates/viewer/tests/datum_draw.rs` drove a patch
past the cap in any direction, so it had never been observed firing.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the
territories table; the change is CHROME's alone.

## Reachability, re-derived at 512

Measured by driving `datums::draws` at a plane seen half a degree
above its own surface — an orbit position, not a pathology — with the
window width varied and everything else the suite's own `view_at`:

| window | lines ruled across |
| --- | --- |
| 1280 px | 109 |
| 3840 px | 323 |
| 7680 px | 512, capped (645 asked for) |

So the cap is crossed from an ordinary seat at a window wider than
about 6100 pixels — an 8K display, which exists. It is NOT dead at
every ordinary view, and the const's own doc was already carrying the
right order of magnitude for the grazing case (*"around 150 on a
1280-pixel window"*) while the row's 26 came from the head-on one.

## Disposition: shrink the patch

The second of the three. `rule_patch` now bounds BOTH directions
once, up front, and shrinks each index span to the cap centred on the
region (`capped_span`) before either family is ruled — so a capped
drawing is a complete ruling of a smaller rectangle, and the family
crossing the capped direction is shortened with it. The cap is now a
bound on the PATCH, which is a true statement the caller can see the
edges of, rather than a bound on the line list, which was not.

Refusing was rejected for the reason the row already names: it costs a
grid at exactly the views where a coarse grid still orients a reader,
and shrinking leaves the caller holding a true statement AND a grid.
Arguing it at the site was rejected because the argument would have to
be that a one-sided silent truncation is legible, and it is not —
under the old code the ruled family stopped at one bound while the
crossing family ran on to another, which is the shape the new row
measures.

Two things moved with it, both stated as invariants at the code: the
outward rounding of each direction's bounds now happens once rather
than once per family, and a bound that is not a number refuses the
WHOLE patch rather than leaving one family ruled between `NaN`
endpoints.

## What is asserted about it now

`a_patch_past_the_grid_backstop_is_shrunk_rather_than_truncated` in
`crates/viewer/tests/datum_draw.rs`. It reads the drawing back and
asserts the two families rule one rectangle, that the ruling tracks
the window while the backstop is slack, and that it stops growing when
the backstop bites. Each of the three dispositions reds it
differently: truncation on the rectangle, refusal on the emptiness,
no cap at all on the growth.

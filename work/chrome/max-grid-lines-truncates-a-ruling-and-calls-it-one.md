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

`MAX_GRID_LINES` is 96. When the patch genuinely holds more lattice
lines than that, the closure rules 96 of them and returns — and the
caller receives a ruling indistinguishable from a complete one. There
is no flag, no count, no shortened patch: a partial grid in the shape
of a whole one, which is the class the sweep that closed
`inclusive-rule-range-draws-a-line-on-a-nan-count` and
`metres-per-pixel-swallows-a-nan-depth` was sent to close, at a site
that sweep read as a backstop and left standing.

The const's own doc says why it reads as a backstop: *"Not a budget
the design expects to spend"* — a patch of `PATCH_COVER` windows at
`TARGET_PITCH_PX` per cell needs about 26 lines on a 1280-pixel
window, so the cap is dead at every ordinary view. That argument is
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

Nothing. No row in `crates/viewer/tests/datum_draw.rs` drives a patch
past 96 lines a direction, so the cap has never been observed firing.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the
territories table; the change is CHROME's alone.

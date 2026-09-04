---
id: drag-tick-has-three-homes
kind: issue
title: The drag tick has three homes and a count field has two answers — the creation forms pick from the four constants by hand
status: open
opened: 2026-09-04
refs: [1776]
---


## Finding

"How fast does a field of this dimension move" is answered in **three
places** in `crates/viewer/src/app.rs`, and in one of them the answers
already disagree.

**The three homes.**

1. `drag_tick(dimension)` (`app.rs:1093-1099`) — the canonical branch
   over the four constants, `FIELD_DRAG_SPEED` / `ANGLE_DRAG_SPEED` /
   `UNIT_DRAG_SPEED` / `COUNT_DRAG_SPEED` (`:1056`, `:1065`, `:1075`,
   `:1079`).
2. `FieldWriting::of` (`app.rs:1121-1166`, PR 1776) — `drag_tick` put
   through the field's written unit, plus the whole-numbers rule for a
   `Count`. The two PANEL fields (a slot's and a document parameter's)
   go through it, and so does the free-move probe.
3. **The creation forms, by hand.** ~30 call sites name one of the
   four constants at the call
   (`rg '_DRAG_SPEED,' crates/viewer/src/app.rs` — 41 lines, of which
   ~30 are a hand-picked argument to `named_field`, `unit_field`,
   `named_scalar` or `vec3_row`; e.g. `:5169`, `:5190`, `:5246`,
   `:5252`, `:5256`). `named_field` (`app.rs:5014-5028`) then does
   the SAME arithmetic `FieldWriting` does — `props::in_written(speed,
   unit)` — over a tick the caller chose rather than one derived from
   the field's dimension.

**The disagreement already standing.** A `Count` moves at **two
speeds**:

- `FieldWriting::of(Count, _).tick` is `1.0` — a count field lands on
  integers, and a tenth of an instance is not a value it can take.
- The pattern form's count field is `COUNT_DRAG_SPEED` = `0.1`
  (`app.rs:4106`), which is the same field for the same quantity in
  the form that creates it.

Both are defensible in isolation and they cannot both be right for one
user. `work/chrome/doc-params-carry-no-display-unit.md`'s own
correction argued the panel's `1.0` is right and did not look at the
form; that is the half this item carries.

Note what makes the form's case genuinely different: a creation form's
draft is a plain `f64` with no row behind it, so `FieldWriting::of`
needs a `(dimension, unit)` the caller supplies anyway. The
unification is therefore about deriving the TICK from the dimension
the form already knows, not about reaching a row.

## What to decide

1. Does a count field step by 1 or by 0.1? One answer, in one place.
   (`COUNT_DRAG_SPEED`'s own rustdoc says "dragged in tenths of one
   and lands on integers", which is a claim about egui's rounding, not
   about the tick — check it before choosing.)
2. Do the creation-form fields derive their tick from their dimension
   (`FieldWriting::of(dimension, Some(unit)).tick`) instead of naming a
   constant? That is a mechanical change over ~30 call sites and would
   leave `drag_tick` with exactly one caller.

## Why it is filed rather than taken

Both findings came out of the style review of PR 1776 (CHROME unit 8),
whose scope was the parameter panel row. Unifying the `named_field`
family is a separate, larger change over the creation forms, and the
count disagreement should be decided with it rather than before it —
fixing one end alone is how the two answers were minted.

## Home

CHROME. All of it is `crates/viewer/src/app.rs`.

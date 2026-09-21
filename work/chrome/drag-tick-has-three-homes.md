---
id: drag-tick-has-three-homes
kind: issue
title: The drag tick has three homes and a count field has two answers — the creation forms pick from the four constants by hand
status: open
opened: 2026-09-04
refs: [1776]
priority: P1
cost: E
---


## Finding

"How fast does a field of this dimension move" is answered in **three
places**, and in one of them the answers already disagree.

**The three homes.**

1. `forms::drag_tick(dimension)` (`crates/viewer/src/forms.rs`) — the
   canonical branch over the four constants, `FIELD_DRAG_SPEED` /
   `ANGLE_DRAG_SPEED` / `UNIT_DRAG_SPEED` / `COUNT_DRAG_SPEED`, all
   four declared in that same file beside it.
2. `FieldWriting::of` (`crates/viewer/src/forms.rs`, PR 1776) —
   `drag_tick` put through the field's written unit, plus the
   whole-numbers rule for a `Count`. It has **four callers, all in
   `crates/viewer/src/pane/properties.rs`**
   (`git grep -n 'FieldWriting::of' -- crates/viewer/src`): the two
   PANEL fields it was built for — `ViewerBehavior::slot_value_ui` and
   the `Selection::Param` arm of `ViewerBehavior::properties_ui` — plus
   the free-move probe, and `ViewerBehavior::add_param_ui`, which is a
   CREATION form and is the one this item's second half has already
   been done at.
3. **The creation forms, by hand.** The call sites name one of the four
   constants at the call, across three files:
   `crates/viewer/src/pane/create.rs`,
   `crates/viewer/src/widgets.rs` and
   `crates/viewer/src/pane/properties.rs`. `widgets::named_field`
   then does the SAME arithmetic `FieldWriting` does —
   `props::in_written(speed, unit)` — over a tick the caller chose
   rather than one derived from the field's dimension; its siblings
   `unit_field`, `named_scalar` and `vec3_row` take a tick the same
   way.

**The population, re-derived 2026-09-15.** Two commands, and what
each prints:

```
git grep -n '_DRAG_SPEED,' -- crates/viewer/src | wc -l      # 45
git grep -c '_DRAG_SPEED,' -- crates/viewer/src              # the per-file split
```

The second prints four `file:count` lines, not a total —
`pane/create.rs:22`, `widgets.rs:17`, `forms.rs:4`,
`pane/properties.rs:2` — and the first is where the 45 comes from.

**45 lines is not 45 call sites**, in both directions.

*Subtract 8.* Four are `drag_tick`'s own match arms (`forms.rs`), which
are the definition and not a use of it; four are `use` lines (two in
`pane/create.rs`, one each in `widgets.rs` and `pane/properties.rs`).

*Add 1.* The grep matches on a trailing comma, so it cannot see a call
that passes the speed as its LAST argument.
`git grep -n '_DRAG_SPEED' -- crates/viewer/src | grep -v
'_DRAG_SPEED,'` returns 9 such lines — four constant declarations, four
rustdoc mentions, and **one real call site**:
`number_field(&mut self.drafts.pattern_count, COUNT_DRAG_SPEED)` in
`pane/create.rs`, which is the very site the count disagreement below
is about.

So the hand-picked population is **38 call sites across three files**
— `pane/create.rs` 21, `widgets.rs` 16, `pane/properties.rs` 1 —
against the `~30 of 41 in app.rs` this row was filed with. `forms.rs`
contributes none: all four of its hits are `drag_tick`'s own arms. The
single `pane/properties.rs` site is the no-dimension placeholder in
`add_param_ui` (below); that file's other grep line is its `use`.

**What both commands are blind to.** A tick held in a local
(`let speed = …;` then `named_field(ui, …, speed, …)`) matches neither,
and so does a bare numeric literal passed where a constant belongs.
The literals exist: `number_field(value, 0.5)` and
`egui::DragValue::new(value).speed(0.5)` appear in `widgets.rs` — but
only inside its two `#[cfg(test)]` modules, as harness fixtures rather
than as chrome, so they are outside this item's subject and are noted
here only so the next sweep does not re-find them and count them in.

**`crates/viewer/src/app.rs` holds none of this.**
`git grep -n 'FieldWriting\|crate::forms' -- crates/viewer/src/app.rs`
returns exactly three lines, and not one of them is a definition: the
`pub use crate::forms::FieldWriting;` itself — a path to the type, not
a home for the rule — a `//` line comment above it explaining why the
re-export exists (a line comment, not a doc comment), and one `//!`
module-doc line naming `crate::forms` as a module, which is not a
mention of the type at all. The row was filed before
`viewer-session-god-module-split` (#1830) and cited `app.rs`
throughout; `drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs`
reported that and this pass discharges it.

**The class is partly converted already.** `ViewerBehavior::add_param_ui`
and `ViewerBehavior::slot_value_ui` (both `pane/properties.rs`) derive
their tick from `FieldWriting::of` rather than naming a constant —
`add_param_ui` through `FieldWriting::of(dimension, None).tick`, with
`FIELD_DRAG_SPEED` only as the placeholder for "no dimension picked
yet". So the shape the second half of this item proposes is already
shipped at two sites, and what is open is the creation forms and the
widget vocabulary they call.

**The disagreement already standing.** A `Count` moves at **two
speeds**:

- `FieldWriting::of(Count, _).tick` is `1.0` (`forms.rs`) — a count
  field lands on integers, and a tenth of an instance is not a value
  it can take.
- The pattern form's count field is `COUNT_DRAG_SPEED` = `0.1`
  (`number_field(&mut self.drafts.pattern_count, COUNT_DRAG_SPEED)`,
  in `ViewerBehavior::pattern_tool_ui`,
  `crates/viewer/src/pane/create.rs`), which is the same field for the
  same quantity in the form that creates it.

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
   constant? That is a mechanical change over the 38 call sites above
   — 37, once `add_param_ui`'s placeholder is read as already done —
   and would leave `drag_tick` with exactly one caller
   (`FieldWriting::of`).

## Why it is filed rather than taken

Both findings came out of the style review of PR 1776 (CHROME unit 8),
whose scope was the parameter panel row. Unifying the `named_field`
family is a separate, larger change over the creation forms, and the
count disagreement should be decided with it rather than before it —
fixing one end alone is how the two answers were minted.

## Home

CHROME. Four files, all `crates/viewer/src/`: `forms.rs` (the rule and
the four constants), `widgets.rs` (the field vocabulary that takes a
tick as an argument), `pane/create.rs` (most of the hand-picking, and
the count disagreement) and `pane/properties.rs` (the two converted
sites).

## Un-parked — the trigger fired (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04, so this row's
only blocker is gone and the row is dispatchable. Un-parked here, from
VIEW's PR #1857, rather than by CHROME: on Ev's ruling there, `work.py
lint` now REFUSES a `parked` row whose every blocker is closed, and a
program cannot un-park another program's rows in the PR that closes
their trigger — `work/README.md`'s one-file-one-item rule makes that a
merge conflict by design.

## Re-pointed by subject (2026-09-15, `chrome/citation-repoint`)

Every citation above was re-derived by finding its subject by name in
the tree at `385c01b3`; no line number was shifted and none is written
here, per `docs/prompts/implementer-discipline.md` §7. The report that
prompted it,
`drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs`, is
closed by it — including the `likely` it carried on the 41 → 45
reading, which the population section above settles.

**Corrected in the same branch's fix pass**, after a style review
caught this pass minting fresh errors into a row it was repointing —
the defect it exists to close:

- The `FieldWriting::of` caller list read as a census and named three
  of four. It is four; `add_param_ui` is the fourth, and the one that
  matters most here, because it is a creation form already doing what
  this item proposes.
- `git grep -c` was described as "giving 45 lines". It prints four
  `file:count` lines. The closed report's whole complaint was a
  reproduction command that misleads whoever runs it, so both commands
  are now shown with what each actually prints.
- "Two of those 38 are `pane/properties.rs`" contradicted the
  subtraction three sentences above it — that file's two grep lines
  include the `use` already subtracted. It is one, and the 38 is now
  decomposed per file so the arithmetic is checkable.
- "Two doc-comment mentions" in `app.rs` was a fresh, wrong census:
  the `FieldWriting` mention is a `//` line comment, and the other line
  names the `forms` MODULE, not the type. Replaced with the grep and
  what its three lines are.

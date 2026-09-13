---
id: a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets
kind: issue
title: a drag field renders a length at a precision its drag speed sets, and can show a length as zero
status: closed
opened: 2026-09-12
refs: [fixed-precision-length-renders-can-read-as-a-value-they-cannot-be, parameter-row-field-has-no-text-door, nothing-holds-a-new-numeric-field-to-the-fields-door, a-fields-text-commits-within-the-renders-own-tolerance]
closed: 2026-09-12
---



(VIEW) Found by the re-derivation for
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`,
as **the member that item's sweep could not match**. That sweep ranged
over a precision spec in a format string (`{…:.N}`) and stated its own
blind spots — no spec at all, a variable precision, a hand rounding.
The property is *a length the chrome renders at a precision fixed
independently of the length*, and there is a fifth way to write one that
is none of those three: hand the number to a widget that picks the
precision itself.

## What the widget does

`egui::DragValue` with no `max_decimals` derives its precision from its
DRAG SPEED. `drag_value.rs:476-480` (egui 0.36.1):

```
let auto_decimals = (aim_rad / speed.abs()).log10().ceil().clamp(0.0, 15.0) as usize;
let max_decimals  = max_decimals.unwrap_or(auto_decimals + 2).at_least(min_decimals);
```

`aim_rad` is `physical_pixel_size()` — `1.0 / pixels_per_point`
(`input_state/mod.rs:804-807`). The text is then
`format_with_decimals_in_range(value, auto_decimals..=max_decimals)`
(`drag_value.rs:531-535`), which is the same SHAPE as this crate's own
render — shortest decimals in a range that reads back — with one
difference that is the whole defect: **when nothing in the range reads
back it returns the range's maximum anyway**
(`emath-0.36.1/src/lib.rs:231-247` — the loop, then
`format!("{value:.max_decimals$}")` under a comment that says *"show the
full value"*), where `crate::readout::number` changes form instead.

So the precision is a function of the drag speed and the display
scaling, and of nothing about the value.

## The arithmetic, for a length field

`forms::FIELD_DRAG_SPEED` is `0.0005` metres — half a millimetre
(`crates/viewer/src/forms.rs:364`), and a field shown in millimetres
passes `props::in_written(speed, unit)`, so `speed = 0.5`
(`crates/viewer/src/widgets.rs:275`). At `pixels_per_point = 1`,
`auto_decimals = ceil(log10(1 / 0.5)) = 1` and `max_decimals = 3`: the
field's coarsest spelling is `{:.3}` over millimetres, which is the
exact spec `delta-field-renders-a-sub-micrometre-delta-as-zero` was
filed against. At `pixels_per_point = 2` it is `{:.2}`. A slot holding
40 nm reads `0.00`, and a slot holding 1.6 µm reads `0.002`.

## The sites

Every `DragValue` in the crate over a value that is a LENGTH. The
census is `grep -rn 'DragValue::new' crates/viewer/src` — eleven hits —
then reading each for what its value denotes, because the pattern finds
the TYPE and the property is the DIMENSION, and nothing in the call says
which. Blind spot of the grep: a `DragValue` built through a helper that
wraps it, of which this crate has none today.

Members, all in production chrome:

- `crates/viewer/src/widgets.rs:275` — `named_field`, the creation
  forms' unit-bearing field, and the one whose speed is converted into
  the display unit, so the arithmetic above is exactly its.
- `crates/viewer/src/widgets.rs:207-209` — `vec3_row_ops`, and
  `crates/viewer/src/widgets.rs:226` — `vec3_row`. Both take the speed
  from the caller, which is `forms::FIELD_DRAG_SPEED` wherever the
  vector is a length (a datum origin, a placement translation) and
  `UNIT_DRAG_SPEED` where it is a direction. The call site decides
  membership, which is why neither function can be read as one.
- `crates/viewer/src/pane/properties.rs:80` — a parameter row's value,
  at `field.tick`, which is `forms::drag_tick(dimension)` and is
  `FIELD_DRAG_SPEED` for a `Length`.
- `crates/viewer/src/pane/properties.rs:550` — a slot row's value. It
  carries a `custom_parser` and no `custom_formatter`, so the authoring
  half is this crate's and the display half is still egui's.
- `crates/viewer/src/pane/properties.rs:196` — a new parameter's value,
  at the same tick through `FieldWriting`.

Not members, and each for its own reason rather than by a shared rule:

- `crates/viewer/src/widgets.rs:293` — `named_scalar`, dimensionless by
  its own doc.
- `crates/viewer/src/pane/create.rs:1007` — a pattern COUNT at
  `COUNT_DRAG_SPEED`, whole by construction and `range`-clamped.
- `crates/viewer/src/widgets.rs:769` — a length in millimetres at speed
  `0.5`, and it WOULD be a member but for sitting inside
  `#[cfg(test)] mod tests` (`crates/viewer/src/widgets.rs:645-646`): a
  gesture harness, not chrome a user reads.

An angle field is not a member for the reason
`fixed-precision-length-renders-can-read-as-a-value-they-cannot-be`
gives at the camera readout: `0.0` is an angle a thing really has.

## Why it was not taken with the four

Three reasons, and the third is the one that decides it.

1. It is a different mechanism. The four were format strings this
   crate writes; this is a precision a library derives, and the repair
   is `DragValue::custom_formatter` rather than an edit to a sentence.
2. It is an EDITABLE field, not a readout. `DragValue` writes back only
   on a real edit (`drag_value.rs:520-527` rounds only when
   `change != 0.0`), so a field reading `0.00` does not commit `0.00` —
   the value shown is wrong, the value held is not. That is a weaker
   defect than the badge's and a different argument.
3. **A custom formatter is one decision about every numeric field in
   the chrome**, not five patches: it would have to answer what a drag
   gesture means when the rendered text no longer matches the tick the
   drag steps by, which is a question about the gesture and not about
   the render. `crate::readout::number` is the rule to hand it; what it
   is not is the answer to that question.

Related but not this: `work/chrome/parameter-row-field-has-no-text-door`
is about the same widget lacking a PARSER, which is the authoring half.
This row is the display half and neither subsumes the other.

## Closed, 2026-09-12

**Fixed, and wider than the item framed it — because the item's third
reason for holding it back turned out to be an argument FOR the wide
shape once the question it names was answered from egui's source.**

`crate::widgets::number_text` is the rule: *a field's text reads back as
the value the field holds*. It keeps the widget's own spelling wherever
`crate::readout::reads_back` accepts it, and hands the rest to
`crate::readout::number`. `crate::widgets::number_field` is the door
that attaches it, and **all eleven `DragValue::new` sites in this crate
now go through that door** — the ten in production chrome and the one in
`widgets.rs`'s own gesture harness.

### The mechanism, re-derived rather than taken from this item

The item's citations of egui are right, including
`emath-0.36.1/src/lib.rs:231-247`: the loop opens at `:231`, the
comment *"In any case: show the full value"* is at `:245`, and the
give-up `format!("{value:.max_decimals$}")` is at `:247`. Four
corrections to what the item says around them:

1. **The default formatter is one hop further away.** The `None` arm
   at `drag_value.rs:530-536` goes through
   `ui.style().number_formatter`, which is
   `emath::format_with_decimals_in_range` only because `style.rs:1434`
   makes it so. That hop is also a third repair shape the item does not
   name — `Style::number_formatter`, set once — and it is in the
   residue row below.
2. **The fallback is not the only place the spelling misreads the
   value.** egui's acceptance test is `almost_equal` in **f32** at
   `16 * f32::EPSILON` (`emath-0.36.1/src/lib.rs:235-237`), about
   1.9·10⁻⁶ relative, with a degenerate clause that returns true for
   ANY pair once both are under that epsilon. So the in-range spelling
   is already a ~10⁻⁶ approximation, not a reading-back.
3. **Reason 2 of *"Why it was not taken with the four"* is false, and
   its falsity is the whole stake of the row.** The item says a field
   reading `0.00` does not commit `0.00` because `drag_value.rs:520-527`
   rounds only when `change != 0.0`. That is true of the DRAG and the
   arrow keys and of nothing else. A `DragValue` seeds its keyboard
   edit with the text it last showed (`:554-557`) and parses that text
   back on losing focus (`:540-552` and `:577-591`), unconditionally on
   whether the text changed. **Clicking into a field and clicking away
   again commits what the field said.** Driven through the real widget
   headlessly: a field holding 4·10⁻⁵ mm came back holding **0.0**, and
   one holding 1.6·10⁻³ mm came back holding 2·10⁻³. This is not a
   weaker defect than the badge's; it destroys the value.
   `crate::pane::properties`'s `slot_value_ui` already knew the path is
   there — its *"Text that says what the slot already says is not an
   edit"* guard exists for exactly that click — and the guard assumed
   the render round-trips.
4. **`properties.rs:550` is not quite *"a `custom_parser` and no
   `custom_formatter`"***: the same builder takes a `custom_formatter`
   at `:567` whenever the slot is driven, errored, or under an
   expression edit. The item's sentence is true of the case that
   matters (a literal slot with a value) and not of the site.

### The fork, answered from the widget rather than weighed

The dispatch asked what a drag means when the text stops matching the
tick. **It never does, and that is provable rather than arguable.** A
drag commits `emath::round_to_decimals(value, auto_decimals)`
(`drag_value.rs:654-659`) — `auto_decimals` being the BOTTOM of the
same range the formatter is handed — so every value a drag produces is
spelled exactly by the range's shortest member, both rules return that
member, and the text a drag steps through is unchanged.
`a_field_shows_what_the_widget_shows_wherever_that_reads_back` asserts
that as sameness over the drag's own landing set (every tenth of a
millimetre from -20,000 to 20,000), so the claim is falsifiable rather
than stated.

The item's reason 3 — *"a custom formatter is one decision about every
numeric field, not five patches"* — is therefore right about the scope
and wrong about the conclusion. It is one decision, and the decision is
cheap once the gesture question is closed.

### So the five members are the wrong population

The property is *a field whose text is not the value it holds*, which
has no dimension in it. A dimensionless field reading `0.00` over
1.6·10⁻⁵ and an angle field reading `0.000` over a microradian make the
same false claim, and each is one click from committing it — so
`named_scalar` and every angle field are members too, and the item's
angle exclusion imported a rule from the READOUT class (*is zero a
value this thing can have*) into a class about whether the text names
the number. **The count field is the one real non-member, and now
provably rather than by classification**: `DragValue::new` gives an
integral value `max_decimals(0)` (`drag_value.rs:61-65`), so its range
is `0..=0`, its only spelling is `{:.0}`, and a whole number reads back
as itself. Nothing in the door has to know which fields those are.

### What did not change, and what it cost

- **No value a drag can produce renders differently.** Asserted.
- **Nothing at or above 1 in the display unit renders differently** on
  a millimetre length field at one point per pixel: the widget's widest
  spelling there is `{:.3}`, whose absolute error is at most 5·10⁻⁴, so
  it clears `REL_TOLERANCE` for every `|value| >= 1`. Measured; the
  largest failing magnitude on that field is 0.966.
- **`readout::number`'s ten-character bound and scientific arm are
  never reached for a large value**, which was the live risk in reusing
  it here: they are reached only where the widget's own spelling
  already misreads the value, and that band is bounded above.
- The two residues are files:
  `nothing-holds-a-new-numeric-field-to-the-fields-door` (no guard
  holds a twelfth site to the door) and
  `a-fields-text-commits-within-the-renders-own-tolerance` (the render
  is accepted within 5·10⁻⁴, and committing it moves the value by that
  much).

### The sweep, and what it could not match

`grep -rn 'DragValue::new' crates/viewer/src` — eleven hits, all eleven
routed, which is the item's own census re-run at this merge base and
agreeing with it. **Its blind spot is the item's and is now a row**: a
`DragValue` built through a helper that wraps it, of which this crate
still has none — except that after this unit it has exactly one, and it
is the door. The tracker pass found no duplicate on any slate; the
CHROME row it would have collided with,
`parameter-row-field-has-no-text-door`, is the PARSER half and is
narrowed rather than closed by this; its four pre-split `app.rs`
citations are reported as
`work/chrome/parameter-row-field-cites-a-pre-split-app-rs`.

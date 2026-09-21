---
id: a-fields-text-commits-within-the-renders-own-tolerance
kind: issue
title: a field's text is accepted within the render's relative tolerance and committing it moves the value by that much
status: open
opened: 2026-09-12
refs: [parameter-row-field-has-no-text-door]
priority: P0
cost: D
branch: vgeom/p0-fields
---



(VIEW) Residue of
`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`,
which closed the band where a field's text was GROSSLY wrong and left
this one, which is the band where it is slightly wrong.

## The finding

`crate::widgets::number_text` keeps the widget's own spelling wherever
`crate::readout::reads_back` accepts it, and that predicate accepts a
spelling within `crate::readout::REL_TOLERANCE` — 5·10⁻⁴ of the value,
the scientific form's own worst case. A field's text is also what
clicking into it and clicking away again commits
(`egui-0.36.1/src/widgets/drag_value.rs`, the `lost_focus` arms), so
**a click-in and a click-away can move a value by up to 5·10⁻⁴ of
itself**, and that is now the stated bound rather than an accident.

Worked: a millimetre field at one point per pixel is handed `1..=3`.
A value of 1000.001 mm is spelled `1000.0` by the widget — egui's own
acceptance test is `almost_equal` in **f32** at `16·f32::EPSILON`
(`emath-0.36.1/src/lib.rs:235-237`), about 1.9·10⁻⁶ relative, which
`1000.0` passes — and `reads_back` passes it too. Clicking in and away
commits 1000.0 and loses the micrometre.

## Why the closing unit did not take it

Because the tolerance is not the spelling's to choose. The spelling in
that band is egui's, and it is egui's on purpose: a drag commits
`round_to_decimals(value, auto_decimals)`, so the bottom of the
widget's own range spells every value a drag produces exactly, and
keeping the widget's spelling wherever it reads back is what makes the
render rule invisible to a gesture. Tightening the acceptance test
below egui's would start changing the text a drag steps through, which
is the interaction question that unit was able to avoid answering
precisely because it did not have to.

## What a fix would have to answer

Whether the right bound here is a RENDER's or a COMMIT's. They are
different questions with different answers: `REL_TOLERANCE` is derived
from what four significant figures can misread by, which is a property
of the text; the commit wants a bound on how far the document may move
when nobody typed anything, which is a property of the edit. The
honest floor for the second is zero — no keystroke, no change — and
reaching it means either an exact spelling (17 significant figures for
1.85% of values a user typed, measured on a tenth-millimetre grid over
`in_written`/`from_written`) or not committing an untouched field at
all, which is `work/chrome/parameter-row-field-has-no-text-door`'s
no-op-guard half generalised to every field.

## Both panel value fields now refuse the echo (2026-09-21, AUTH-2)

The generalisation this row's last paragraph names — *"not committing
an untouched field at all, which is
`parameter-row-field-has-no-text-door`'s no-op-guard half generalised
to every field"* — landed for the two fields of the property panel.

**The guard is over TEXT, and it has no tolerance.** `crate::props::
echoed` is the rule's one home: the field's own formatter is the one
that produced the text an `egui::DragValue` seeds its keyboard edit
with, so `crate::widgets::value_field_ops` keeps what the formatter
returned and the parser compares the typed text against it. Equal is
the field talking to itself and emits nothing; different is the user's
and takes its door. `pane::properties`' `slot_value_ui` and the
`Selection::Param` arm are one call to that function.

**That answers this row's own question in one direction.** The bound
on a COMMIT is now zero — no keystroke, no change — without touching
the RENDER, which still spells a text naming its value only to
`REL_TOLERANCE`. The two questions came apart because the echo is
identifiable as text, which needs no bound at all.

**What is left here, stated rather than assumed.**

- every OTHER field in the crate, which is most of them: the creation
  forms' `widgets::unit_field` / `named_field` write back on
  `response.changed()` and have no such guard. The δ field is the
  exception and guards itself (`pane::view`'s
  `a_draft_typed_back_to_the_render_commits_nothing`), which is a
  second spelling of the same rule and a candidate for the same home;
- the render itself, which still spells a text naming its value only
  to `REL_TOLERANCE` — so a field can go on SHOWING a number it does
  not hold, which is a display fault even where nothing commits it;
- the sibling this shape turned up:
  `a-typed-field-hands-its-text-over-on-two-frames`, which is why the
  field's guard is not the only thing standing between one typed
  number and two undo steps.

**Corrected.** An earlier entry here recorded AUTH-2 as taking a
numeric guard over `readout::reads_as` and named its cost as *"the
chrome cannot tell an echoed text from a re-typed one"*. That was
wrong in both halves and was replaced before AUTH-2 merged: the
numeric shape discarded real edits — a field reading `1000` in
millimetres and a user typing `1000.4` got nothing, no edit and no
refusal — and the echo IS distinguishable, as text, which is what the
landed guard does.

## The COMMIT half is now closed for every field the chrome builds (2026-09-21, `vgeom/p0-fields`)

The first of the three bullets above — *every OTHER field in the
crate, which is most of them* — is answered, at the constructor rather
than at the ten call sites.

**`crate::widgets::number_field` carries the guard now.** It stashes
what its formatter returned and its parser asks `crate::props::echoed`
the same question `value_field_ops` asks: text equal to the field's own
render parses to nothing, text that differs takes the number door. So
`unit_field`, `named_field`, `named_scalar`, the vector and point
rows, the two form counts and the panel's new-parameter field all
inherit it, and `named_field`'s `response.changed()` write-back — which
`egui` marks exactly when the value MOVED, so the one gesture that
fired it was an inexact echo — no longer fires on a click that typed
nothing.

**Cost, stated rather than absorbed.** The parser is now
`crate::props::field_edit`'s rule (`f64`'s own parse over the trimmed
text), where it was `egui`'s private `default_parser`. `egui`'s strips
interior whitespace and maps U+2212 to a hyphen, so `1 234 567` and a
typographic minus stop parsing in the creation forms. The panel's two
value fields have never accepted either — `value_field_ops` has read
text through `field_edit` since AUTH-2 — so this makes the chrome
consistent rather than making it poorer in one half; it is recorded
here because it is a user-visible input change nobody asked for.

**What is left, unchanged from the list above.**

- **the render itself**, which still spells a text naming its value
  only to `REL_TOLERANCE`, so a field can go on SHOWING a number it
  does not hold. Not taken here: tightening it means changing
  `readout`'s own ratified accuracy rule, and the bound a RENDER owes
  is the question this row says a fix would have to answer. The
  question is now clean, because the commit no longer rides on it.
  **It moved in one direction this branch:**
  `the-fields-door-has-no-width-bound-at-all` bounds the field's text
  by `readout::MAX_CHARS`, so above `1e8` mm (`1e7` with a sign) the
  field falls from `egui`'s spelling — accurate to `f32`'s
  16·`f32::EPSILON`, about 1.9·10⁻⁶ — to `readout::number`'s, accurate
  to `REL_TOLERANCE`. A wider render band, over a commit path that is
  now closed. The two rows are one door and were taken together for
  this reason.
- **`a-typed-field-hands-its-text-over-on-two-frames`**, its own row.
- **`a-bare-field-still-commits-its-own-render`**, filed by this
  branch: the floor under the door carries the render as a context
  default and cannot carry this guard, because `egui::Style` has a
  `number_formatter` and no parser.

**Rows**: `widgets::field_tests::a_field_whose_render_is_not_exact_still_commits_nothing`
(the item's own `1000.001` worked example, both signs, plus the value
the width bound moved) and
`a_creation_forms_field_commits_nothing_on_a_click_through`, driven
through `named_field` so the canonical→written→parse→canonical round
trip is the one under test.


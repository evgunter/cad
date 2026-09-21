---
id: a-fields-text-commits-within-the-renders-own-tolerance
kind: issue
title: a field's text is accepted within the render's relative tolerance and committing it moves the value by that much
status: open
opened: 2026-09-12
refs: [parameter-row-field-has-no-text-door]
priority: P0
cost: D
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

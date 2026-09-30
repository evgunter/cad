---
id: a-fields-text-commits-within-the-renders-own-tolerance
kind: issue
title: a field's text is accepted within the render's relative tolerance and committing it moves the value by that much
status: closed
opened: 2026-09-12
closed: 2026-09-22
refs: [parameter-row-field-has-no-text-door, the-render-grids-cap-is-a-length-and-angles-go-through-it]
priority: P0
cost: D
pr: 3068
branch: vgeom/render-grid
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



## Orchestrator's disposition on the partial (2026-09-21)

Accepted as a partial and left OPEN deliberately. Recording it here
because a row that stays open after a unit touched it otherwise reads
as a unit that ran out of time.

**What closed**: the commit half, at the constructor, so all ten call
sites inherit it rather than ten guards inheriting a convention.

**What did not, and why it is not a follow-up to be scheduled
casually**: the render still spells a value only to `REL_TOLERANCE`.
Tightening that changes `readout`'s own **ratified** accuracy rule, so
it is a design question and not a lane's to answer. It now sits clean,
which is the thing the unit bought beyond the fix: the question *what
bound does a RENDER owe* no longer has a commit path riding on its
answer.

**The user-visible cost is disclosed above and is the orchestrator's
to carry, not the lane's.** Moving the creation forms' parser from
`egui`'s `default_parser` to `props::field_edit` drops interior-space
and U+2212 spellings THERE, which the properties panel has never
accepted. Consistent rather than poorer in one half, and put to Ev in
chat rather than absorbed silently. If Ev wants both spellings kept,
that is a new row against the shared parser and not a revert of this.


## The RENDER half is answered, and the row closes (2026-09-22, `vgeom/render-grid`, #3068)

The question this row said a fix would have to answer — **what bound
does a RENDER owe** — is answered: *the accuracy the kernel can
decide*, not the accuracy a box can hold.

**`crate::readout`'s grid is now two-armed.** `tolerance(value)` is
`EPS_CAP.min(value.abs() * REL_TOLERANCE)`, where `EPS_CAP` is
`DEFAULT_EPS * 0.1` — one decade below ε, compile-time and never the
run's live `Tolerance::eps()`. The shape, the `min`, the cap's decade
and the refusal to put a FLOOR under it are all
`crates/profile/src/path.rs`'s `num`'s, and the reason is ε being a
LENGTH (`docs/DESIGN.md` D4 ¶1): a purely relative rule crosses ε and
is coarser above the crossing, so two lengths the kernel certifies as
different rendered as one number.

**`REL_TOLERANCE` stayed at 5·10⁻⁴ and that is the arm's decision,
argued rather than inherited.** It was never wrong; it was
incomplete. The arms cross at 2·10⁻⁷ of a display unit, and below the
crossing the relative arm is finer than the cap by construction — so
it never decides whether the render can separate two values the kernel
separates, and four figures there is what ε justifies. Taking `num`'s
`1e-9` would have moved the crossing to a tenth of a display unit and
sent most sub-millimetre values into scientific notation for figures
no probe established.

**The row's own worked example, both directions**: a field holding
1000.001 mm rendered `1000.0` and now renders `1000.001`.
`readout::tests::a_millimetre_value_keeps_the_micrometre_it_holds` is
that row, and
`two_values_the_kernel_decides_between_render_as_two_numbers` is the
general claim — two lengths one ε apart, at magnitudes where a
relative rule spelled them the same.

**`MAX_CHARS` gave way, not the grid**: 10 → 22, re-derived by the
rule it always had (the decimal arm is held to the scientific arm's
worst case), which is now the exact spelling at the top of `f64`. The
band that used to exceed the bound no longer does, so the module has
no excepted band. `pane::view`'s `FIELD_WIDTH` doubled to meet it.

**What did NOT close here, and each has its own file:**

- `the-render-grids-cap-is-a-length-and-angles-go-through-it` — the
  cap is ε-derived and ε is a length; the chrome renders angles and
  scalars through the same door, where the cap is never too coarse
  and never argued.
- `the-camera-hud-spells-its-angles-at-a-fixed-tenth-of-a-degree` —
  the sweep's one production hit, waiting on the row above.
- `a-bare-field-still-commits-its-own-render` and
  `a-typed-field-hands-its-text-over-on-two-frames` — unchanged, and
  neither is this question.

**Disclosed and judged an improvement**: a drag's text is no longer
byte-identical to `egui`'s. `format_with_decimals_in_range` accepts a
spelling its own `f32` `almost_equal` passes (~1.9·10⁻⁶ relative),
which is coarser than the grid, so where they disagree the field shows
the finer spelling mid-drag. The property that survives is the one
worth having and is asserted over ~9000 magnitudes: a drag's text
names the value the drag commits, exactly.

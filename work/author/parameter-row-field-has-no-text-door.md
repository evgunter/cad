---
id: parameter-row-field-has-no-text-door
kind: issue
title: A parameter row's value field is a bare DragValue — no parser, no unit authoring, no no-op guard
status: closed
opened: 2026-09-04
refs: [1776]
priority: P0
cost: D
branch: author/param-notation
pr: 2957
closed: 2026-09-21
---


## Finding

**A slot's value field is a text door; a document parameter's is a
bare `DragValue`.** `props`' module docs describe the text door as
"One field for numbers and expressions"
(`crates/viewer/src/props.rs`, the module docs' own section heading)
with no disclaimer that it is a slot's field only — the section was
written when parameters had no panel story at all, and CHROME unit 8
(PR 1776) brought the parameter row up to the units half without
bringing it up to this one. The disclaimer is now in the docs; the
affordance is not.

What the slot field has that the parameter field does not
(`ViewerBehavior::slot_value_ui` against the `Selection::Param(name)`
arm of `ViewerBehavior::properties_ui` — both in
`crates/viewer/src/pane/properties.rs`):

1. **A `custom_parser` routing typed text through `props::field_edit`**
   — the `.custom_parser(…)` arm of `slot_value_ui`. It is what makes
   `50 mm` an authoring of a unit and `base_r * 2` an authoring of an
   expression. The parameter field has none, so egui's own number parse
   is the whole door: text is a number or it is nothing.
2. **The "text that says what the slot already says is not an edit"
   guard** — the `match typed.into_inner()` arm of `slot_value_ui`,
   under the comment that names it. The field commits on leaving it, so
   clicking into one and clicking away again must not cost an undo
   step. The parameter field has no such guard.

## What each half is worth, and what each half needs

- **The expression half does not apply.** A `DocParam::Continuous`
  holds an `f64`, not an `Expr` (`DocParam` in
  `crates/editor-core/src/doc.rs`),
  so there is no expression a parameter can be set to and no
  `SetDocParamExpression` to reach. Typing `base_r * 2` into a
  parameter field is a refusal, not a door — and saying so is itself
  an affordance the field currently does not offer.
- **The unit half is `50 mm`.** Setting the value AND its notation
  from one piece of text needs the kernel door that does not exist
  (`work/edit/doc-param-unit-edit-has-no-door.md`) — unless the
  typed unit is the one the parameter is already declared in, which is
  a `SetDocParamValue` and works today.
- **The no-op guard is free** and belongs to this crate alone.

## Its relation to the sibling item

`work/author/add-parameter-form-authors-canonical-only.md` is the
CREATE door's version of the same question. They are two items
deliberately: that one mints a declaration (and can be done today
through `written_length`/`written_angle`), this one edits a standing
one (and its unit half cannot). Whoever takes either should read the
other — the design call about how notation crosses `props` is shared.

## Home

AUTHOR (here since CHROME's 2026-09-20 priority-seam cut,
`work/author/log.md`). The field is `crates/viewer/src/pane/properties.rs`
(`ViewerBehavior::properties_ui`'s `Selection::Param` arm, with
`ViewerBehavior::slot_value_ui` beside it as the shape to copy); the
parse door is `crates/viewer/src/props.rs` (`props::field_edit`). The
unit half is blocked on the `editor-core` issue above; the guard and
the refusal wording are not.

## The fired entry pruned, still parked (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04. This row also
waits on `doc-param-unit-edit-has-no-door`, which is open, so `parked`
is still TRUE of it and only the fired entry leaves `blocked_on`.
Pruned from VIEW's PR #1857 for the reason the eight sibling rows carry:
a `blocked_on` entry that has fired is stale data whether or not the
status it supports is still true. It is a lint WARNING rather than an
error precisely because this row is genuinely still blocked.

## Re-pointed by subject, still parked (2026-09-15, `chrome/citation-repoint`)

Four `app.rs` bands in this row named lines past the end of that file —
they were addresses in the 5,696-line `app.rs` that
`viewer-session-god-module-split` (#1830) took apart. All four subjects
now live in `crates/viewer/src/pane/properties.rs` and are re-derived
above **by name**, with no line number written, per
`docs/prompts/implementer-discipline.md` §7. The report that found
them, `parameter-row-field-cites-a-pre-split-app-rs`, is closed by this
pass.

Two further corrections the same pass made, neither of them a number:

- `work/issues/doc-param-unit-edit-has-no-door.md` is now
  `work/edit/doc-param-unit-edit-has-no-door.md` — the item was
  claimed by EDIT, not deleted. It is **open**, which is why this row
  is still `parked` and its `blocked_on` still resolves.
- `## Home` said the field is `crates/viewer/src/app.rs`. It is not,
  and had not been since #1830.

**What VIEW narrowed underneath this row, carried here so it does not
die with the report that disclosed it.**
`work/view/a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`
gave every numeric field in the crate — the parameter row's included —
a text that reads back as the value it holds, so a click-in/click-away
is no longer DESTRUCTIVE (a parameter holding 40 nm used to become a
parameter holding zero). It is still an EDIT: the render is accepted
within `crate::readout::REL_TOLERANCE`, so the round trip can still
move a parameter by up to 5·10⁻⁴ of itself and cost an undo step for a
click nobody meant as one. The no-op guard this row asks for is
therefore still owed, with a smaller stake.
`work/view/a-fields-text-commits-within-the-renders-own-tolerance` is
the same residue read from the other side and is open.

## The unit half's blocker is built (2026-09-16, EDIT)

`doc-param-unit-edit-has-no-door` landed the kernel door this row's
unit half waits on: `DocEdit::SetDocParamUnit { name, unit }`, routed
through `DocParam::with_display_unit` (`crates/editor-core/src/edit.rs`,
`doc.rs`), refusing typed on an undeclared name, on a `Count` and on a
unit that does not measure the declared dimension. PR 2732, branch
`edit/doc-param-unit`.

Evidence added rather than a second row opened: this file already owns
the panel half. Still `parked` — the blocker is `review`, not closed,
and `work/README.md` reads a fired trigger off a CLOSED blocker. Once
it closes, the unit half here is unblocked and the `SessionOp` mirroring
`SetSlotUnit` is buildable: the kernel edit takes a `UnitSym`, so
`props`' `slot_unit_edit` shape transfers to a parameter row without the
rebuild-from-parts problem that shape had before the door existed.

## Unparked (2026-09-16, EDIT orchestrator)

The trigger fired: `doc-param-unit-edit-has-no-door` closed with PR
#2732, which built `DocEdit::SetDocParamUnit` and
`DocParam::with_display_unit` — the door this row's panel half waits
on. It is open work on CHROME's slate now; the API the panel consumes
is named in that item's `## Built`. Header edited from outside CHROME's
fence only to keep the tracker true (`work/README.md`: a fired trigger
is not a blocker).

## Dispatched 2026-09-21 — AUTH-2, with the create door riding along

`docs/AUTH-2-SPEC.md`, branch `author/param-notation`. This row and
`add-parameter-form-authors-canonical-only` went out as ONE unit
because this file already says why: *"the design call about how
notation crosses `props` is shared"*, and settling it twice is how the
two doors drift. The parser is
`editor_core::parse::parse_expr` — the one that already reads `50 mm`
— rather than a second unit table; the unit-and-value edit is one
`commit_action`, so one submit is one undo.

## Closed 2026-09-21 — PR 2957 merged (`8352822c2`)

**A parameter is written in the unit a person thinks in, at both
doors.** `50 mm` typed into the row sets the value AND the notation as
ONE undo, through `commit_action`; a bare number leaves the notation
alone; an expression is refused with a sentence saying a document
parameter holds a number. The picker over `DocEdit::SetDocParamUnit`
is this crate's first consumer of the door PR 2732 built, and the
"A LABEL … because there is no edit for one to push" comment is gone
with the reason that made it true.

**The no-op guard took two passes and a re-review to get right**, and
the shape of that is the part worth carrying forward.

1. The first pass gave BOTH fields one guard comparing numbers within
   `readout::REL_TOLERANCE`. It was correct as a pure function and
   **dead code at the panel** — `drag_ops` emitted above it — so this
   row's own defect survived; and on the SLOT row, where it was the
   sole emitter, it silently discarded real edits up to 5·10⁻⁴ of the
   value (±0.5 mm on a 1 m slot). NOT-MERGEABLE, two MAJORs, both
   measured by running rather than read.
2. The guard was re-decided to compare **TEXT**: `props::echoed` asks
   only *did this come out of the field itself*, needs no tolerance,
   and `readout.rs` went back byte-identical to `main`. That pass then
   found why a text guard alone cannot hold the invariant: **`egui`
   hands a field's buffered text over on two consecutive frames** and
   `Response::lost_focus()` is true on both, so one typed number
   emitted two identical ops. The numeric guard had been masking it by
   accident.
3. So there are TWO rules, each with one home:
   `props::echoed` at the field over text, and
   `DocSession::writes_nothing` at the session door over the
   `DocEdit`, asked after each door's own refusals so a driven slot
   keeps its affordance. The second is exhaustive over all 21
   `DocEdit` variants — a value-writing variant added later is a
   compile error at the decision, not a silent inheritance of
   "submit".
4. A targeted re-review of that NEW design (neither earlier lane had
   seen `writes_nothing`) found the fix pass had deleted the slot
   row's `row.source != text` check claiming the echo guard subsumed
   it. **It did not**: `set_slot_expression` called `commit` rather
   than `commit_written`, and the field guard compares against the
   RENDER, which for a literal slot is the bare number while the
   source is the unit-bearing spelling. Re-typing `8 mm` over a field
   showing `8` cost **two** undo steps — worse than the review's own
   measurement, because both hand-overs committed. One word fixed it.

**Net against `main`, stated plainly**: the driven case is a GAIN, not
a restoration — main's guard was a string comparison against
`row.source`, so `base_r*2.0` over a slot driven by `base_r * 2.0`
cost an undo step there and costs none now.

**Residue, scheduled**: `work/vgeom/a-typed-field-hands-its-text-over-on-two-frames.md`
(new, and recording `egui::Memory::had_focus_last_frame` as the
field-level route considered and declined);
`work/vgeom/a-fields-text-commits-within-the-renders-own-tolerance.md`
(the generalisation to every field, corrected to describe the text
guard); `work/chrome/bounds-reading-respells-the-panels-one-divide`
and `work/forms/dimension-to-unit-ladders-have-six-homes-in-the-viewer`.
The harness cannot drive refusal paths — stated in a comment on
`Row::frame` rather than papered over.

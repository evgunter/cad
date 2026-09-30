# AUTH-2 — a parameter's notation, at both doors (spec)

Unit of the `author` program. Two items, taken together because the
tracker says to: `work/author/parameter-row-field-has-no-text-door.md`
(the EDIT door, D) and
`work/author/add-parameter-form-authors-canonical-only.md` (the CREATE
door, E). Read both in full. The first says of the second: *"Whoever
takes either should read the other — the design call about how notation
crosses `props` is shared."* That shared design call is this unit, and
splitting it would settle it twice.

**Track:** a chrome build. No ratified clause is touched and no kernel
change is needed — `DocEdit::SetDocParamUnit` landed at PR 2732 and
`DocParam::written_length` / `written_angle` have always been there —
so nothing here waits on Ev. One implementer, then two reviewers
(§Review). No A/B row.

Branch `author/param-notation`, off `main`.

## The gap, in one sentence

A parameter is the only value in the viewer a person cannot write in
the unit they think in: the CREATE form mints it in the canonical unit
with no way to say otherwise, and the EDIT row is a bare `DragValue`
whose only parser is egui's own — so `50 mm` is not a thing a person
can type at either end, and a click-in/click-away can still cost an
undo step.

## What the tree says now

1. **The slot field is the shape to copy, and it is right next door.**
   `ViewerBehavior::slot_value_ui`
   (`crates/viewer/src/pane/properties.rs`) has a `.custom_parser(…)`
   routing typed text through `props::field_edit`, and a
   `match typed.into_inner()` arm under the comment *"Text that says
   what the slot already says is not an edit"* — the no-op guard. The
   parameter row is the `Selection::Param(name)` arm of
   `ViewerBehavior::properties_ui`, in the same file, and has neither.

2. **A document parameter holds a number, not an expression.**
   `DocParam::Continuous` holds an `f64` (`crates/editor-core/src/doc.rs`).
   There is no `SetDocParamExpression` and no expression a parameter
   can be set to, so the slot field's second door does not carry over:
   `base_r * 2` typed into a parameter field is a REFUSAL. The item's
   own words — *"and saying so is itself an affordance the field
   currently does not offer"*.

3. **The unit half's blocker is built.** `DocEdit::SetDocParamUnit
   { name, unit }` (`crates/editor-core/src/edit.rs`) refuses typed on
   an undeclared name, on a `Count`, and on a unit that does not measure
   the declared dimension. **No viewer `SessionOp` reaches it** — grep
   `SetDocParamUnit` across `crates/`: every hit is in `editor-core`.

4. **The row's unit is a LABEL, and the comment says why.** The
   `Selection::Param` arm renders `ui.weak(unit.symbol())` under a
   comment reading *"A LABEL, where a slot row has a picker … because
   there is no edit for one to push"*, citing
   `work/issues/doc-param-unit-edit-has-no-door.md`. **That reason has
   expired** (item 3) and the citation is doubly stale — the item moved
   to `work/edit/` and then closed. Correcting that comment is this
   unit's, not a drive-by: it is the code's own record of a constraint
   that is gone.

5. **The create form authors canonical only.**
   `ViewerBehavior::add_param_ui` holds `new_param_dimension` and
   `new_param_value` and no unit; `props::doc_param` routes every
   continuous value to `DocParam::continuous`, whose `display_unit` is
   `UnitSym::canonical_for(dim)`. `DocParam::written_length` /
   `written_angle` are total doors that take a `WrittenLength` /
   `WrittenAngle` and produce a declaration whose unit measures its
   dimension by construction. The form's tick already derives from
   `FieldWriting::of(dimension, None)` and its comment says *"The form
   authors in the canonical unit"* — another sentence this unit falsifies
   and therefore owns.

6. **A unit-bearing number already has ONE parser.**
   `editor_core::parse::parse_expr` reads `50 mm` into an
   `Expr::literal_with_unit`, deriving the dimension from
   `UnitSym::measures` and refusing an unknown symbol with
   `ParseError::UnknownUnit` naming the token and its byte offset.
   This is the parser the slot field's expression door already uses.

## What the unit builds

**A. The edit door — the parameter row's field.**

Three things, in `crates/viewer/src/pane/properties.rs`'s
`Selection::Param` arm and the `props` doors under it:

1. **A parser.** Typed text reaches a door instead of egui's `f64`
   parse. **Route it through `editor_core::parse::parse_expr`, NOT a
   new unit table.** That is the whole architectural instruction here:
   a second place that maps `"mm"` to a factor is the duplication
   `docs/prompts/reviewer-style-lane.md` Q1 exists to catch, and the
   unit vocabulary, its refusal wording and its byte offsets are all
   already answered once.

   The three outcomes a parameter can have, and what each does:

   - **a bare number** — `SetDocParamValue` in the unit the field is
     written in, exactly as today, notation untouched;
   - **a number with a unit** (`50 mm`) — the value AND the notation,
     which is `SetDocParamValue` plus `SetDocParamUnit`. **One user
     action is one undo**, so these go through `DocSession::commit_action`
     (`crates/viewer/src/session.rs`), which applies a `Vec<DocEdit>`
     as one history state and is all-or-nothing. Do not emit two ops
     and hope.
   - **anything else** — a REFUSAL with a sentence, per item 2. It says
     that a document parameter holds a number and not an expression;
     the `Refusal` vocabulary in `crates/viewer/src/session/` is where
     the wording lives (`Refusal::exists_wording` and
     `Refusal::offer_wording` are the neighbours to match). A
     `ParseError` from `parse_expr` — an unknown unit, a malformed
     number — carries its own sentence and should NOT be re-worded
     here; carry it.

   **Decide and say in the PR** whether a unit whose dimension differs
   from the parameter's declared one (`50 mm` on an angle) is refused
   at the chrome or left to `SetDocParamUnit`'s own typed refusal. Both
   are defensible; the kernel refusal is the safety either way, so the
   question is only what the person reads.

2. **The no-op guard.** The item is precise about the stake: since
   VIEW's drag-field work a click-in/click-away is no longer
   destructive, but the render is accepted within
   `crate::readout::REL_TOLERANCE`, so the round trip can still move a
   parameter by up to 5·10⁻⁴ of itself and cost an undo step for a
   click nobody meant as one. Copy the slot field's guard and its
   reasoning, not its code, if the two differ —
   **but look for the shared home first.** Two fields with one
   "text that says what the field already says is not an edit" rule is
   exactly Q1's shape, and the honest answer may be one function both
   call. Say in the PR which you did and why.

3. **The label becomes a picker.** Item 4: the comment's stated reason
   is gone. The slot row's unit picker (`slot_unit_ui`, emitting
   `SessionOp::SetSlotUnit`) is the shape; the parameter's emits a new
   op over `DocEdit::SetDocParamUnit`. A `Count` and the dimensionless
   row still have nothing to say and keep saying nothing. **Rewrite
   that comment to state the invariant that now holds** — implementer
   discipline §4: no archaeology about the label it used to be.

**B. The create door — `add_param_ui`.**

The form offers the unit beside the dimension, and mints through
`DocParam::written_length` / `written_angle` instead of
`DocParam::continuous`. `props::doc_param` is the function that routes
it and is where the change belongs; its doc-comment names the canonical
behaviour and is this unit's to correct.

Reuse the form vocabulary that exists — `length_picker`, `angle_picker`,
`Drafts::length_unit` / `angle_unit`, `FieldWriting::of` for the tick.
**A `Count` has no unit and a `Scalar`'s symbol is the empty string**;
the form offers a picker only where there is something to pick, the way
the parameter row's label already filters on
`!u.symbol().is_empty()`.

Keep the tick honest: it derives from the dimension today, and once the
unit is authorable it derives from the dimension AND the unit, which is
what `FieldWriting::of(dimension, unit)` already answers for every
other field in the crate.

## Out of scope

The extrude probe row
(`work/author/a-negative-extrude-distance-probes-as-valid.md`) is
`crates/viewer/src/bounds.rs` and `crates/viewer/tests/valid_range.rs`,
not these files. Leave it.

## Claims to falsify (for the reviewers, and for you first)

- **C1.** `50 mm` typed into a parameter row declared in metres sets
  both the value and the notation, as ONE undo step, and reads back as
  `50` beside `mm`.
- **C2.** A bare number typed into a parameter row leaves the notation
  exactly as it was.
- **C3.** `base_r * 2` typed into a parameter row is refused with a
  sentence saying a document parameter holds a number, and changes
  nothing.
- **C4.** An unknown unit symbol is refused with `parse_expr`'s own
  wording, naming the token — not a sentence re-written at this site.
- **C5.** Clicking into a parameter field and clicking away again
  emits no op at all, for every dimension, including one whose rendered
  text differs from the stored value within `readout::REL_TOLERANCE`.
- **C6.** The add-parameter form mints `base_r = 50 mm` as a
  declaration whose `display_unit` is `mm`, and the document reads back
  that way after a save/load round trip.
- **C7.** No second unit table, symbol map or factor ladder was added
  anywhere. The only parser of a unit-bearing number in the viewer is
  still `editor_core::parse`.
- **C8.** Every comment this unit falsified is rewritten to the
  invariant that now holds — the row's "A LABEL … because there is no
  edit for one to push", the form's "The form authors in the canonical
  unit", and `props::doc_param`'s doc — with no archaeology.

## Tests

`crates/viewer/tests/panel_edits.rs` and `panel_display.rs` are the
panel's rows; read how they drive the panel before inventing a harness.
`crates/viewer/tests/creation_ops.rs` is where a `SessionOp` replay row
lives. The ops are the testable surface — a headless row asserts which
`SessionOp`s a piece of typed text produces and what the document reads
back afterwards.

**Write assertions a bug could break** (discipline §2). C5 is the one
that needs care: a row that clicks in and out on a value whose render
is exact cannot go red on the tolerance case, which is the case the
item is actually about. Build the row on a parameter whose stored value
and rendered text differ.

## Review

**Two reviewers, no dual**, per `work/author/plan.md` §Review posture.
This unit's failure mode is a confident wrong answer — a parameter
silently re-declared in the wrong unit, or a value scaled twice — and
not a refusal.

- **Correctness lane**: C1–C8, to falsify. C7 and the double-scaling
  hazard behind C1 are the two to push hardest: `parse_expr` already
  multiplies by the unit factor (`value * unit.factor()`), so a
  chrome that multiplies again is the bug this unit is most likely to
  ship, and it is invisible in metres.
- **Style lane**: `docs/prompts/reviewer-style-lane.md` in full. Q1
  (two notation vocabularies, two no-op guards) and Q2 (the comments in
  C8 — `git blame` them against the code they defend) are the live ones.

## Seam

AUTH-1 (`author/face-frame-seat`) is live in the same crate and touches
`crates/viewer/src/drafts.rs`, which this unit also touches — additive
fields on the same struct. Everything else is disjoint: AUTH-1 is
`session/author.rs`, `forms.rs` and `pane/create.rs`; this unit is
`pane/properties.rs`, `props.rs`, `session/op.rs` and `session.rs`.
Merge `origin/main` before you land and resolve `drafts.rs` by keeping
both sets of fields. Announce the seam in your PR.

## Discipline

`docs/prompts/implementer-discipline.md` in full, by path, before you
start. Hosted CI is the verification of record. File anything you find
outside this fence on the owning program's slate, in this PR (§6).

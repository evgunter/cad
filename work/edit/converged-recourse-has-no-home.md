---
id: converged-recourse-has-no-home
kind: issue
title: the converged 'declare it first' recourse is two literals in two crates, held in step only by a test
status: closed
closed: 2026-09-20
pr: 2920
branch: edit/recourse-one-home
opened: 2026-09-05
---


Disclosed by the #1932 style review (finding 10), on the unit that
created the pairing.

## What happens

One mistake — a parameter name that does not exist — reaches two
doors: typing it into the value field goes to `DocEdit::SetDocParamValue`
and refuses `EditError::DocParamNotDeclared`; dragging its row is a
lookup and refuses `Refusal::NoSuchParam`. The two are deliberately
converged on the RECOURSE rather than on the sentence
(`crates/viewer/src/session/refuse.rs`, at the variant), so both end
with the clause `— declare it first`.

That clause is now written twice, in two crates, and nothing but a
test holds the copies in step. `refuse.rs` itself is the file that
argues against exactly this: `affordance`, `exists_wording` and
`offer_wording` each carry a doc-comment saying *"two independently
built copies is how the wording drifts"*, and each has one home.

## Why the unit did not give it one

The other copy is in `crates/editor-core`, and giving the pair one
home means that crate exporting the phrase — a `pub const`, or a
recourse accessor on `EditError`. That is API surface, and the
amendment which permitted the wording change is scoped to
`EditError`'s `Display` wording. Widening it to add an item was
refused on the spot rather than taken quietly.

What holds them meanwhile is
`crates/viewer/tests/panel_edits.rs::refusals_render_as_sentences`,
which asserts both renderings carry the clause; drop it on either side
and that row reds. A test is a tripwire and not a home, which is why
this is filed rather than closed.

## What resolving it looks like

Two shapes, and the first needs DOCM:

- editor-core exposes the recourse once — as a `pub const` beside
  `EditError`, or better as a method on the error (`recourse()`),
  which is a shape LIB has wanted for the bindings anyway — and the
  viewer's arm renders it rather than repeating it;
- or the convergence is dropped and the two routes go back to
  differing, which is the decision this unit argued against and would
  need a reason.

Ride it on whichever unit next touches `EditError`'s surface with an
authorisation wider than `Display` wording.

## Re-homed to EDIT, 2026-09-19

Filed on VIEW's slate, carried to VNEWS by the 2026-09-17 re-scope, and
moved here by VNEWS' orchestrator at its first dispatch (Ev, in-chat
2026-09-19, granting the move without a ruling; the general authority to
re-home a unit between tracks came with it).

**Why it could not be dispatched from VNEWS.** Both shapes above — a
`pub const` beside `EditError`, or a `recourse()` accessor on it — add
API surface to `crates/editor-core`. The authorisation VNEWS inherits
is Ev's amendment of 2026-09-04, scoped to `EditError`'s user-facing
`Display` **wording** (the `edit: ` prefix and the `{:?}`-quoted
payloads) and to nothing else. This row's own text records that
widening it "was refused on the spot rather than taken quietly", so the
row was correct about its own blocker and the tracker was not.

**Why EDIT.** `EditError` is declared at `crates/editor-core/src/edit.rs`
and `edit.rs` is EDIT's territory; the clause this row is about is
written there and again in the viewer's `session/refuse.rs`. EDIT is
DOCM's successor for exactly this surface, and this row's closing
instruction — *"ride it on whichever unit next touches `EditError`'s
surface with an authorisation wider than `Display` wording"* — names
EDIT's slate by description.

**What stays VNEWS'.** Only the viewer arm: once `editor-core` exposes
the recourse once, `crates/viewer/src/session/refuse.rs` renders it
rather than repeating the clause. That is a forward at one site and
needs no row of its own — it lands with whichever EDIT unit exposes
the recourse, as an announced crossing into `session/refuse.rs`
(VNEWS' and VSEAM's, written on both sides).

**The tripwire is still the only thing holding the copies in step**:
`crates/viewer/tests/panel_edits.rs::refusals_render_as_sentences`
asserts both renderings carry the clause. Verify it still does before
relying on it — that file is VDOC's, S-TCOST's and S-TINT's ground and
the row is three programs' from where you will be standing.

## Ruled and spec'd (2026-09-20, EDIT orchestrator) — E-class, wave 16, branch `edit/recourse-one-home`

**Ruling: the first shape, as a `pub const`.** The row's own analysis
carries it: the clause is one recourse over one fact reached by two
doors, and the discipline in `refuse.rs` ("two independently built
copies is how the wording drifts") is the project's. The authorisation
this row lacked was API surface on `crates/editor-core`; `edit.rs` is
EDIT's and this is EDIT's slate, so it is granted here. A `pub const`
rather than `recourse()`: the viewer's `Refusal::NoSuchParam` holds no
`EditError` value to ask, and a method answering one recourse for one
arm and `""` for every other would say "already told" of arms that
have no recourse at all (`finding.rs`'s own meaning of an empty
recourse). The accessor LIB wants is a census over every arm's
recourse — a different unit, not this row's.

1. `crates/editor-core/src/edit.rs` gains one `pub const` beside
   `EditError` carrying the clause (name it for the recourse, in the
   tree's vocabulary — grep `DECLARE`/`RECOURSE` consts first and
   match), with a doc saying what it is (the one recourse for a
   parameter name that does not exist, reached by `DocEdit`'s doors
   and by the viewer's lookup) and where the second reader is.
   `DocParamNotDeclared`'s `Display` renders through it; the arm's
   comment about the viewer convergence points at the const instead of
   re-telling the story.
2. `crates/viewer/src/session/refuse.rs`'s `NoSuchParam` arm renders
   the const (an announced crossing into VNEWS'/VSEAM's ground, one
   site, written on both sides per the row's `## Re-homed` section);
   the `refuse.rs:140` paragraph that describes the convergence points
   at the const.
3. `crates/viewer/tests/panel_edits.rs::refusals_render_as_sentences`
   stays, re-worded from tripwire to pin: both renderings carry the
   ONE const (assert against the const, not a literal), so the copy
   cannot come back without the row going red. The three
   `editor-core` rows that assert the literal (`edit_doc_param_unit`,
   `edit_doc_param_distribution`) assert the const too.
4. Territory: `edit.rs` (EDIT); `crates/viewer/src/session/refuse.rs`
   (VNEWS/VSEAM by announcement); `crates/{editor-core,viewer}/tests/*`
   (TCOST/TINT). E-class: green CI and the orchestrator's read; the
   PR body records the sweep (`declare it first`, every hit and its
   disposition) and the crossing.

## Built (2026-09-20) — PR #2920

`crates/editor-core/src/edit.rs` carries
`pub const UNDECLARED_PARAM_RECOURSE: &str = "declare it first"` beside
`EditError`, documenting the one recourse for a parameter name that does
not exist, why the two doors converge on the recourse rather than on the
sentence, and naming `crates/viewer/src/session/refuse.rs` as the second
reader. `EditError::DocParamNotDeclared`'s `Display` renders it and its
comment points at the const. The viewer's `Refusal::NoSuchParam` arm
renders it and the `NoSuchParam` doc paragraph cites it (the announced
crossing into VNEWS'/VSEAM's ground, one site).
`panel_edits::refusals_render_as_sentences` is now a pin: it binds the
const and asserts both renderings carry it. The three `editor-core` rows
(`edit_doc_param_unit` ×2, `edit_doc_param_distribution` ×1) assert the
const too, and `story_parametric`'s narrative comment names it instead
of quoting the clause.

Rendered text is byte-identical on both sides — a single-homing, not a
wording change.

Not done, and deliberately: the const is not lifted to `editor-core`'s
root and not carried through `pncad`'s façade. The viewer reads it
through its existing direct `editor-core` edge, the ruling `pncad`'s
crate docs state for a name the façade does not carry. The public-surface
question belongs to the recourse census accessor LIB wants, which the
ruling already separates from this row.

## Closed (2026-09-20, EDIT orchestrator) — E-class, merged on green CI and the orchestrator's read

`UNDECLARED_PARAM_RECOURSE` beside `EditError` (`edit.rs`) is the one
home of "declare it first"; `EditError::DocParamNotDeclared` and the
viewer's `Refusal::NoSuchParam` both render it, the viewer through a
direct `editor-core` edge (the ruling `pncad`'s crate docs state for a
name the façade does not carry — no root re-export, so no LIB census
moves), and `panel_edits::refusals_render_as_sentences` pins both
renderings against the const rather than a literal. Rendered text is
byte-identical on both sides; nothing re-baselined. The name follows
the tree's `<subject>_RECOURSE` shape. The recourse-census accessor
LIB wants stays a separate question. PR #2920.


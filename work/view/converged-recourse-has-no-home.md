---
id: converged-recourse-has-no-home
kind: issue
title: the converged 'declare it first' recourse is two literals in two crates, held in step only by a test
status: open
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

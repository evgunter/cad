---
id: viewer-has-no-door-to-accept-a-parts-updated-version
kind: issue
title: viewer: no door emits DocEdit::UpdateReference, so a repaired part's new version cannot be accepted from the GUI
status: closed
opened: 2026-09-29
priority: P3
cost: M
branch: author/accept-part-version
closed: 2026-09-30
---


## What

A part's repair moves its content pin, so every instance of it then
refuses with a pin mismatch whose recourse is to accept the updated
version: `pncad::workspace::PIN_MISMATCH_RECOURSE`
(`crates/pncad/src/workspace.rs`) names `DocEdit::UpdateReference`, or
`workspace::update_to_store` for every site at once.

The viewer has no door that emits either. No `SessionOp` builds a
`DocEdit::UpdateReference`; the one mention in the crate is the
no-op filter in `crates/viewer/src/session.rs` (the `writes_nothing`
match, `DocEdit::UpdateReference { .. }` among "every other edit
submits"), which never sees one. So the refusal a GUI user reads
names a recourse the GUI cannot take, and the user has to leave the
viewer to record it.

Found while building `edit/part-root-carried-refusal`: the part-root
failure's own recourse ("open the part and repair node N") deliberately
stops short of "then accept the updated version" for this reason
(`work/edit/part-root-failure-nests-a-whole-refusal-past-the-budget.md`,
"Put to Ev").

## What would close it

A door on the instance row (or the pin-mismatch refusal's line) that
records `DocEdit::UpdateReference` against the store's current pin,
beside the `Add part…` door (`crates/viewer/src/parts.rs`).

Dispatched 2026-09-30 as **AUTH-15** (`docs/AUTH-15-SPEC.md`, branch `author/accept-part-version`). Checked first: `DocEdit::UpdateReference`, `workspace::update_to_store` (one edit per site, typed refusal) and `PIN_MISMATCH_RECOURSE` all exist, the viewer emits neither, and `docio::DirResolver::workspace` gives the viewer a `Workspace`.

## Built (AUTH-15, `author/accept-part-version`)

- **The door.** `SessionOp::AcceptPartVersion { id }` names the part.
  The session calls `pncad::workspace::update_to_store` over the
  directory the resolver consults (`DocSession::read_store`, through
  `DirResolver::workspace`; `add_instance` shares it). It commits the
  answered edits with `commit_action`: one action, one undo. The viewer
  builds no `DocEdit::UpdateReference` of its own.
- **The offer.** `frame::version_offer` reads `session::VersionOffer`
  off an instance's own `PartFault::Unresolved { fault: PinMismatch }`,
  beside `creation_offer` and `declare_offer`. `tree::rows` puts it on
  the row (`TreeRow::version_offer`). The Features pane draws
  `Refusal::version_question` and an **Accept updated version** button
  under the failure line. Every other failure, and every healthy row,
  offers nothing. While a run is outstanding (`busy()`),
  `DocSession::tree_rows` withholds every offer, so an accept whose own
  run has not landed is not offered again.
  - The button label is `VersionOffer::LABEL`, pinned by a row to the
    name `PIN_MISMATCH_RECOURSE` quotes.
  - It shows only after the assembly is (re)opened: see
    `work/offer/document-seam-no-in-session-change-detection.md`.
- **Calls.**
  - Every site of the part, not the one instance, because the kernel's
    door is that one.
  - The author sees the part's file on the row and in the question,
    with both pins in the store's own failure line above.
  - With nothing newer, or no file, the store's refusal is said as-is.
- Rows: `tests/instance_authoring.rs`
  (`a_pin_mismatched_instance_offers_the_accept_and_accepting_is_one_undo`,
  `accepting_with_no_newer_version_or_no_file_says_the_kernels_refusal`,
  `the_accept_reads_the_committed_document_before_its_run_lands`),
  `frame::tests::only_a_pin_that_no_longer_holds_offers_the_accept` (a
  census over every `PartFault` arm),
  `session::refuse::version_offer::the_accept_button_is_the_edit_the_pin_mismatch_recourse_quotes`,
  and `pane::features::tests::a_pin_mismatched_instance_row_draws_the_accept_and_its_button_is_the_offer`.
- The OFFER row `a-refusal-offers-no-action-in-the-viewer` carries this
  instance's evidence and the sweep's residue (the tolerance lever).

## Closed 2026-09-30 — PR 3591 merged (`f88b6438`)

**A pin-mismatched instance offers "Accept updated version".** Accepting records `workspace::update_to_store`'s edits, one per site whose pin moves, as one action and one undo, computed against the committed document. The offer is read off the landed run and withheld while a run is outstanding. The kernel's own refusals (nothing newer, or the file gone) are said as-is, and nothing commits.

**It is reachable only after the assembly is reopened.** Results are cached per reference, so a part repaired during a live session shows no mismatch until then. That is OFFER's `document-seam-no-in-session-change-detection`, which now carries the evidence.

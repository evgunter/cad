---
id: the-part-chooser-keeps-its-snapshot-across-a-document-swap
kind: issue
title: The Add part chooser's entries keep the refusals scanned for the document it opened on, and a swap leaves them live over add_instance's refusals
status: open
opened: 2026-09-28
priority: P3
cost: E
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`
(VNEWS), at merge base `f4e9aa68b`.

## What happens

The Add part chooser's entries (`part_entry`,
`crates/viewer/src/pane/create.rs` ~:256, through `refusable_button`)
are gated on `PartEntry::refusal()`, which is `Refusal::SelfInstance`
computed from the `parts::PartCensus` taken when the chooser opened or
was rescanned (`PartChooser::opened`, `crates/viewer/src/parts.rs`). A
click pushes `SessionOp::AddInstance { id }` (create.rs ~:704) to
`DocSession::add_instance`, which re-checks, against the session as it
now stands:

- `Refusal::self_instance(committed_doc().id(), id)`;
- a resolver (`Refusal::NoDocumentDirectory`);
- `workspace().current_pin(id)` (`Refusal::Workspace`).

`ViewerApp::part_chooser` (`crates/viewer/src/app.rs`, field ~:445) is
written only at construction and by the pane. Nothing resets it on
`Open` or `NewDocument`. So, with the chooser window open:

- File > New: every entry stays live, and the door answers
  `NoDocumentDirectory`;
- Open a sibling B: B's own entry stays live, and the door answers
  `SelfInstance`;
- a sibling removed since the scan: `Workspace(..)`.

The `Workspace` arm for a file whose header scans but whose full load
fails is late (only the load finds it) and is not part of this row.

## Fix

Drop or rescan the chooser when the document it was scanned for
changes. `PartCensus` already carries the directory it read, so
comparing that directory and the open document's id against the
session's current ones is enough. The census's reachability assumption
is that New and Open can be reached while the non-modal chooser window
is open; confirm that first.

## 2026-10-31 — the door now exists

`viewerapp-document-derived-state-has-no-boundary` put the viewer's
reset for a replaced document in one place, `ViewerApp::document_replaced`
(`crates/viewer/src/app.rs`), reached for exactly the operations
`SessionOp::replaces_the_document` names. Its sweep left `part_chooser`
outside on purpose: dropping the chooser is one answer and rescanning it
against the new directory and id is another, and that choice is this
row's. Whichever it is, it belongs in that door — not in a second match
on `Open` / `NewDocument`.

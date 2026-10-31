---
id: new-document-derives-its-id-from-the-typed-name
kind: issue
title: NewDocument derives the document id from the typed name, so two new documents with one name are one part
status: closed
opened: 2026-10-31
closed: 2026-10-31
priority: P1
cost: E
refs: [session-save-is-two-acts]
pr: 6145
branch: vseam/a-new-document-mints-a-random-id
---

Found by the designer pair weighing `session-save-is-two-acts`
(`[ev]` PR 6123). (VSEAM orchestrator)

`DocSession::new_document` (`crates/viewer/src/session.rs`) builds the
fresh document with `Doc::empty_derived(name, tol)`, so its
`DocumentId` is derived from the typed name. Two documents each
created as "bracket" are therefore ONE part to every store. Once Save
goes through `Workspace::save_at`, saving the second into a folder
that holds the first at `bracket.pncad` is a resave: identity matches,
the file name matches, and an unrelated drawing is overwritten with
no refusal. Today `docio::save_path` overwrites it too, behind only the
OS chooser's "replace file?".

The derive-from-name rule was an orchestrator's decision at GAUTH's
opening. Ev never reviewed it
(`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`,
whose census lists the `SessionOp::NewDocument` "identity ruling" as
FALSE). Both designers recommend a random id. The store's module doc
already says interactive authoring should mint one. The open question
is whether anything relies on a name-derived id: a test fixture, a
demo, or the tour. Check that before switching.

## 2026-10-31 — what the store route makes of it

Since PR 6135, Save goes through `Workspace::save_at`. A second
"bracket" saved over the first's `bracket.pncad` is a resave of the same
id: the store allows it, so the first drawing is replaced with no
refusal. Before, `docio::save_path` overwrote it too, but only after the
OS chooser's "replace file?" prompt. Now a typed `bracket` gains
`.pncad` after the dialog returns, so the dialog never saw a name that
existed, and the prompt is skipped. A file holding a DIFFERENT id now
refuses `SaveWouldReplaceDocument`. Only this same-id case stays silent,
and a random id would close it.

## Closed (2026-10-31): a new document mints a random id

Decided by the VSEAM orchestrator (2026-10-31): a new document gets a
fresh random id, as a fork does. `DocSession::new_document` now builds
`Doc::empty(pncad::workspace::random_document_id()?, tol)`. A refused
entropy source answers `Refusal::Workspace(RandomnessUnavailable)`, and
both refusals come before anything is replaced. `Doc::empty_derived`
is unchanged, and so are its other callers.

What relied on a name-derived id through this door: only
`crates/viewer/tests/creation_ops.rs`. The ring stream compared
`bit_eq` against the gallery fixture, which carries the demo's derived
id; it now compares under the fixture's id (`Doc::under_identity`).
The new-document row asserted `DocumentId::derive("fresh-part")` and
the trim-gives-one-id rule; it now asserts the id is not the derived
one and a second document of one name gets another id. The bracket
row's derive assertion went. The demos, the story tests and the
other `Doc::empty_derived` callers build their documents directly and
never pass through `NewDocument`.

The rows that go red without the fix are in
`crates/viewer/tests/save_and_fork_doors.rs`
(`a_second_new_document_of_one_name_refuses_to_save_over_the_first`)
and the new-document row in `creation_ops.rs`.

The name now decides nothing past its blank check. That is filed as
`a-new-documents-typed-name-is-kept-nowhere`. The sweep found the
same case at the startup door, whose plate carries one derived id in
every launch. That is filed as
`the-startup-document-has-one-id-in-every-launch`.

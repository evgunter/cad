---
id: new-document-derives-its-id-from-the-typed-name
kind: issue
title: NewDocument derives the document id from the typed name, so two new documents with one name are one part
status: open
opened: 2026-10-31
priority: P1
cost: E
design: true
refs: [session-save-is-two-acts]
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

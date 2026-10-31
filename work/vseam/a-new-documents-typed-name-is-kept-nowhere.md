---
id: a-new-documents-typed-name-is-kept-nowhere
kind: issue
title: a new document's typed name is kept nowhere once its id stops deriving from it
status: open
opened: 2026-10-31
priority: P3
cost: E
refs: [new-document-derives-its-id-from-the-typed-name]
---


Found by the unit that gave `NewDocument` a random id
(`new-document-derives-its-id-from-the-typed-name`). (VSEAM implementer)

The New… form asks for a name, and `Refusal::new_document_name`
(`crates/viewer/src/session/refuse.rs`) refuses a blank one. Past that
check `DocSession::new_document` (`crates/viewer/src/session.rs`) uses
the name for nothing. It used to be the seed of the id; now the id is
`pncad::workspace::random_document_id()`. `Doc` carries no name, the
session keeps only `path`, the toolbar label is
`app::document_name(session.path())`, which reads "untitled" until the
first save, and `ViewerApp::file_dialog`'s Save As pre-fills a file
name only from that path. So a document created as "bracket" shows as
"untitled", and the save dialog offers no name.

The form either should keep the name, as the unsaved document's
label and the Save As default file name, or should stop asking for
one. Which one is the open question. Keeping it is a session field
plus the two chrome readers. Dropping it removes `EmptyName` and the
form's field.

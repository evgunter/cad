---
id: the-tour-gallery-writes-its-documents-around-the-store
kind: issue
title: the tour's gallery writes each standalone document with persist save and fs write, around the store
status: open
opened: 2026-10-31
priority: P3
cost: E
---


Found by the sweep of `work/vseam/session-save-is-two-acts` (VSEAM),
which routes every viewer save through the store of the target's folder
(`Workspace::save_at`). No program's `paths` cover `demos/tour`, hence
here.

`demos/tour/src/gallery.rs`, `write_one`: each of the nine standalone
gallery documents is written as `pncad::document::save(doc, &[], tol)`
then `std::fs::write(dir/{name}.pncad)`. The folder's store never sees
the write, so nothing refuses two scenes that derive one `DocumentId`,
or a rerun into a folder already holding one of these ids under another
name; the viewer's own Save into that folder would then refuse
`DuplicateId` for every document in it. The assembly scene's documents
in the same mode already go through `Workspace::create` / `resave`
(`demos/tour/src/assembly.rs`).

The fix is the natural public spelling: `Workspace::open(dir)` once,
then `save_at(doc, "{name}.pncad", tol)` per document, which is what a
user writing a gallery would call (implementer-discipline §3, demos
through the public doors). It moves no frame; the bytes are the same.

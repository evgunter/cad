---
id: gallery-writes-every-document-scene
kind: unit
title: the document gallery writes impeller, plate and chain beside the five it writes today
status: open
opened: 2026-10-02
priority: P4
cost: E
---

## What

`demo-tour gallery` writes the document-authored scenes as `.pncad`
files the GUI opens. `impeller` (`Doc::empty_derived`), `plate` and
`chain` already build documents and are not written; add one
`gallery_document` each, and their rows to `gallery.rs`'s denotation
table (`each_gallery_document_denotes_its_scene_or_says_why_not`).
If `save`'s validation refuses one, that is the finding: record it in
the table's `why` and file it. Then open each in the viewer
(`cargo run -p viewer --features app` is not available headless; the
viewer's doc_io test path is) and say what loads.

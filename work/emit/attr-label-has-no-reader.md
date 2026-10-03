---
id: attr-label-has-no-reader
kind: issue
title: Attr::Label, a face's or body's label, is stored and carried and read by no surface
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [node-labels-are-document-data]
---

`Attr::Label` — a face's or body's label, an appearance attribute since M4 PR 7, and since PR 3713 the same validated `Label` text a node's label is (`crates/editor-core/src/appearance.rs`, `Attr`) — is stored, saved, pinned, carried by split and inline and repaired by `Rebind`, and no surface reads it: the viewer's scene reads `Attr::Color` only (`crates/viewer/src/theme.rs`), Python has no getter for an attribute's value, and no kernel sentence speaks a face by it. Found by the `node-labels-are-document-data` row (PR 3565's designers both noted it).

What a reader would be is a design question the node-label ruling did not settle: whether a picked face's header in the Properties pane says its label, whether a kernel sentence about a face (`a face of Extrude "base plate" (…)`) names the face's own label too, and whether Python reads attributes at all. Until one is chosen the attribute is write-only data.

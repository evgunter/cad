---
id: add-part-offers-the-open-document
kind: issue
title: Add part… lists the open document itself, and its add does nothing
status: open
opened: 2026-10-31
priority: P3
---

## Finding

Found driving the viewer (2026-10-31; Ev: P3, "not show the document or visibly refuse it"). In an assembly saved beside a part, the **Add part…** chooser lists every document in the directory, the open document included (`assembly.pncad` beside `plate.pncad`). Adding the plate works. Clicking **add** beside the open document does nothing: no node, no message, and the chooser stays open (`part_listing`, `crates/viewer/src/pane/create.rs`). The chooser should leave the open document out, or say why it cannot be added.

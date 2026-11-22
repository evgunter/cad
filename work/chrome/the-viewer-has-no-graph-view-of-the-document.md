---
id: the-viewer-has-no-graph-view-of-the-document
kind: issue
title: viewer: there is no graph view of the document's dependency DAG
status: open
opened: 2026-11-22
priority: P1
cost: X
points: 40
---

**Ev suggested this** (in chat, 2026-11-22): "this would be a bigger project, but a graph display for the parts?".

The viewer has no graph or DAG view (nothing in `crates/viewer/src`). The feature tree deliberately keeps every node row in one un-indented column (`crates/viewer/src/tree.rs:103-107`); only constraints nest under their placement (`tree.rs:50`, `:295`). `docs/GUI-DESIGN.md:22` and `:145-148` name the recipe DAG and result DAG but no view of them. The dependency data is `doc.upstream` (`crates/editor-core/src/doc.rs:2495`).

This needs a design pass before a lane builds it. Open questions: which DAG (recipe, result, or both); its home (a pane beside the tree or a mode of it); how it relates to the GUI-DESIGN tree ruling; layout for a deep recipe; and whether nodes in it select, as tree rows do. `points: 40` is a placeholder for a program-sized row, to be re-priced once the design settles.

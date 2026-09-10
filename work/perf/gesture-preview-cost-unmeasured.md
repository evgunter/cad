---
id: gesture-preview-cost-unmeasured
kind: issue
title: a slider drag submits one full index build per preview under restart-without-cancel - unmeasured
status: open
opened: 2026-09-10
---

## The finding

Not measured — filed so the class has a home. The GUI lane measured
the committed-edit path only. The gesture path
(`BeginGesture` / `PreviewGesture` / `CommitGesture`) submits an index
build per preview, and the index seam is restart-**without**-cancel
(`crates/viewer/src/evalseam.rs:79`: the index step has no token to
check between), so a slider drag that outruns the worker on a
million-triangle document plausibly costs several full index builds
before the picture settles. That is the worst case in the tree for
`index-rebuilds-every-root-on-every-edit`, and it needs a driver that
can drag an egui `DragValue` under Xvfb, or a headless harness that
replays a gesture through `DocSession`.

## What is owed

A measurement: builds submitted and builds completed per drag, and the
wall from the last preview to the settled picture, on `gallery_ring`.
If the incremental re-tessellation unit lands first, re-take it after.

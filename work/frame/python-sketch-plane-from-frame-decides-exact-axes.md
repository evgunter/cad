---
id: python-sketch-plane-from-frame-decides-exact-axes
kind: issue
title: pncad-py SketchPlane.from_frame decides two lengths on every call, exact axes included, with no turned spelling beside it
status: open
opened: 2026-10-21
priority: P3
cost: M
---


`crates/pncad-py/src/py/doc.rs`, `SketchPlane.from_frame(origin, u, v)`, decides two lengths under `sketch_plane_frame_norm` on every call, so a Python user with exact axes goes through a refusal path that cannot fire, and has no spelling of an axis-aligned plane as a written turn (`exact-frame-mints-cover-three-of-the-world-frames`, decided 2026-10-21: the written turn is the vocabulary for an exact frame). Found by the exact-frames designer pair. Parked on D′: the Python façade's placement and sketch-plane doors are being re-spelled there (the helpers ruling), and this row's answer is whatever spelling D′ gives an authored plane, extended to the turned form.

## Released (INTENT, 2026-10-26)

This row was parked on the stage-3 placement pivot (`a-placement-pins-by-written-values-without-a-mate`). All three of its PRs are merged: D′1 (PR 5592), D′2 (PR 5613) and D′3 (PR 5636, `29110056e8`). Re-check this row against main, then fix, narrow or close it. That is the owning track's call.

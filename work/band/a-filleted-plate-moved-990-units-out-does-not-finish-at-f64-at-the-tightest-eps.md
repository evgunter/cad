---
id: a-filleted-plate-moved-990-units-out-does-not-finish-at-f64-at-the-tightest-eps
kind: issue
title: A filleted rounded plate moved 990 units out on each axis is not a finished body at f64 at eps 1e-12: four edges' shared_row_touch reads 1.4e-12, inside the band
status: open
opened: 2026-11-23
priority: P3
cost: E
---


## Finding

From lane `embed/tangent-corner` (PR 7320, 2026-11-23). Not traced.

`crates/sweep/tests/self_intersection_proves_injectivity.rs`,
`one_uneven_plate_is_embedded_in_every_rigid_pose_and_its_axis_mirror`:
a `2 × 1.2 × 0.5` plate of corner radii `0.3, 0.26, 0.34, 0.28`, every
edge filleted at `0.1`, at `f64`. Moved by the translation
`(990, −990, 990)` and finished (`moved`, the file's helper), at
`CAD_TOLERANCE_EPS=1e-12` the finish refuses:

`NotFinished { findings: [EdgeCertification { edge: 11v1, error:
Escalated { check: Transversality, cause: Indeterminate { margin:
1.3738342963808244e-12, band: { zero: 1e-12, escalate: 1e-11 },
predicate: "shared_row_touch" } } }, …] }`, four edges (`11v1`,
`12v1` at `1.374e-12`; `15v1`, `16v1` at `1.332e-12`).

At the default ε and at `1e-6` the same move finishes and the body is
`Embedded`. A hundred units out it finishes at all three ε. The row
skips that one pose at the tightest ε and says so.

Beside it: `work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`
is the same predicate at `Interval`; whether this is that row at `f64`
with a thousand-unit lever is not known.

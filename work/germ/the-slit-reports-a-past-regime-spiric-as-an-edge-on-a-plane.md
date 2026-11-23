---
id: the-slit-reports-a-past-regime-spiric-as-an-edge-on-a-plane
kind: issue
title: The slit reports a spiric × plane pose refused for the spiric's regime as Coincide::EdgeOnPlane, a label that names a different refusal
status: open
opened: 2026-11-23
priority: P3
---


Filed by GERM's `germ/co-surface-table` lane (PR 7431), from that PR's
confirmation review at `7f72931e9d`.

## Finding

The slit's spiric × plane arm (`crates/topo/src/boolean/slit.rs:1444`)
maps every `PoseRefusal::InBand` from `geom_brep::curve_face::pose` to
`BooleanError::coincidence(Coincide::EdgeOnPlane, …)` (`:1454`). The
spiric arm of the pose also refuses `InBand { rung:
PoseRung::SpiricRegime }` when the spiric is outside its two-oval
convention or in band of it. That refusal is about the curve's regime,
not about an edge lying on the plane, so the label misnames it. The
`diag` it carries does name the regime margin
(`curve_face_pose_spiric_off_axis` / `_two_ovals`).

## Settles when

The arm reads the rung and reports a regime refusal under a label that
says so: an existing regime coincidence, or a new `Coincide` variant
if none fits. A row must pin the label for a past-regime spiric.

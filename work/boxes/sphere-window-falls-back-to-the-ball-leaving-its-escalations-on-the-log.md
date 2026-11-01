---
id: sphere-window-falls-back-to-the-ball-leaving-its-escalations-on-the-log
kind: issue
title: sphere_window answers the ball on an undecided read and leaves that read's escalation on the log
status: open
opened: 2026-10-14
---

Found by PR 5140's fix pass, in the blind-spot sweep after it moved
`face_box_rule`'s cylinder fallback to `k_stats::detached` +
`splice_superseded`.

`crates/topo/src/boolean/boxes.rs` `sphere_window` (~:1700–1785)
answers a definite, looser box (`SphereWindow::Ball`, or `Zone` in place
of `Rect`) whenever one of its band reads does not decide. Each such
read records its escalation on the open `k_stats` frame first, and
nothing splices it superseded:

- `geom_brep::props::boundary_material_sign(..).ok()` (~:1739) → `Ball`;
- `sphere_chart_trim` `Err(PointInSolidError::Escalated { .. })` (~:1748) → `Ball`;
- `UnitVec3::new(*axis, BOX_SPHERE_AXIS, band)` refused (~:1752) → `Ball`;
- `UnitVec3::new(*u_ref, BOX_SPHERE_SEAM_UNIT, band).ok()` and the
  `BOX_SPHERE_SEAM` decide matched as `Ok(Sign::Zero)` only (~:1760) →
  `Zone`.

The box rests on none of those escalations: the fallback is the whole
ball, or the zone, both sound with no decision behind them. The
subdivision driver reads an escalation on a node's log as the node's
cause (`k_stats::splice_superseded` docs), so a node that later fails for
another reason would be refined, or named a sliver, by one of these.
The census lane's mirror (`census::face_reach_in` → `sphere_reach`)
shares the walk.

Fix shape: run each read under `detached` and splice it superseded on the
fallback arm, normally where its answer is used (as `face_box_rule`'s
cylinder arm does). Worth checking `torus_chart_window` (boxes.rs and
census.rs) for the same shape in the same pass; this sweep did not.

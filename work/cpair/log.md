# CPAIR — the log

## 2026-10-18 — opened

Opened by FUSE's close (`docs/doc-ledger/fuse-leaves-the-tracker.md`)
for the curve-pair reader's residue: nine rows moved from `work/fuse/`
by `git mv`, ids kept (their `work/fuse/` citations re-pointed). Nothing dispatched; no
orchestrator yet.

## 2026-10-18 — RESTREAD lands a router change in `pose.rs`

Branch `restread/meet-apart-on-one-line` (RESTREAD's
`census-meet-reads-two-segments-on-one-line-two-metres-apart-as-coincident-lines`)
changes `geom_brep::curve_pair::pose`'s router: after the parallel
argument, the gap between the two spans' projections on the line is
read (`curve_pair_pose_gap`), and a gap past the band is `Pose::Apart`.
`overlap` now returns the signed length, and `Route` has an `Apart`
arm. No CPAIR row covered it (the task named FUSE's
`curve-pair-meetings-are-runs`, which no longer exists, and
`curve-pair-polish-meters-refuse-shallow-meetings`, which is about the
polish meters, untouched). Its sweep filed
`the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc` here.

## 2026-10-20 — NURBS build A takes the polish meters

NURBS build A (dispatched by the NURBS orchestrator) claims
`curve-pair-polish-meters-refuse-shallow-meetings`: the basin meter in
`geom_brep::curve_pair::pose` (`Polish::closest`,
`curve_pair_pose_basin`) on shallow meetings with no found meeting
beside them, the needle at ε 1e-12 its first witness. No CPAIR or BAND
branch touches `crates/geom-brep/src/curve_pair/` (checked against every
live remote branch). Plan before code, at the NURBS orchestrator.

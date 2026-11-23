---
id: a-spline-face-has-no-inside-read
kind: issue
title: The session box has no point on a spline face's interior, so a spline body past the box only there is refused as a limit
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by the r1 review of PR 7225 (MINOR 2) and filed by that PR's lane.

**What.** `topo::cover::face_extreme_points` names no candidate on a spline face: its critical points have no closed form. So a spline body whose only point past the session box is interior to a spline face is refused as the limit `SessionBoxUnshown`, not as past the box, against `crates/geom-core/README.md`'s "one with a point past it is refused as past the box". Sound (refused either way), but the refusal's sentence must not assert the body may be inside.

**The shape of a fix.** Candidates where the face's net is extreme along each axis, each projected onto the face (`spline_region::spline_face_placement` places a point and reports its in-trim foot), kept where placed on the face; or the net's extreme control points' feet. Either gives points on the face from below.

**Why P2.** With no point read on a spline face, a body past the box by a spline face alone is refused as a limit rather than as past it. That is the right body refused for the wrong reason, and its recourse (move the geometry) is the same. No valid body's verdict moves, so this is the message's precision, not the box's.

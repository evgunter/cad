---
id: nurbs-face-meeting-a-plane-in-an-interior-loop-is-unguarded-on-the-crossings-path
kind: issue
title: A NURBS face meeting a plane face in a loop interior to both, while crossings exist elsewhere, is seen by nothing on the crossings path
status: open
opened: 2026-09-28
priority: P1
cost: M
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

The section certificate (PR 3372) examines pairs in which one face is a
torus, sphere, cylinder or cone; a NURBS face paired with one of those
refuses on reach. A NURBS face paired with a PLANE is outside its scope
on the crossings path: nothing there asks whether the two meet in a loop
interior to both faces while crossings exist elsewhere, and the plane ×
NURBS arm is on the union's roster (`reduce.rs` `boolean_arm_exists`).
On the no-crossings path the extent scan's NURBS re-gate refuses any
fallback entry with a NURBS face (`ops.rs` `sphere_extent_scan`,
`NurbsExtentUnsupported`), so only the crossings path is open.

Unmeasured: build a NURBS bump a plate grazes in an oval while an edge
crosses elsewhere, and run ∪. If it answers, it is the interior-loop
class's last open member; the cheap guard is to refuse the pair on reach
at the certificate, as the spline × curved pairs already do.

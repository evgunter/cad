---
id: pinch-tessellate-row-escalates-coincidence-at-eps-1e-6
kind: issue
title: sweep: pinch_faces_tessellate's notch307 fib117 'edge psi=1.9 cp S' pose builds no body at eps 1e-6 (Coincidence(Join, Moot) escalates), so the PR gate's 1e-6 row is red on main
status: open
opened: 2026-10-06
priority: P1
cost: E
refs: [a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id]
---

Found by BAND's per-shell blend lane, whose PR gate went red on the
1e-6 row of `sweep` in a test its diff does not touch.

`crates/sweep/tests/pinch_faces_tessellate.rs`,
`a_face_through_two_vertices_on_one_point_tessellates`, landed with
`a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id`
(PR 4074). At `CAD_TOLERANCE_EPS=1e-6` it fails on origin/main 15c135a4
by itself:

    notch307 fib117 edge psi=1.9 cp S: the op builds no body:
    Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
    margin: MarginDiag(Value(-5.196042020649827e-6, None)), ... } })

The margin `−5.2e-6` sits inside the 1e-6 row's band (`zero 1e-6`,
`escalate 1e-5`), so the pose's boolean escalates rather than building.
At the default ε and at 1e-12 the row passes. The row needs to say what
this pose does at a coarse ε: either loud-skip the in-band pose there
(`test_utils::loud_skip_marker!`), or accept a typed escalation for it.
Today it demands a body at every ε.


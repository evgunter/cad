---
id: no-suite-body-reaches-a-pole-junction-off-the-closing-side
kind: issue
title: no suite body reaches unwrap_tie at a pole junction: the walk's tie zone at a pole is unwitnessed
status: open
opened: 2026-11-21
priority: P3
cost: M
refs: [mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
---

Found by TESSLIE's lift-turns measurement (2026-11-21).

`mesh::walk::unwrap_tie` has an arm for a meridian column opening at a
POLE junction in a rim-anchored loop: there `prev_u` is the previous
column, not a rim's end azimuth, so the copy about it can stand half a
turn off and the tie zone, placed by `anchor`, decides a whole turn.
Instrumented over the mesh and sweep suites, `unwrap_tie` was called
21561 times and none of those calls was at a pole junction
(`pole_v(jid)` was `None` at every call). The rim-anchored pole junctions
the suites hold are the closing side's, whose column is `anchor` itself
and is not read through `unwrap_tie`.

So the arm is unwitnessed: nothing shows what it answers, or that the
stored lift agrees with it. A witness is a curved face whose rim-anchored
loop meets a pole between two meridians that are NOT its last iso side:
for example a sphere face bounded by a rim, a meridian up to the pole,
a second meridian down from it and a further meridian before the loop
closes (a rim-split lune), or a cone face with a rim and two struts
meeting at the apex followed by a third side. Build one through the
public doors, mesh it, and compare its column with
`Body::loop_lift`'s.



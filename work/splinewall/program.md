---
id: splinewall
kind: program
title: SPLINEWALL — the spline leg, and the exact rational NURBS wall of every sweep whose segment has no surface kind
status: ready
opened: 2026-11-22
group: kernel/sweep/spline
prefix: splinewall/
tag: (SPLINEWALL orchestrator)
paths: [crates/sweep/src/revolve/surfaces.rs, crates/sweep/src/extrude.rs, crates/sweep/src/loft.rs, crates/geom/src/surfaces/nurbs.rs, crates/step-import/tests/all.rs]
keep_out: [opened 2026-11-22 by PROFILE's cut of the ovoid build Ev ruled on PR 7289 (part 3) - parts 1 and 2 (the ellipse segment and the Spheroid kind) are OVOID's and its parent row is there - the two share the profile Segment enum and the revolve door and announce on each other's logs, every path above is shared, the spline lanes this track's bodies reach are SPLICE's and CHARTTAIL's (the closed chart's seam - the trim at a seam or a pole - the crease at a G1 arc joint - the collapsed boundary row) and a row there that blocks a unit here inherits its band, crates/profile/* and docs/PATHS-DESIGN.md are PROFILE's - the NURBS legs are specified there (VQ7) and announced there, docs/DESIGN.md is ratified design - a re-word forced by an approved change lands with it and a change to what a clause decides is an [ev] PR]
---

Part 3 of the build Ev ruled on PR 7289 (2026-11-21): the spline leg
`docs/PATHS-DESIGN.md` already specifies (the profile README's VQ7),
and the exact rational `Nurbs` wall for every sweep that has no kind:
an ellipse arc revolved off its axis or tilted, an ellipse arc
extruded (unless the build makes the elliptic cylinder a kind), and any
spline leg. A true egg, blunter at one end, is a spline meridian, so it
is here. The ruled sentence is `docs/DESIGN.md` `[surfaces]`: "a wall
swept from a profile segment with no kind of its own is the exact
rational `Nurbs` surface, never an `Approx`".

Nobody has measured what such a body does: no body with a NURBS wall
of revolution exists in the tree. The first unit is that measurement,
on hosted CI, and the slate after it is re-cut on its result.

Opened beside OVOID, which holds parts 1 and 2 and the parent row.
Charter and order: `work/splinewall/plan.md`; narrative in
`work/splinewall/log.md`.

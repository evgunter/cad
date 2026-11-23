---
id: ovoid
kind: program
title: OVOID — an ellipse arc in the profile, and the spheroid a revolve mints from it
status: ready
opened: 2026-11-22
group: kernel/geom/surfaces
prefix: ovoid/
tag: (OVOID orchestrator)
paths: [crates/geom/src/surfaces.rs, crates/geom/src/chart_point.rs, crates/geom-brep/src/chart_singularity.rs, crates/sweep/src/revolve/axis.rs, crates/sweep/src/revolve/surfaces.rs, crates/profile/src/lib.rs, crates/profile/src/path/verbs.rs, demos/tour/src/lily.rs]
keep_out: [opened 2026-11-22 by PROFILE's cut of the ovoid build Ev ruled on PR 7289 (parts 1 and 2) - the parent row came from profile by git mv with its id kept, part 3 (the spline leg and the exact NURBS wall of every sweep with no kind) is SPLINEWALL's - the two share the profile Segment enum and the revolve door and announce on each other's logs, every path above is shared - a new surface kind reaches every exhaustive match on Surface across geom geom-brep topo sweep mesh the step crates editor-core and pncad-py - so each unit runs scripts/work.py territory on its branch and announces on every program it crosses, crates/profile/* and docs/PATHS-DESIGN.md are PROFILE's and the ellipse segment is a vocabulary change there - announce on profile's log and on LIB's python programs (pydoor and bind), the ellipse pair rows are the planar meeting reader's (PROFILE's check-9-arm-5 build ruled on PR 7290) and this track reads them and writes none of its own, docs/DESIGN.md and the crate README pages in its companion table are ratified design - a re-word forced by an approved change lands with it and a change to what a clause decides is an [ev] PR, the persist schema is contended ground - announce on the away channel before landing]
---

The build Ev ruled on PR 7289 (2026-11-21: "this sounds good! no real
opinion about the lean in 3"), parts 1 and 2 of its three:

1. an ellipse-arc segment in the profile, its own segment kind, lifting
   to the existing `Curve3::Ellipse`, authored as one new leg
   `ellipse_to(Stretched { c, along, factor, winding, p })`;
2. a `Spheroid` surface kind, minted by the revolve where the ellipse
   arc is centred on its axis with a principal direction along it,
   with parity with the sphere at first.

The ruled text is `docs/DESIGN.md` `[surfaces]`. The parent row,
`an-ovoid-cannot-be-authored`, holds the corrected premise and the bar
the designers set (its "Weighed" section) and the cut (its "Cut"
section).

Cut out of PROFILE, which was over its 30-point budget before this
build was priced. The build is about 53 points, most of it a surface
kind, which is `geom` and everything above it rather than profile
ground. Part 3 went to SPLINEWALL because it rests on the spline lanes,
its size is unmeasured, and its blockers sit on SPLICE and CHARTTAIL;
on one track, this track's close would wait on them.

Charter and order: `work/ovoid/plan.md`; narrative in `work/ovoid/log.md`.

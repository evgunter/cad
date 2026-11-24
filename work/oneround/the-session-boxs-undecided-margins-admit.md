---
id: the-session-boxs-undecided-margins-admit
kind: issue
title: The session box admits on an undecided cover margin, and stage 2 decides on carrier data that can stand past the rounding reach
status: open
opened: 2026-11-21
priority: P3
cost: M
---

Found by both reviews of PR 7225 (`oneround/box-region`: r1 NOTE C, r2 NOTE 9 and NOTE 10) and filed by that PR's lane.

**An undecided box margin admits.** `topo::session_box`'s stages 1 and 2 admit on anything but a definitely negative margin, an escalation included, as main did on the points. The README says a region is in the box only where its cover is; an escalated cover margin is not that. At `f64` a finite body's margin is a value and never escalates, so it is unreached there. At `Interval` a cover margin whose enclosure straddles the band admits without the inside read running; neither reviewer nor the lane could build one (the window is the enclosure's width at the box's edge). `Sym` reads as `Interval` here.

**Stage 2 decides on data that can stand past the rounding reach.** Stage 2's decisions (a sphere's chart window and seam, a torus's ring test and chart walk, a cone's lifts, apex incidence and nappe) are levered at the face's own geometry, never at the session box's extent, and every datum they read lies in the face's stage-1 region. A verdict among them is sound where that region's reach is within the run's rounding reach (`Tolerance::rounding_reach`, at least the box). A cone's apex is a datum of that region and can stand far past the box while the face stands at the origin (a 1 m frustum tapered 1 mm over 10 m has its apex 10 km off). At ε = 1e-9 the rounding reach is about 5 000 km; at ε = 1e-12 it is about 5 km, so there such an apex is read past the premise. No row reaches a wrong verdict this way; the decisions involved (a vertex's distance from the apex) are far from their bands on every fixture measured.

**The shape of a fix**: an undecided cover margin refused as the limit rather than admitted; and stage 2 refusing as a limit (without its decisions) where the stage-1 reach passes the rounding reach. Both narrow what is admitted; measure what moves.

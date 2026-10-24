---
id: an-extrudes-cap-carrier-is-minted-by-newell-not-by-its-frame
kind: issue
title: An extrude's cap carrier is minted by Newell's method, not as the frame's plane D10 describes
status: open
opened: 2026-10-23
priority: P2
cost: M
---

Found by the designer pair on MSOLVE's `a-plane-poses-foot-outgrows-the-coincidence-doors-budget` (record on `analysis/design-fork/msolve-plane-foot-sym-budget`), and filed by its implementer on PR 5624.

D10 `[coincidence.structural]` (`docs/DESIGN.md`) says an extrude's end cap "comes out as `plane(frame, direction, depth)` with no profile variable in it". The code does not do that.
- An extrude's cap carrier is minted by Newell's method over the cap's placed loop: `sweep::swept::cap_plane` (`crates/sweep/src/swept.rs`) calls `geom_brep::newell_plane(&ordered, place.linear.c0, band)` and keeps that plane's origin and normal.
- Its callers are in `crates/sweep/src/extrude.rs`: the cap at `cap_plane(…)` and the top plane `cap_plane(&far_loop, …)`. A wall plane there is `newell_plane(&corners, …)` too.

So in the `Sym` replay the cap's normal is a normalised Newell sum over the profile's vertices. That is a function of every profile variable, and its `x` and `y` are computed forms rather than literal zeros, even for a profile on its own xy. The coincidence door's replay then reads a document normal the tier cannot reduce (see `work/sym/a-plane-poses-normal-read-off-a-face-is-opaque-to-the-unit-relation-at-sym.md`).

**Done when** the cap carrier is the frame's plane at the extrude's depth, as D10 describes, or D10's sentence is corrected to what the code does. The latter would be a change to ratified text, so it goes to Ev.

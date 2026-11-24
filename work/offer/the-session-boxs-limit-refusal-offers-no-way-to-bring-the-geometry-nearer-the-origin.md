---
id: the-session-boxs-limit-refusal-offers-no-way-to-bring-the-geometry-nearer-the-origin
kind: issue
title: The session box's limit refusal says 'place the geometry nearer the origin' and the viewer offers no way to do it
status: open
opened: 2026-11-24
priority: P1
cost: M
refs: [a-refusal-offers-no-action-in-the-viewer]
---


Asked for by Ev on PR 7225 (2026-11-24), approving the session box's new refusals: "this sounds good, might need some gui handling to make the recourse easy".

**The refusal.** With PR 7225 (`oneround/box-region`) the session box, the cube of every point within 1 km of the origin along each axis, asks whether the region a body could occupy, as the kernel can prove it, lies inside. A body is refused in two ways (`topo::session_box`):

- `OutsideSessionBox`: a point known to lie on the body is past the box.
- `SessionBoxUnshown { margin }`, a limit: "the kernel's bound on the body's faces reaches N m past the 1e3 m session box … the body may lie inside the box or past it". The body may be valid and inside. It happens where a face's proven region overshoots the face: a ball a boolean has cut (or a pole-to-pole wedge) within its diameter of a box face; a spindle face (a lemon, an apple) within its tube's span; a torus face a boolean has cut; a spline face within its control net's standoff. PR 7225's body has the table per face kind. Of 391 sampled placements truly inside the box and within 40 m of a face, 85 are refused so.

Both carry one recourse: place the geometry nearer the origin.

**What is missing.** The viewer shows the sentence and offers nothing. The person has to work out which node placed the body out there, by how much to bring it in (the refusal's own margin says: at least N m, along which axis the sentence does not say), and edit that placement by hand. Where the body is a copy or a pattern member, the placement to edit is not the node the refusal names.

**What an answer has to settle** (a `design` question may be in it; this row does not weigh it): whether the kernel's refusal hands over the edit as data, as `a-refusal-offers-no-action-in-the-viewer` proposes for recourses that fully determine an edit (this one does not fully determine it: the direction and the amount are a choice); what the refusal must carry for a GUI to offer it (the axis and the overshoot per face, which `SessionBoxUnshown` holds as one margin today); and whether the offer is "move this placement by the overshoot along that axis" or a pointer to the placement with the numbers.

P1 by `work/README.md` Priority (a GUI matter Ev raised; it does not block authoring).

---
id: support-boundary-meter-bounds-other-carriers-by-the-whole-carrier
kind: issue
title: blend: the rim support-boundary meter bounds an ellipse, spiric or NURBS edge over its whole carrier, which can over-refuse
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## Finding

`ring_clearance_pass`'s support-boundary walk
(`crates/sweep/src/blend/surgery.rs`) reads a line or circle edge over
its stored window exactly (`piece_distance`, `piece_along`), and any
other carrier through `whole_carrier_reach`: an ellipse's height range
over its whole period (exact for the whole ellipse), and otherwise a
ball holding the whole carrier (an ellipse's distance, a spiric oval's
torus, a NURBS curve's control points). Every such bound is sound —
it can refuse a carvable request, never pass a consumed one — but it
reads the whole carrier rather than the edge's window, and the ball
bounds on DISTANCE (a plane support) are loose by up to the carrier's
size. So an ellipse or NURBS edge on a plane support near a rim can
refuse `RingClearance` though the edge itself clears.

Measured reachable for the ellipse on a curved support: a 45°-tilted cut
across a cylinder wall
(`band_annulus_host_boundary::a_tilted_cut_reaching_a_cylinder_hosts_trim_between_samples_refuses`),
where the whole-ellipse height range is tight. No row measures an
over-refusal; a plane support with an ellipse or NURBS outer edge (a
STEP-imported spline plate with a bore, a tilted bore crossing a cap's
outer cycle) is where one would show.

## What the taker owes

A windowed reading for the ellipse (its height is a sinusoid of the
parameter, so the extremes are at the ends unless the window holds the
stationary parameter) — routed through the one window-membership home
`arc-window-membership-has-three-spellings` asks for, not a fourth
spelling — and a tighter distance bound for NURBS (the control hull's
own distance, or subdivision) if a plane-support row shows the ball
over-refusing.

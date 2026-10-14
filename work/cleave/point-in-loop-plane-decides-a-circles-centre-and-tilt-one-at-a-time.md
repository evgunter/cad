---
id: point-in-loop-plane-decides-a-circles-centre-and-tilt-one-at-a-time
kind: issue
title: point_in_loop_plane decides a conic's centre and its tilt off the plane one at a time, not their sum
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

`topo::splitting::containment`'s `certify_plane`
(`crates/topo/src/splitting/containment.rs:1946`) certifies that a loop
lies in its plane by deciding, per edge, the conic's centre off the plane
(`off(center)`) and its axis's tilt off the normal levered at its radius
(`tilt(axis, radius)`, `:1967`) as separate `point_in_loop_plane` rows
(`:1999`), each required Zero. A circle's points stand off the plane by
up to the centre's offset plus the tilt levered at the radius, so each
row just inside the band admits a loop whose rim stands nearly twice the
band off the plane, and `point_in_loop` then answers for it.

Found by the sweep of `section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time`
(branch `cleave/section-cert-axis-pose`).

## The shape of a fix

Decide one margin per conic edge, `|off(center)| + tilt·radius` (the
ellipse's and spiric's alike, with their own levers), on the Zero side
the gate serves.

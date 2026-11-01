---
id: point-in-loop-plane-decides-a-circles-centre-and-tilt-one-at-a-time
kind: issue
title: point_in_loop_plane decides a conic's centre and its tilt off the plane one at a time, not their sum
status: open
opened: 2026-10-11
priority: P2
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: centre offset and tilt decided as two rows: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

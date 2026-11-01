---
id: a-torus-meridian-circle-is-a-stood-in-section-read-on-its-own-plane
kind: issue
title: A torus's meridian circles stand in for a plane read through the axis, but a chord on one carries the plane's own reading (OnPlane)
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Found by the review of PR 4810 (MINOR-4) and measured there.

## What

`plane_torus_section` stands two meridian circles in for a plane it
reads through the torus's axis (`pt_axis_in_plane`, `pt_axis_plane_gap`
Zero), so the circles miss the plane by its tilt and offset read in
their zero bands. PR 4810 gives the concentric circles (a plane read
square to the axis, `pt_axis_normal`) the square reading, as a
cylinder's rim has. The meridian circles still carry `OnPlane`
(`chord_join::section_case`'s torus arm), whose lever names the
vertex's offset and not the plane's tilt or offset off the axis.

## Measured (PR 4810's census probe, family `torus`)

The ring torus `R = 2`, `r = 0.5` about `y`, against a block whose face
`z = d` (`d ∈ {0, ε, 2ε}`) is turned `t` about `x`
(`t ∈ {0, ε/5, 2ε/5, 4ε/5, 1e-6, 1e-2}`), every op and order, at ε =
1e-9, 1e-6 and 1e-12: no run reaches a meridian chord's pin. They
build, refuse at the join's own escalation (`Join(Escalated)`), at
`Coincidence`, or at `GermSectionSpiric`.

## Owed

If a pose reaches it: a reading of its own (the plane read through the
axis), its lever ("clearly through the axis or clearly off it"), and a
check that the lever builds.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEJOIN (`work/honejoin/`) by `git mv`, id and body unchanged: whose charter is the curved and chord join's residue. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

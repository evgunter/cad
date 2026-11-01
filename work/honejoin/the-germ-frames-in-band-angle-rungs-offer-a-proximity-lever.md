---
id: the-germ-frames-in-band-angle-rungs-offer-a-proximity-lever
kind: issue
title: The germ frame's in-band angle rungs of a plane against a quadric's axis offer a proximity lever
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [an-in-band-axis-parallelism-offers-a-proximity-lever, coincidence-tangent-locus-contact-section-escalate-without-their-rung]
---

Found by the sweep of `an-in-band-axis-parallelism-offers-a-proximity-lever`
(branch `cleave/join-face-surface-one-home`), which gave the cylinder
pair's `cc_axes_parallel` its own decision, `BooleanDecision::CylinderAxes`,
and a turn for its lever.

## What

`join::section_refusal` and the frame's own arms wrap every section-table
escalation as `FrameError::Escalated`, which `join::frame_refusal` refuses
as `Coincidence(Section, Moot)`. Its ending (`refusal_routes`,
`Coincide::Section => Ending::Lever(MEET)`) tells the user to "move the
parts so they clearly meet or clearly stand apart there". Several rungs
behind it read an angle, not a distance: `pc_rim_alignment` and
`pc_axis_plane_parallel` (plane against a cylinder's axis),
`pn_axis_normal` and `pn_conic_type` (plane against a cone's axis and
aperture), `pt_axis_normal` (plane against a torus's axis). A turn moves
them; a translation does not.

A public door reaches `pc_rim_alignment`: the one-face unit rods of
`parallel_cylinder_join::one_face_rod`, A's seam at azimuth 0
(`one_face_rod((0, 0), 1, 0, (-1, 1))`) and B's at π
(`one_face_rod((1.2, 0), 1, PI, (-0.3, 0.3))`), with B tipped `1.3·zero`
about `x` through `(1.2, 0, 0)`: `union(a, b)` at ε = 1e-9 refuses
`Escalated { Coincidence(Section, Moot) }`, predicate `pc_rim_alignment`,
margin `1.3e-9`, ending in the proximity lever; `union(b, a)` refuses
`CylinderAxes` on the same pose.

## Direction

Read each rung `FrameError::Escalated` carries for what it measures. An
angle rung takes a decision of its own with a turn for its lever, as
`BooleanDecision::CylinderAxes` does; the distance rungs keep `MEET`.
The topotail row on the section family's missing rung is the same
plumbing, so the two may land together.



## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEJOIN (`work/honejoin/`) by `git mv`, id and body unchanged: whose charter is the curved and chord join's residue. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

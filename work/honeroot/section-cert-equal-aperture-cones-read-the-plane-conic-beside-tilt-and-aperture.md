---
id: section-cert-equal-aperture-cones-read-the-plane-conic-beside-tilt-and-aperture
kind: issue
title: section_cert's equal-aperture cone pair serves the plane conic beside the tilt and the aperture gap one at a time
status: open
opened: 2026-10-11
priority: P2
cost: M
---


## What

`topo::boolean::section_cert`'s `cone_cone`
(`crates/topo/src/boolean/section_cert.rs:2036`), reached from
`axis_pose`'s parallel pose, decides the two cones' aperture gap
`cos²α₁ − cos²α₂` levered at the reach (`section_cone_pair_aperture`,
`:2057`) and on Zero serves `cone_plane_pair` (`:2083`): the section read
as one cone's section by the plane its forms' difference names, exact
only for parallel axes and equal apertures. The tilt (decided Zero by
`section_axes_tilt`) and the aperture gap are each just inside the band
and their sum is not read: the plane conic's class is then decided by
`cone_plane`'s aperture margin as though both were zero.

Found by the sweep of `section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time`
(branch `cleave/section-cert-axis-pose`). That unit measured its own
shape and found no wrong verdict at the default K, only a served
reading off by up to one and a half bands; this one is likely the same,
a claim-level P3.

## The shape of a fix

Carry the slack `axis_pose` now hands its parallel pose
(`Pose::Parallel { slack, .. }`) and the aperture gap's own levered
reading into `cone_plane`'s rows as one bracket, read across
(`signs_across` in the same file, over `geom_brep::intersect::decide_across`).

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: tilt and aperture gap decided one at a time: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEROOT (`work/honeroot/`) by `git mv`, id and body unchanged: whose charter is the boolean's root doors and sector readers and their margins. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

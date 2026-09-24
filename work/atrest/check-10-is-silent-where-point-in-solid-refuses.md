---
id: check-10-is-silent-where-point-in-solid-refuses
kind: issue
title: tier 3's shell-winding check is silent on a shell whose sign or point-in-solid walk cannot answer: spline and Approx faces, partial curved faces, escalations
status: open
opened: 2026-09-24
priority: P3
cost: D
refs: [tier-3-does-not-check-shell-roles-per-solid, check-9-nesting-is-line-bounded-only]
---



Filed by ATREST-7, which landed check 10 (`crates/topo/src/validate.rs`,
`shell_winding_errors`). The check answers only where both of its reads
answer, by design (the spec's D-B: this program must not add to the
false-refusal direction), so its residue is exactly where they do not:

- **A shell's role**: its own signed volume, certified by
  `props::sign_certified` over the shell's faces in the certified door
  and by check 7's `plus_v` hook in the battery. Silent where the sign
  stays in band, where the quadrature schedule runs out first, and at
  every door that makes no check 7 (`validate_geometric_structural`).
- **The walk**: `boolean::solid_contain::point_in_solid_faces` over a
  per-shell selection (`SolidFaces::of_shell`). Silent on
  `KindUnsupported` (NURBS and `Approx` faces), on
  `PartialSphereFace` / `PartialConeFace` / `PartialTorusFace`, on an
  escalation or an exhausted ray schedule, on `VolumeUncertified` (a
  no-hit ray whose at-infinity side needs a closed-form volume the
  props inventory refuses), and on a group-read surface key shared
  across two shells (`SurfaceSharedOutsideSolid`).

So a winding-2 or winding-(-1) solid whose offending shell is
spline-walled certifies. What closes it is the walk growing arms (the
spline kinds are `KindUnsupported`'s own open subject), not a change
here. Named in `validate_geometric`'s not-yet-checked list.

---
id: check-10-is-silent-where-point-in-solid-refuses
kind: issue
title: tier 3's shell-winding check is silent on a shell whose sign or point-in-solid walk cannot answer: spline and Approx faces, partial curved faces, escalations
status: open
opened: 2026-09-24
priority: P3
cost: D
refs: [3301, 3179, 3204, ray-wall-and-cone-near-root-cancels-over-a-small-lead]
---



Filed by ATREST-7, which landed check 10 (`crates/topo/src/validate.rs`,
`shell_winding_errors`). The check answers only where both of its reads
answer, by design (the spec's D-B: this program must not add to the
false-refusal direction), so its residue is exactly where they do not:

- **A shell's role**: its own signed volume, read by
  `props::sign_walk` over the shell's faces through the lane the door
  made check 7 through, and decided by check 7's own `plus_v_decide`
  (`shell_role` in `validate.rs`). Both doors make it: the composed
  `validate_geometric` through the certified quadrature, and
  `validate_geometric_structural` (which makes check 7 through the
  closed form) through the closed form, where a shell with a face that
  needs the quadrature has no role. Silent where the sign stays in band,
  where the schedule runs out first, and where the walk refuses.
- **The walk**: `boolean::solid_contain::point_in_solid_faces` over a
  per-shell selection (`SolidFaces::of_shell`). Silent on
  `KindUnsupported` (NURBS and `Approx` faces), on
  `PartialSphereFace` / `PartialConeFace` / `PartialTorusFace`, on an
  escalation or an exhausted ray schedule, on `VolumeUncertified` (a
  no-hit ray whose at-infinity side needs a closed-form volume the
  props inventory refuses), and on a group-read surface key shared
  across two shells (`SurfaceSharedOutsideSolid`). A witness vertex
  where two shells TOUCH (`OnBoundary`) is skipped for the shell's next
  vertex; only a shell every vertex of which touches another is silent.

So a winding-2 or winding-(-1) solid whose offending shell is
spline-walled certifies. What closes it is the walk growing arms (the
spline kinds are `KindUnsupported`'s own open subject), not a change
here. Named in `validate_geometric`'s not-yet-checked list.

## The other direction: a WRONG answer is a false refusal (review, 2026-09-26)

Check 10 promotes the point-in-solid walk from a boolean helper to an
at-rest gate. The silences above are the safe failure: a walk that
REFUSES leaves a verdict unmade. A walk that ANSWERS WRONGLY is not
absorbed by any silence — an `Out` from inside, or an `In` from outside,
becomes a `ShellWinding` refusal of a VALID body. This is not
hypothetical: it is how check 10 first measured
(`work/atrest/point-in-solid-reads-out-from-inside-a-re-posed-torus-barrel`,
closed by ATREST-9 — the planar arm read arc-bounded caps as their
vertex polygons, so the re-posed hollow torus barrel refused), and
`work/contact/ray-wall-and-cone-near-root-cancels-over-a-small-lead` is
a plausible future source of the same kind. Every defect of that kind
in the walk now reaches `validate_geometric` and every verb that
validates its own output; a finding against the walk's correctness is
therefore a finding against tier 3's refusal surface, and should be
banded as one.

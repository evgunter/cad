---
id: the-trimmed-lanes-nu-2-seam-assertion-fires-on-cylinder-booleans-at-a-coarse-delta
kind: issue
title: The trimmed lane's 'nu = 2 with an id repeated apart' assertion, documented as vacuous, panics on cylinder booleans at δ = 1
status: closed
opened: 2026-10-06
priority: P1
cost: M
branch: tesslie/nu-2-repeat
closed: 2026-11-20
pr: 7200
---


## What

Found by `join/pinch-one-vertex-per-cone` (PR 4074) while meshing
review r2's cylinder pinch poses (`r2_pinch_probes cyl`, on
`join/pierce-pinch-families-review-r2`) at δ = 1.0.
`crates/mesh/src/trimmed.rs`, the cylinder arm of the candidate grid,
asserts `nu != 2 || !id_repeats_apart(&polygon)` and documents it as
"VACUOUS TODAY ... the two conditions are arithmetically exclusive".
It panics on these bodies. None of them is a pinch (`pts=0`: no two
vertices on one point). The assertion runs before the CDT, so PR 4074's
change, which reads only the CDT's pinch handles, cannot reach it.
`[profile.release]` arms debug assertions, so it fires in release.

Lines (`<prism> cyl <dir> psi=<ψ> <off|seam> <order> <op>`, δ = 1.0):
- `Lbot cyl fib4 psi=2.2 seam pc U`;
- `notchbot cyl fib15 psi=0.9 off pc U`, `psi=0.9 off cp U`,
  `psi=0.9 seam pc U`, `psi=0.9 seam cp U`, `psi=2.2 seam cp U`.

At δ = 0.05 the same bodies refuse `CertificateExceeded` instead. So
the shape is reached at a coarse δ, where `nu` falls to 2 on a wall
whose walk repeats an id apart. "`nu == 2` needs `u1 − u0 ≤ π/2`, a
repeat-apart needs the full 2π seam" is false on these walls.

## The shape to give

Find which id repeats apart on these walls, and why the walk spans
≤ π/2 there. Then either make the arithmetic hold, or handle `nu == 2`
with a repeated id (the #678 fan). Pin one of the lines above.

## Moved to TESSLIE (the mesh answers Ok, or panics, where it should refuse or mesh right) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

## Closed (TESSLIE, PR 7200, 2026-11-20)

The premise was false: no seam was involved. The face's loop passes one vertex twice (a self-touch), and `loop_polygon` reads each entry off its own traversal's pcurve, so the two UV images differ by an ulp and the CDT held two vertices with one mesh id. `trimmed::one_location_per_branch` now gives a later entry of a non-singular id the first entry's UV bit for bit, per branch: the branch is named by `Period::near`'s half-period marks, asked of later entries only. A seam a whole turn off keeps its own location. Across a 2,864-union cylinder battery at δ 1, the reviewer measured 292 panics and 43 Triangulation refusals becoming meshes that converge in volume. The assertion stays, and its comment states the true reason it holds. Pinned by `a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` and `curved_notch_pinch::the_curved_notch_less_a_corner_pinch_meshes`, with check_volume, and by the per-branch unit rows. Follow-ups: `a-vertex-met-twice-on-a-spline-face-keeps-two-rounded-uv-locations` and `cylinder-union-battery-refusals-carry-no-repeated-id`.

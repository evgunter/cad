---
id: the-trimmed-lanes-nu-2-seam-assertion-fires-on-cylinder-booleans-at-a-coarse-delta
kind: issue
title: The trimmed lane's 'nu = 2 with an id repeated apart' assertion, documented as vacuous, panics on cylinder booleans at δ = 1
status: open
opened: 2026-10-06
priority: P1
cost: M
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

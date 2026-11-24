---
id: main-red-the-square-wall-arm-search-mismatches-the-traced-section-at-a-seed
kind: issue
title: main red at a seed: topo's square_search::the_square_wall_arm_agrees_with_the_traced_section reports two mismatches at CAD_FUZZ_SEED=0x76580296e6dc2407
status: open
opened: 2026-11-23
---


Found by BOXFINISH's `boxfinish/signed-radius` in its slow-set
differential; `crates/topo` is not in that branch's diff, and the row
fails the same way on its merge base (main `e1830a1cc5`) at the same
seed.

`cargo nextest run -p topo --ignore-default-filter` draws a fresh seed
per run, so the row is green on most runs. With
`CAD_FUZZ_SEED=0x76580296e6dc2407 CAD_FUZZ_EFFORT=1`,
`boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
reports 2 mismatches, both "a strip crossing at a tube circle's top":

- `R 0.06333508206690769, r 0.02333724935397388, (e, z0, rc) (-0.06017019732448682, -0.01283021472348554, 0.01097334469516636), tilt 0`:
  `R-tan at Degeneracy { tangent: 0.000466310064678014, top: 9.860187388799346e-10 }: Tangent("section_torus_square_wall_fold_half")`
- `R 21.679732433253566, r 10.335857007836012, (e, z0, rc) (20.19537924422026, -10.240324329704167, 1.4874242466198964), tilt 0.00000000000000002820639650964764`:
  `R-tan at Degeneracy { tangent: 1.3918915684880542, top: 2.3693333961460894e-9 }: Tangent("section_torus_square_wall_fold_half")`

Not diagnosed: whether the search's oracle or the arm is wrong at a
crossing within a few ε of the tube circle's top.

**The test that says it is closed.** The row is green at that seed.

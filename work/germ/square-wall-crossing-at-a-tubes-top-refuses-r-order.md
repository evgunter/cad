---
id: square-wall-crossing-at-a-tubes-top-refuses-r-order
kind: issue
title: The square-wall arm refuses R-order where the wall crosses the tube at its top or bottom parallel, though the section is regular there
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [radial-hole-through-a-tube-has-no-section-arm, 4530]
---

## What

`section_cert.rs` `square_folds` and `across_strip` refuse
`Section::Unordered` (R-order) when a fold of the wall's trace `K` on a
tube circle, or a crossing of `K` with a strip edge `z = ±r`, lies
within the band of the circle's top or bottom point `(±R, ±r)`
(predicates `section_torus_square_wall_fold_half` and
`section_torus_square_wall_edge_over`). The carriers cross
transversally there unless `K` is tangent to the tube circle (which
the `_tube` margin refuses R-tan first), so the section is regular and
its topology is stable under every pose the band admits: the refusal
is the arm declining to pick a side, not a pinch.

Measured on PR 4530's fix pass (1,200 poses of the search's family
"a strip crossing at a tube circle's top", offsets log-uniform from
`1e-12` to `0.5` of `min(r, rc)`, seed `0x5eed`): 244 refuse R-order at
ε = 1e-9, 408 at 1e-6, 96 at 1e-12; every pose within the escalation
band of a top point refuses (1,200 of 1,200 at 1e-9).

## The fix it wants

Answer by a side the band admits, the way `reaches_edge` answers a
strip-edge tangency beyond the parallel: read every near-top crossing
as standing just outside the parallel (`|y| > R`), which is one
perturbation of the pose (K moved so each such crossing steps outward)
when the near-top crossings are realizable together. What needs
proving is that joint realizability: up to four top and bottom points
can lie near `K` at once, and a reading no single nearby pose produces
could pair the folds wrongly. A fold and the strip crossing beside it
agree to second order in their offset from the top point (`K` against
`T` there: `z − r ≈ k (y − y_e)` against `−(y − σR)²/2r`), which is
the start of that argument.

Rows: `section_cert_square_rows.rs`
`a_crossing_at_a_tubes_top_refuses_r_order` pins today's refusal and
turns into the answered row.

## A seed of the square search that reds on this family (2026-11-17)

`topo`'s `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
at `CAD_FUZZ_SEED=0x2d854e95cb708d44`, `CAD_FUZZ_EFFORT=1`, reds on main
with one mismatch (found on a SECTARMS branch's run, replayed on main by
PR 7052's reviewers):

    a strip crossing at a tube circle's top: R 0.015043003400221869,
    r 0.007531328765283513, (e, z0, rc) (0.01590952267596522,
    -0.005535154178584106, 0.0021761347361288),
    tilt 0.00000000000644107827255586: R-tan at
    Degeneracy { tangent: 0.00024737568798874343, top: 1.4351601400341973e-9 }:
    Tangent("section_torus_square_wall_fold_half")

A second seed, `CAD_FUZZ_SEED=0xf833f1903ca13458` (`CAD_FUZZ_EFFORT=1`),
reds with three mismatches of the same shape (`R 4.528…, r 0.5539…`;
`R 0.04898…, r 0.007838…`; `R 37.93…, r 8.648…`), each `R-tan at
Degeneracy { … top: ~1e-9 }`, on embed/graph-piece's local run of topo
(2026-11-17); that branch does not touch the section code.

- 2026-11-18 — **another draw of the family, from the nightly slow set** (ONEHOME's PR 7088 lane, local run): `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` drew `CAD_FUZZ_SEED=0xe0950175e9cd5888` at `CAD_FUZZ_EFFORT=1` and found 1 mismatch, "a strip crossing at a tube circle's top": `R 24.455411074718242, r 14.679004457317978, (e, z0, rc) (25.867814796788632, 14.064956615358236, 1.5401100768027751)`, tilt `9.894e-17`, refused `R-tan at Degeneracy { tangent: 0.9968011759838724, top: 7.2513461901735354e-9 }`. It reproduces with main's `topo` and `geom-brep` (`1b8712e48`) with that seed and effort.
- 2026-11-18 — **another seed of the same search**, `CAD_FUZZ_SEED=0xcd6fb650afffa55c` (`CAD_FUZZ_EFFORT=1`), reported by PR 7088's delta reviewer from the nightly slow set, on `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`. The branch does not touch the section code; the mismatch is not replayed here.
- 2026-11-18 — **another seed from the slow set**, `CAD_FUZZ_SEED=0xa853a4936c699bbf` (`CAD_FUZZ_EFFORT=1`), on PR 7088's head `0e4a88b3b` (a local run), on `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`. It found 2 mismatches, both "a strip crossing at a tube circle's top" at tilt 0, refused `Tangent("section_torus_square_wall_fold_half")`:
  - `R 1.7431386315133757, r 0.5301515634601375, (e, z0, rc) (-1.779863039014376, 0.5140418444666239, 0.04010242868703958)`, `Degeneracy { tangent: 0.025302880813285522, top: 4.560202028491833e-9 }`;
  - `R 0.15708184039820414, r 0.03233924842649333, (e, z0, rc) (-0.16267172524735657, -0.040157338658700345, 0.009610900698025776)`, `Degeneracy { tangent: 0.0014056222339962066, top: 3.367093825978884e-9 }`.

  The branch does not touch the section code.
- 2026-11-18 — **another seed of the same `Tangent("section_torus_square_wall_fold_half")` family**, `CAD_FUZZ_SEED=0x80d159b76d7315df` (`CAD_FUZZ_EFFORT=1`), on `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`. PR 7088's delta-3 reviewer drew it from the slow set; it is not replayed here.
- 2026-11-18 — **another seed from the slow set**, `CAD_FUZZ_SEED=0x55e1793adeb1c7e3` (`CAD_FUZZ_EFFORT=1`), on PR 7088's head `343514f92` (a local run). It found 1 mismatch, "a strip crossing at a tube circle's top": `R 0.019909668948330725, r 0.003584650030920035, (e, z0, rc) (0.0522248658400246, 0.011720238237453275, 0.033323555767347746)`, tilt `8.64e-10`, refused `Tangent("section_torus_square_wall_fold_half")` at `Degeneracy { tangent: 0.0025332740334559137, top: 5.644358709588904e-9 }`.

A third seed, `CAD_FUZZ_SEED=0x838f6bb8bc006541` (`CAD_FUZZ_EFFORT=1`),
reds with two mismatches of the same shape (`R 0.08967…, r 0.009745…`;
`R 7.828…, r 5.006…`), each `R-tan at Degeneracy { … top: ~1e-9 }:
Tangent("section_torus_square_wall_fold_half")`, at ε = 1e-9 on
importgate/build's local run of topo (2026-11-18); that branch does not
touch the section-certificate code.
- 2026-11-19 — **another seed from the slow set**, `CAD_FUZZ_SEED=0x2a3bc317ec000517` (`CAD_FUZZ_EFFORT=1`), on PR 7099's merge of main (`bb3030aed`, a local run). It found 3 mismatches of the same `Tangent("section_torus_square_wall_fold_half")` shape, "a strip crossing at a tube circle's top", at tilt 0 (the first: `R 0.03718180571997164, r 0.026862353293602703, (e, z0, rc) (0.013995399530219547, -0.03597452314082404, 0.02491266790414395)`). The branch does not touch the section code.
- 2026-11-19 — **another seed from the slow set**, `CAD_FUZZ_SEED=0x339a0ffc371059d8` (`CAD_FUZZ_EFFORT=1`), on PR 7099's head `d51b45838` (a local run), on `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`: 2 mismatches of the same family. The branch does not touch the section code.
- 2026-11-19 — **another seed from the slow set**, `CAD_FUZZ_SEED=0x1d6e16a1245acf31` (`CAD_FUZZ_EFFORT=1`), on PR 7099's head `1efd4c4df` (a local run), on the same row: 3 mismatches of the same family, "a strip crossing at a tube circle's top", at tilt 0 (the first: `R 26.979895739936712, r 24.905971927878202, (e, z0, rc) (-30.689145667498128, -27.7322030360061, 4.663273232846134)`). The branch does not touch the section code.

Moved from GERMTAIL to GERM on 2026-11-20, by `git mv`, when GERMTAIL merged into GERM: GERMTAIL held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GERM carries.
- 2026-11-20 — **another seed from the slow set**, `CAD_FUZZ_SEED=0xa1ab1c0d0f7d7a02` (`CAD_FUZZ_EFFORT=1`), on `onehome/placed-boolean-doors` merged with main `a0924e461` (a local run), on the same row: 3 mismatches of the same family, "a strip crossing at a tube circle's top" (the first: `R 7.028423511900605, r 6.482671226270791, (e, z0, rc) (21.213647545706475, 21.985762723140905, 21.01348201972288)`, tilt 1.8e-12). The branch does not touch the section code.

---
id: section-cert-square-wall-reads-its-trace-beside-the-square-tilt
kind: issue
title: section_cert's square-wall arm reads its trace and fold drift beside the square tilt one term at a time
status: open
opened: 2026-10-11
priority: P3
cost: M
---


## What

`topo::boolean::section_cert`'s `square_pose`
(`crates/topo/src/boolean/section_cert.rs:1290`) decides the wall's
axis square to the torus axis on `a·d` levered from the pivot
(`section_torus_square_wall_tilt`), then reads the wall's trace `(e, z0)`
at the pivot as if the axis stood exactly square. The turn that squares
it moves the wall by up to `|a·d|·(|centre − pivot| + radius)`, half the
band, anywhere the section can matter. `torus_square_cylinder` and
`square_folds` (`:1392`, `:1450`) then decide their rows on `(e, z0)`
alone: the top and bottom reach, the tube rows, the fold half, and the
served Zero of `section_torus_square_wall_fold_drift` (`:1481`), whose
drift carries the trace's rounding `slop` but not the turn's
displacement. A fold whose shallow angle stays inside the band for the
rounding can move `turn / sin θ` along the tube circles.

Found by the sweep of `section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time`
(branch `cleave/section-cert-axis-pose`), which fixed the same shape in
`axis_pose`. There the measurement found no wrong verdict at the default
K: each definite row stands `K` bands past its flip, the residue under
one and a half. The same is likely here, which makes this a claim-level
P3.

## The shape of a fix

Read each row across the reach beside the turn's swing (the `across` and
`signs_across` helpers `axis_pose` now uses), and add the swing to the
fold drift's numerator beside `slop`. The square-wall search
(`section_cert/square_search.rs`) measures what moves.

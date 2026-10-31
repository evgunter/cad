---
id: section-cert-square-wall-reads-its-trace-beside-the-square-tilt
kind: issue
title: section_cert's square-wall arm reads its trace and fold drift beside the square tilt one term at a time
status: open
opened: 2026-10-11
priority: P2
cost: M
---


## What

`topo::boolean::section_cert`'s `square_pose`
(`crates/topo/src/boolean/section_cert.rs:1325`) decides the wall's
axis square to the torus axis on `a·d` levered from the pivot
(`section_torus_square_wall_tilt`), then reads the wall's trace `(e, z0)`
at the pivot as if the axis stood exactly square. The turn that squares
it moves the wall by up to `|a·d|·(|centre − pivot| + radius)`, half the
band, anywhere the section can matter. `torus_square_cylinder` and
`square_folds` (`:1427`, `:1485`) then decide their rows on `(e, z0)`
alone: the top and bottom reach, the tube rows, the fold half, and the
served Zero of `section_torus_square_wall_fold_drift` (`:1516`), whose
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

Read each row across the reach beside the turn's swing (`signs_across`
and `geom_brep::intersect::decide_across`, which `axis_pose` now uses), and add the swing to the
fold drift's numerator beside `slop`. The square-wall search
(`section_cert/square_search.rs`) measures what moves.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: the trace read as if the decided-square axis stood exactly square: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEROOT (`work/honeroot/`) by `git mv`, id and body unchanged: whose charter is the boolean's root doors and sector readers and their margins. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

2026-10-31 (SPLICE's W5 lane, evidence only): one red of
`section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
at ε 1e-12 on `splice/w5-loop-free-sweep`, its seed not captured; nine
re-runs at 1e-12 were green. That branch changes no torus or cylinder
reading (the shared chart recursion gains a duty; its seeding and
accounting duties are unchanged), so the mismatch is this fuzzer's,
possibly this row's drift. Replay needs a seed: run with
`CAD_FUZZ_SEED` logged (`--no-capture`).

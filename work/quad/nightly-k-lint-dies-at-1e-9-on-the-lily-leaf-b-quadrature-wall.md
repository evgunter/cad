---
id: nightly-k-lint-dies-at-1e-9-on-the-lily-leaf-b-quadrature-wall
kind: issue
title: The nightly k-lint (dev-probe) dies at ε = 1e-9 on demo tour's lily_leaf_b quadrature-convergence refusal, so the 1e-12 pass and the lint step never run
status: open
opened: 2026-10-09
priority: P1
cost: E
---


(Filed by the ENCL orchestrator from the review of PR 4423, which hit this while re-taking k-stream dumps.)

## What

The nightly's `k-lint (dev-probe)` job (run 37889242439, main `13089c93c7`, 2026-10-09) dies at `demos/tour/src/main.rs:512`:

> lily_leaf_b: mass properties failed … props_quad_converged … margin -2.717e-9 … (1e-9, 1e-8)

The panic takes down the ε = 1e-9 demo pass, so the 1e-12 pass and the k-lint step itself never run. The nightly's k-distribution lint is therefore blind.

## Relation to open rows

- The **cause** is tracked in `quadrature-convergence-test-escalates-instead-of-refining.md`, in its "Evidence (2026-10-06, CARVE)" section. There the refusal is a tour wall pinned on purpose (`LEAF_B_VOLUME_WALL`).
- The **consequence** (a red nightly) is not on any row. The same shape at 1e-12 was `projectbox-cutaway-…-under-k-lint.md`, closed by PR 3976.

## Repair shape

The demo pass must not die on a refusal it expects. One option is for the dev-probe tour to treat the pinned wall as an expected typed refusal and carry on, as PR 3976 did for the cutaway. The other is to fix the cause. Either way, the k-lint step has to run again.

## 2026-10-09 — the leaf_b refusal is gone at the default ε (NURBS, PR 4438)

PR 4438 retires wall 17: `lily_leaf_b` measures at the default ε
(`V = 0.003134 m³ ± 8.0e-8`), so the release tour at the default ε
(1e-9) runs green, and so do its 1e-12 and 1e-6 rows. The k-lint
dev-probe itself was not run here. The next nightly after that PR
merges says whether this row can close.

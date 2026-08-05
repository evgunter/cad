# docs/archive — index

Historical milestone documents, moved here 2026-08-05 (docs-rot
unit; method ratified by Evan: `docs/archive/` + this index + a git
tag `archive/2026-08-05` laid by the orchestrator on the archiving
merge). Everything remains greppable and history-following
(`git mv`). Nothing here is normative: the living contract is
`docs/DESIGN.md` + the companion design docs it lists; the live
milestone record is the highest-numbered `docs/M*-PLAN.md` /
`M*-LOG.md` remaining in `docs/`.

Format: file — what it was (milestone) — superseded by.

## Plans, logs, exit walks

- `M0-PLAN.md` — M0 work order (geom-core scalar/intervals, arenas, validation) — DESIGN.md D1/D4; the design-question-PRs-wait-for-sign-off rule lives in CLAUDE.md/git-workflow.
- `M0-LOG.md` — M0 orchestrator log (L-numbered unilateral decisions) — conventions ratified into DESIGN.md.
- `M1-PLAN.md` / `M1-LOG.md` — topology + Euler operators milestone — D1's tiers and the validator checklist, DESIGN.md.
- `M2-PLAN.md` / `M2-LOG.md` — analytic curves/sweeps/tessellation/STL milestone — DESIGN.md; K telemetry story continues in K-REPORT.
- `M3-PLAN.md` / `M3-LOG.md` — intersections/booleans/mass-properties milestone — DESIGN.md; PERF-PLAN carries the deferred-quadratic record.
- `M4-PLAN.md` — M4 work order incl. the F1–F8 fork framings (#80) — outcomes: M4-LOG appendix (relocated from DESIGN.md).
- `M4-LOG.md` — M4 orchestrator log; carries the appendices holding the M4 shipped-list and fork-outcome record relocated from DESIGN.md — still the fork trail of record.
- `M4-EXIT-WALK.md` — M4 exit-criteria walk — done-state summarized in DESIGN.md's M4 bullet.
- `M5-PLAN.md` — M5 work order — DESIGN.md M5 bullet + M5-EXIT-WALK (kept live in docs/).
- `M5-LOG.md` (4259 lines) — M5 orchestrator log — M5-EXIT-WALK is the done-state of record.
- `M6-PLAN.md` — M6 work order (main-path completions) — M6-LOG close statement; re-banked items listed there and in DESIGN.md's M6 bullet.
- `M6-LOG.md` — M6 orchestrator log, ENDS WITH THE M6 CLOSE STATEMENT (units merged: #171/#176/#192/#178; re-banks: unit 5 fillet vocabulary, the ratified sense-flip gate, the k-lint floor) — cited by DESIGN.md, K-REPORT, CONTACT-DESIGN.
- `M6-BOUNDARY.md` — the #161 milestone-boundary ruling — paraphrased into DESIGN.md's M6/M7/M8 roadmap bullets (judgment call: archived rather than merged verbatim; flag if any nuance is missed).

## Per-unit binding specs (superseded by merged code + PR descriptions)

- `M3-PR6A-SPEC.md` — M3 tier-3′ split unit (M3).
- `M4-PR1-SPEC.md` … `M4-PR6-SPEC.md`, `M4-PR8-SPEC.md` — M4 unit contracts (recipe substrate, DAG, naming, persistence, exit sweep). Note: M4-PR8-SPEC §D5/D8 name "standing" process conventions — all now carried by memories/orchestration-model.md and MODEL-AB-LOG; no unique standing content found (flagged per Evan's extraction refinement).
- `M5-PR1-SPEC.md` … `M5-PR14-SPEC.md` (incl. `M5-PR7B-SPEC.md`, `M5-PR9C-SPEC.md`) — M5 main-path unit contracts (interval swap, C9 ring, NURBS substrate, projection/fitting, BVH, SSI, pcurves, ellipse dispatch, loft/sweep nodes, tessellation+props, fillets+die, curved STEP, exit sweep).
- `M5-S1-SPEC.md`, `M5-S2-SPEC.md`, `M5-S6-SPEC.md` … `M5-S11-SPEC.md`, `M5-S13-SPEC.md` — M5 side-unit contracts (two-tolerance sweep, arc-leg sugar, orientation sense, S13 containment enablers, …).
- `M6-2-SPEC.md` — SSI generic-`T` lift unit contract (M6, merged #176).
- `M6-3-SPEC.md` — loft/sweep body-assembly unit contract (M6, merged #192).

Code comments citing pre-archive paths (`docs/M<n>-…`) were NOT
rewritten by the docs-only archive unit — a follow-up code-lane
sweep may repoint them; grep still finds the files here.

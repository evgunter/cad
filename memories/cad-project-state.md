---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0–M4 COMPLETE, M5 COMPLETE at the PR 14 exit sweep 2026-08-03 (Evan ruled: #89 CLOSED / K=10 permanent, shape (v) accepted piecewise, 3 conventions ratified; walk 13 MET / 7 honest / 0 carried-unowned; live status = docs/M5-LOG.md tail + docs/M5-EXIT-WALK.md); RENUMBERED 2026-08-03: M6 = main-path completions (SSI lift, loft assembly, composition surgery, analytic-chart pcurves, census design doc), M7 = STEP adoption ONLY, M8 = error propagation (was M6); merge gate = hosted Actions, gate.sh fallback (see git-workflow); references live in the MAIN checkout; name pending (Q9)
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Greenfield B-rep CAD kernel in Rust (repo evgunter/cad).
**docs/DESIGN.md is the authoritative, ratified design contract** —
read it before any design or implementation work; do not re-litigate
settled decisions D1–D9. This memory is CURRENT STATE ONLY; the
PR-by-PR history is the milestone logs (trimmed 2026-07-23, Evan-
approved: each log keeps only its latest state snapshot; merge-only
git history preserves the rest).

**Milestones:**

- **M0 COMPLETE (2026-07-16)** — geom-core: comparison-free `Real`,
  trilean predicates, single-ε `Tolerance`, intervals/duals, linalg;
  topo skeleton. Record: docs/M0-LOG.md.
- **M1 COMPLETE (2026-07-16)** — half-edge topology, ten Euler
  operators, tier-1/2 validators, cube + holed box through public ops
  only. Record: docs/M1-LOG.md.
- **M2 COMPLETE (2026-07-21)** — analytic curves/surfaces, profiles,
  EdgeGeometry certification, extrude/revolve, certified tessellation,
  STL + admesh gate, exact mass properties, K-report (K = 10 FINAL,
  run-configured). Record: docs/M2-LOG.md; docs/K-REPORT.md.
- **M3 COMPLETE (2026-07-23, exit walk 13/13 zero gaps)** —
  splitting/booleans/tier-3′ per docs/M3-PLAN.md, all through the
  full implement → adversarial-review → fix-pass cycle: PRs 1–5.5
  (#53, #55, #61, #62, #65, #70), PR 6a (#75 —
  `validate_pseudomanifold`, coincidence census, two-directional
  declared-contact certification, touching corpus), PR 6b (#76 —
  DESIGN.md exit sweep, K inventory, state-doc trim), PR 6c (#73 —
  scripts/gate.sh). Known envelopes on record in DESIGN.md's M3
  conventions block (operand-internal-declaration gap → M4;
  both-sided pinch frontier; PR 5.5 seam refusals). Record:
  docs/M3-LOG.md (M3 EXIT section + final snapshot).
- **Pre-M4 design ratified (2026-07-23)**: docs/NAMING-DESIGN.md
  (#74, N1–N7) and docs/SOLVER-DESIGN.md (#79, W1–W9 — M4 takes
  the contracts; solver implementation is M6).
- **M4 COMPLETE (2026-07-27, exit walk 12/12)** — the parametric
  model layer: recipe substrate + expression sublanguage, scalar-
  generic evaluation with a memoized result DAG, the naming stack
  (StableName/RolePath, resolution + diff + Rebind, GeomSource),
  AP214 STEP export with FreeCAD acceptance, persistence schema v1
  (frozen), declared-tangency discipline, watertight CDT
  tessellation, the Band 4 corpus + rebuild-latency lane, and the
  K-telemetry + large-K lint. Record: docs/M4-LOG.md,
  docs/M4-EXIT-WALK.md.
- **M5 COMPLETE (2026-08-03, exit walk 13 MET / 7 MET-WITH-RECORDED-HONESTY / 0 carried-unowned)** — curved geometry: exact `Ellipse`
  carriers, SSI marching with the three-limb certificate, certified
  pcurve storage, per-class curved booleans (plane×cylinder,
  plane×sphere), `Loft`/`Sweep` nodes + schema v2, certified
  tessellation and quadrature mass properties, curved STEP export,
  constant-radius fillets and the die; the interval backend swapped
  to the in-house crate (inari GONE from the tree). PR 14 (the exit
  sweep) carries Evan's rulings (2026-08-03): **#89 CLOSED**, K = 10
  the permanent ratified default with an in-band-landings re-open
  trigger; **acceptance shape (v) accepted PIECEWISE** with the
  composition surgery sequenced early in M6; the three DESIGN.md
  conventions RATIFIED. Exit walk: 13 MET / 7
  MET-WITH-RECORDED-HONESTY / 0 carried-unowned.
  **Seven units are banked by name with typed doors** — composition
  surgery, the SSI generic-`T` lift, loft/sweep body assembly, the
  canal blend (PARKED), cyl×sphere germ chords, the NURBS extent
  lift, curved REST contact. Record: docs/M5-LOG.md,
  docs/M5-EXIT-WALK.md, docs/M6-BOUNDARY.md.
  **NOTE (2026-08-03): docs/M5-LOG.md and docs/MODEL-AB-LOG.md
  DIVERGED between main and the orchestrator branch
  `mngr/cad-implement-m5-7plus` — main carries the per-unit
  technical entries, the branch carries the dispatch/ruling
  narrative, and neither is a superset. PR 14 imported the A/B log
  from the branch; the M5-LOG merge is still owed.**
- **Sequencing + RENUMBERING (ratified #161, then Evan on PR #169
  comment 5171303851, 2026-08-03)**: **M6 = the main-path
  completions** — SSI generic-`T` lift (first; it gates the rest) →
  loft/sweep body assembly (which also owns completing pcurve
  certification on the ANALYTIC charts — only Plane and Cylinder
  certify today) → in-place edge-blend composition surgery
  (sequenced EARLY, comment 5171351203) → cyl×sphere germ chords /
  NURBS extent lift; plus the census/declared-contact design doc,
  design-only. **M7 = STEP adoption ONLY** (Evan: "we shouldn't fold
  any core work like ball and socket into M7"). **M8 = error
  propagation** — this was M6 until 2026-08-03; `ERROR-DESIGN.md`'s
  body still says "M6" and has a status line saying so.
  Q9 name shortlist parked in [[name-candidates]].

**Key operational facts:**

- **Merge gate = hosted GitHub Actions** (policy 2026-07-25): PR
  checks green = mergeable; `scripts/gate.sh` is the billing-outage
  FALLBACK only. Details + --auto caveat in [[git-workflow]].
- **Reference books/notes live in the MAIN checkout's `references/`**
  (git-ignored; does not propagate to worktrees). Scans read visually
  (poppler installed); the TOG 1986 boolean paper has a text layer.
- **The LGPL quarantine is RETIRED BY REMOVAL (M5 PR 1, #127,
  2026-07-28)**: the `interval` lane's backend is the in-house
  `interval-transcendentals` crate; inari and its gmp/MPFR stack are
  gone from the tree (Cargo.lock zero hits, dev-dependencies
  included), so the kernel is copyleft-free in EVERY build
  configuration and issue #4's exit condition is met. inari survives
  only as an optional differential oracle inside that crate's own
  workspace. The historical x86-64-v3 / AVX+FMA target-cpu floor was
  DROPPED post-swap (2026-07-29, Evan's #127 retroactive review) —
  no correctness need remains.
- License dual MIT OR Apache-2.0; project name still pending (Q9,
  shortlist in [[name-candidates]]).

See [[cad-working-style]], [[orchestration-model]], [[git-workflow]],
[[worktree-disk-hygiene]].

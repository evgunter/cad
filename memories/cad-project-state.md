---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0–M3 ALL COMPLETE (M3 exit walk 13/13, 2026-07-23); NAMING-DESIGN #74 + SOLVER-DESIGN #79 ratified; pre-M4 design DONE (NAMING #74 + SOLVER #79); next = M4-PLAN ratification with Evan; merge gate = scripts/gate.sh (hosted CI DOWN); history lives in the milestone logs, not here
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
- **Pre-M4 design**: docs/NAMING-DESIGN.md (selection stability /
  persistent naming, N1–N7) RATIFIED (#74, 2026-07-23). Remaining
  before M4 planning: NONE — both docs ratified 2026-07-23
  (docs/SOLVER-DESIGN.md, #79, joined NAMING-DESIGN). Next: M4-PLAN
  drafting + ratification conversation with Evan (his sign-off
  lane). Q9 name shortlist parked in [[name-candidates]].

**Key operational facts:**

- **THE MERGE GATE IS `scripts/gate.sh <ref>`** (#73): flock-
  serialized run of scripts/ci-local.sh (11-row mirror of ci.yml) on
  a persistent standalone clone at `~/.local/share/cad-gate/repo`;
  ~4 min warm. Run it on the merged tree before any merge to main.
- **Hosted GitHub Actions is DOWN** (free-plan minutes exhausted;
  resets at Evan's billing-month rollover). No branch protection, so
  `gh pr merge --auto` merges IMMEDIATELY — never rely on it to wait.
- **Reference books/notes live in the MAIN checkout's `references/`**
  (git-ignored; does not propagate to worktrees). Scans read visually
  (poppler installed); the TOG 1986 boolean paper has a text layer.
- The `interval` cargo feature quarantines LGPL (issue #4); x86-64
  floored at x86-64-v3 (inari directed rounding REQUIRES it — never
  override RUSTFLAGS, it silently drops .cargo/config.toml's
  target-cpu).
- License dual MIT OR Apache-2.0; project name still pending (Q9,
  shortlist in [[name-candidates]]).

See [[cad-working-style]], [[orchestration-model]], [[git-workflow]],
[[worktree-disk-hygiene]].

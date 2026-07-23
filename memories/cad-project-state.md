---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0/M1/M2 COMPLETE; M3 nearly complete (PRs 1–5.5, 6a #75, 6c #73 merged; NAMING-DESIGN.md ratified #74; 6b docs sweep + exit walk remain); merge gate = scripts/gate.sh (hosted CI DOWN); history lives in the milestone logs, not here
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
- **M3 IN PROGRESS, near exit** — splitting/booleans/tier-3′ per
  docs/M3-PLAN.md. Merged through the full
  implement → adversarial-review → fix-pass cycle: PRs 1–5.5 (#53,
  #55, #61, #62, #65, #70 — surgery, split, plane_section, boolean
  reduce/classify, public union/intersect/subtract, first voids, seam
  discipline; 21-pip die builds e2e watertight), PR 6a (#75 —
  `validate_pseudomanifold`, coincidence census, two-directional
  declared-contact certification, touching corpus), PR 6c (#73 —
  scripts/gate.sh). docs/NAMING-DESIGN.md (selection stability /
  persistent naming, N1–N7) RATIFIED by Evan (#74, 2026-07-23).
  Remaining: PR 6b (M3-exit DESIGN.md sweep + K snapshot + state-doc
  trim — the PR carrying this memory rewrite) and the M3 exit walk
  against M3-PLAN's exit criteria. Record: docs/M3-LOG.md (its latest
  state snapshot is the resumption contract).

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
- License dual MIT OR Apache-2.0; project name still pending (Q9).
- Pre-M4 design queue: GQ1 mechanism details doc (naming doc done).

See [[cad-working-style]], [[orchestration-model]], [[git-workflow]],
[[worktree-disk-hygiene]].

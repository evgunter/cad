---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0–M3 COMPLETE; M4 IN FLIGHT (plan ratified #80 — live status is docs/M4-LOG.md's tail snapshot, not this memory); merge gate = hosted Actions, gate.sh fallback (see git-workflow); references live in the MAIN checkout; name pending (Q9)
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
- **M4 IN FLIGHT** — plan RATIFIED (PR #80, 2026-07-23,
  docs/M4-PLAN.md; binding per-PR specs in docs/M4-PR*-SPEC.md).
  Which PRs are merged / in review / queued changes weekly: read
  the CURRENT-STATE tail of docs/M4-LOG.md, not this memory.
  Q9 name shortlist parked in [[name-candidates]].

**Key operational facts:**

- **Merge gate = hosted GitHub Actions** (policy 2026-07-25): PR
  checks green = mergeable; `scripts/gate.sh` is the billing-outage
  FALLBACK only. Details + --auto caveat in [[git-workflow]].
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

---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0 and M1 complete (2026-07-16); M2 (analytic geometry, extrude/revolve, tessellation, STL) planned, awaiting ratification
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Greenfield B-rep CAD kernel in Rust (repo evgunter/cad). **docs/DESIGN.md
is the authoritative, ratified design contract** — read it before any
design or implementation work; do not re-litigate settled decisions
D1–D9. D1 now carries the M1 ratifications: half-edge structure (typed
`LoopBoundary`, computed mate, `outer ∉ rings`), the one-rule
orientation convention (interior-left ⇒ CCW-from-outside; GWB diagrams
are MIRRORED — never transcribe), the ten-operator set + `ring_move`
with site-enum addressing and the atomic/deterministic/postcondition
contract, and the three validity tiers (euler-valid / closed-solid /
geometric) with component-aware per-shell Euler–Poincaré.

**M0 complete (2026-07-16)** — `geom-core` (comparison-free `Real`,
trilean predicates, single-ε `Tolerance`, linalg) + `topo` skeleton.
**M1 complete (2026-07-16)** — half-edge topology + all ten Euler
operators + 12-pass tier-1 validator + `validate_closed`; cube and
holed box build through public ops only; raw builder is crate-internal;
`Body<Interval>` instantiates. PRs #15–#26 (see docs/M1-LOG.md; M0:
docs/M0-LOG.md). Notable: Mäntylä Program 11.6 erratum on record
(reading notes); replay-with-kills is per-arena (see D9); the
adversarial-review corpus of both milestones runs in CI
(`review_m{0,1}_pr*` suites).

**M2 next**: analytic curves/surfaces, extrude/revolve from
polyline+arc profiles, tessellation, STL export. `docs/M2-PLAN.md`
drafted, PR #24 awaiting Evan's ratification; forks flagged there:
profile format (bulge-chain recommendation) and revolve pole policy.
K-value experiments run in M2 (first predicates in anger). Mäntylä
ch. 12/13 notes archived in `<main-checkout>/references/notes/`.

Key operational facts: **reference PDFs and notes live in the MAIN
checkout's `references/`** (git-ignored dirs don't propagate across
worktrees — NURBS book + Hoffmann were stranded in the original
session's worktree until 2026-07-16); the `interval` cargo feature
(geom-core AND topo) quarantines LGPL per issue #4 (closed; README +
rustdoc carry the consumer-facing note); x86-64 floored at x86-64-v3;
CI: fmt/clippy/test + ε matrix {1e-6,1e-9,1e-12} + interval lane +
`Real +` discipline grep. License dual MIT OR Apache-2.0; name still
pending (Q9). See [[cad-working-style]], [[orchestration-model]],
[[git-workflow]].

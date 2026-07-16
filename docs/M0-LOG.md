# M0 Implementation Log

Orchestrator's running log for M0. Two purposes: (1) record design
decisions made *during* implementation that didn't need Evan's input but
should be visible and revisitable; (2) snapshot orchestration state so any
session can resume. Update and commit at every checkpoint.

Decisions here are numbered L1, L2, … ("log decisions") to distinguish
them from DESIGN.md's D-series. An L-decision is one the orchestrator made
unilaterally; if any turns out to be contentious it gets promoted to a
design conversation.

## Process conventions

- Orchestrator (this log's author) does central planning, design, and
  meta-review; implementation and first-pass review are delegated to
  subagents (Opus for straightforward tasks, Fable for medium/hard).
- Branches: `ev/m0-<n>-<slug>`, one per M0-PLAN PR, stacked serially
  (each off the previous until the previous merges). PRs target `main`.
- Design PRs (2, 3, 4, 5, 7 — the Q1 residue) are opened with a full
  design writeup in the description and **wait for Evan's sign-off**;
  work continues stacked on top in the meantime, accepting rework risk
  if review changes the design. Scaffolding-type PRs (1, 6) are
  self-merged after subagent review.
- Merge commits only, per CLAUDE.md git workflow.

## Log decisions

- **L1 — Workspace layout**: virtual cargo workspace (no root package —
  sidesteps Q9's pending name), crates under `crates/`, starting with
  `crates/geom-core`. Crate names follow DESIGN.md's layering table.
- **L2 — Toolchain pinning**: `rust-toolchain.toml` pinned to a specific
  stable version (1.97.0), edition 2024. Rationale: D9 determinism —
  same build + same inputs → bit-identical outputs starts with a pinned
  compiler.
- **L3 — CI shape**: GitHub Actions; jobs: `fmt --check`,
  `clippy --all-targets --all-features -- -D warnings`, `test`, plus a
  multi-ε matrix job (env `CAD_TOLERANCE_EPS`) that for now just reruns
  tests — the env var gets wired to `Tolerance` initialization in PR 2.
  The env var name is provisional until PR 2's design discussion.
- **L4 — Unsafe and warnings policy**: `#![forbid(unsafe_code)]` in
  `geom-core` (D9 "essentially no unsafe" — the exception budget lives
  in vetted dependencies, not our code). Warnings deny is CI-side
  (`-D warnings` flag), not `#![deny(warnings)]` in source, so local
  iteration isn't blocked by e.g. an unused import mid-edit.

## State snapshot

- **Current**: PR 1 (workspace scaffolding) in progress on
  `ev/m0-1-workspace`; delegated to an Opus implementer; orchestrator to
  review, then self-merge.
- **Next**: PR 2 (Real trait + Tolerance) — orchestrator drafts the trait
  surface design, Fable implements, PR opened for Evan's sign-off; PR 3
  proceeds stacked while waiting.
- **Task tracker**: session tasks #1–#8 mirror the M0-PLAN PR sequence.

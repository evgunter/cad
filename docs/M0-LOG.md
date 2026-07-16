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
- Reviews include hands-on e2e exercise (reviewers write/run real usage
  demos against the API under review), per Evan's standing rule — see
  `memories/review-and-dependency-policy.md`. Same memory: new
  dependency versions want a ~2-week minimum release age.

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

- **L5 — Clippy lint posture** (set while the codebase is empty, on PR 1
  review's recommendation): panic-family lints (`unwrap_used`,
  `expect_used`, `panic`, `todo`, `unimplemented`, `unreachable`,
  `dbg_macro`) at `warn` workspace-wide — CI's `-D warnings` makes them
  hard errors there, mechanically enforcing D9's no-panic rule. Test
  code allows the unwrap/expect/panic lints per-module (panicking is a
  test's failure mechanism). `clippy::pedantic` as a group is
  deliberately *not* enabled (too noisy; targeted lints get added
  individually when they earn their keep). `indexing_slicing` (a real
  panic source) deferred until evaluation code exists to judge noise
  against — revisit at PR 6.
- **L6 — `publish = false`** in `workspace.package` until Q9's name
  lands.
- **L7 — Evaluation-code discipline + CI tripwire** (from PR 2's
  adversarial e2e review): the no-comparison enforcement is structural
  for the convenient paths only; the residual channels (extra bounds
  like `T: Real + PartialOrd`, `Debug` format-string gadgets,
  `Any`/`TypeId` dispatch) are banned by a named style rule documented
  in `real.rs`, and CI's `discipline` job greps for the extra-bound
  pattern (`\bReal\s*\+`). When a legitimate `Real +` combination first
  appears (likely PR 4's bound-extraction trait), refine to an
  allowlist as a design decision. Optional escalation noted for later:
  clippy `disallowed-methods` for `Any`/`TypeId` on scalars.

## PR 2 design conversation (live)

- Evan review 2026-07-16: (1) `sin_cos` as the primitive — agreed and
  **implemented** (sin/cos are defaulted projections, overridable
  bit-identically; f64 overrides for scalar performance); (2) εₐ
  dimensional-honesty concern — orchestrator agrees and has **proposed
  revising D4 ¶1 to a single ε** with angular thresholds always derived
  per predicate as θ = ε/r (lever arm named at the call site;
  `Band::angular_at(r)` replacing `Band::angular()`), **implementation
  held pending Evan's confirmation** (it revises a ratified decision).
  If confirmed: PR 2 drops `eps_angular` + its env var; PR 3 reworks
  `Band::angular()`; DESIGN.md D4 ¶1 revised at ratification.

## PR 4 pre-work (inari probe, 2026-07-16)

Empirical findings (full report in issue #4): transcendentals require
inari's `gmp` feature (MPFR-backed; without it they don't exist) →
LGPL-3.0+ transitive deps (`gmp-mpfr-sys`, `rug`) — **license fork filed
as issue #4** (recommendation: cargo feature `interval`, default builds
stay MIT/Apache + C-free). Hard CPU floor AVX+FMA (plan:
`-C target-cpu=x86-64-v3` via `.cargo/config.toml`; aarch64 fine
unflagged). Determinism excellent (bit-identical enclosures across
CPUs/SIMD paths at pinned deps). Poison model differs from f64: partial
out-of-domain **clamps** (only full misses go empty), violations
signalled via `DecInterval` decorations → PR 4 wrapper builds on
`DecInterval`; `from_f64(NaN)` mapped to empty explicitly. inari 2.0.0
(2024-08-07, MIT itself) satisfies the dependency-age policy.

## State snapshot

- **Done**: PR 1 (workspace scaffolding) merged to main (#2), CI green
  incl. multi-ε matrix.
- **Current**: PR 2 (`Real` + `Tolerance`) on `ev/m0-2-real-tolerance` —
  implemented (Fable), adversarially e2e-reviewed (verdict: ratify with
  wording amendments, all applied), opened as the first **design PR
  awaiting Evan's sign-off**. On sign-off: ratify trait surface,
  totality/NaN policy, evaluation-code discipline, and Tolerance
  once-init semantics into DESIGN.md, then merge.
- **Next**: PR 3 (trilean predicates) proceeds stacked on the PR 2
  branch; design drafted (Sign/Band/Indeterminate + the noise-buffer vs
  sliver-band fork for (ε, kε), recommending sliver-band with k = 10
  provisional).
- **Task tracker**: session tasks #1–#8 mirror the M0-PLAN PR sequence.

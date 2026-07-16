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

## PR 2 design conversation (resolved 2026-07-16)

- Evan review: (1) `sin_cos` as the primitive — agreed and
  **implemented** (sin/cos are defaulted projections, overridable
  bit-identically; f64 overrides for scalar performance); (2) εₐ
  dimensional-honesty concern — orchestrator proposed **revising D4 ¶1
  to a single ε** with angular thresholds always derived per predicate
  as θ = ε/r (lever arm named at the call site). **Evan confirmed**
  (👍 + "current plan sounds good"); implemented: PR 2 dropped
  `eps_angular` + its env var (+ `ToleranceField`), PR 3 replaced
  `Band::angular()` with `Band::angular_at(lever_arm)` (new
  `BandError::InvalidLeverArm`), DESIGN.md revised (D4 ¶1, D2's εₐ
  mentions, deferred list, Q1 residue-status block). Awaiting Evan's
  explicit merge sign-off on the final PR 2 state.

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
**Issue #4 resolved (2026-07-16)**: option (a) — `interval` cargo
feature gates `inari/gmp`; default builds stay MIT/Apache + C-free;
post-M7 roadmap entry added to DESIGN.md for an in-house replacement
that drops LGPL. PR 4 design drafted (orchestrator scratchpad):
DecInterval newtype, decoration < def ⇒ poison, pown override for
tight powers, `Bounds` certification trait, x86-64-v3 floor via
.cargo/config.toml, separate CI job with m4 + caching.

## State snapshot

- **Done**: PR 1 (workspace scaffolding) merged to main (#2), CI green
  incl. multi-ε matrix.
- **Done**: PR 2 (`Real` + `Tolerance`) **merged to main** (#3,
  2026-07-16) with Evan's sign-off after the design conversation
  (sin_cos primitive; εₐ eliminated — D4 ¶1 revised in-branch; Q1
  residue ratified into DESIGN.md). Evan's lingering K concern answered
  on the PR and folded into PR 3's description: **K is a policy dial
  (refusal rate + f64 noise headroom), not a correctness parameter** —
  soundness (escalate-never-guess, certification, interval replay) holds
  for any K > 1.
- **PR 3** (trilean predicates): implemented + e2e-reviewed (verdict
  ratify, amendments applied) on `ev/m0-3-predicates`, εₐ restructure
  merged in; PR to be opened once PR 2 merges (description drafted in
  orchestrator scratchpad).
- **PR 6** (linalg): Fable implementer running in an isolated worktree,
  branch `ev/m0-6-linalg` off the PR 2 branch (affine/linear
  point-vector distinction, column-field matrices, total ops with
  poison propagation — pinned design in the agent prompt).
- **Task tracker**: session tasks #1–#8 mirror the M0-PLAN PR sequence.

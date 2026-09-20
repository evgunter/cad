---
id: klint-gate-has-no-interval-row-so-interval-clippy-is-ungated
kind: issue
title: k-lint (gate)'s five rows are all default-lane, so a clippy warning on the interval lane is ungated by construction
status: open
opened: 2026-09-19
---

## Finding

- **Where**: `.github/workflows/ci.yml`, the `klint_rows` output
  (~:445) and the `k-lint (gate, …)` matrix it feeds.
- **Importance**: medium — a whole lane's lints are ungated, and a
  green run reads as evidence about them
- **Confidence**: sure. The five row names were read off the workflow
  and off a real run's job list; the gap was then hit for real, twice
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

`klint_rows` is `["dev-default", "release-default", "release-budget",
"dev-budget", "dev-probe"]`. Every one of those five is the **default**
lane (one is the `probe` scalar); **none is `interval`**. So
`cargo clippy` never runs with `--features interval` in the per-PR
gate, and a warning that only the interval lane can even compile is
ungated by construction.

`docs/prompts/implementer-discipline.md` tells a lane that *"a green
k-lint means green at all five"* feature unifications, which is true
and is read as covering the matrix. It does not: the twelve `test (…)`
jobs gate `{default, interval} x {default, 1e-6, 1e-12}`, and the five
k-lint jobs gate one lane.

**Hit twice on one branch, which is what makes this a row rather than
an observation.** `dup/private-box-builders` orphaned imports in
feature-gated code:

- five unused imports in `crates/sweep/tests/` behind
  `#![cfg(feature = "interval")]`, which `cargo clippy --workspace
  --all-targets` cannot compile and therefore cannot warn about;
- one more in `crates/sweep/examples/p1b_r2_ab_interval.rs`, reachable
  only with `--features interval`.

All six were found by the lane running
`cargo clippy -p sweep --all-targets --features interval` by hand. None
would have been found by the gate. A lane that did not think to run it
would have merged them, and `unused_imports` is `-D warnings` in the
rows that DO run — so the tree's own posture says these are errors
everywhere except where nothing looks.

**Not a proposal to add a sixth row without measuring it.** The
interval build is the expensive half of the matrix and `k-lint`'s five
rows are already the cheap half; whether the answer is a sixth row, an
interval arm on one existing row, or a nightly row with a named
`workflow_dispatch` proof at the demotion
(`implementer-discipline.md` §2) is this program's call with a timing
measurement. What is not an option is the status quo plus the sentence
that reads as covering it.

## Why it sits here and not on S-DUP's slate

`scripts/work.py territory` puts `.github/workflows/ci.yml` on S-CIW's
ground, and the finding is about the gate rather than about any
duplication. Filed straight onto the owner's slate per
`work/README.md` (*"a lane does not need the owner's permission to put
a finding where it belongs"*); the lane that found it owns no part of
this file.


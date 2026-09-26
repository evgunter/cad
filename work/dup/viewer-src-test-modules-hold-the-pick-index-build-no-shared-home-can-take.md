---
id: viewer-src-test-modules-hold-the-pick-index-build-no-shared-home-can-take
kind: issue
title: pane/viewport.rs's test module restates index_of, which names DocSession and so cannot live in the vocabulary test_support
status: open
opened: 2026-09-26
priority: P4
cost: D
---


## Finding

- **Where**: `crates/viewer/src/pane/viewport.rs`'s `#[cfg(test)]`
  `plate_index(session, delta)` — `tests/common`'s `index_of`, body for
  body.
- **What batch 6 folded, and why this is what is left**: the row as
  filed also named `marks.rs`'s `delta()`, a restatement of
  `plate_delta`. On review the home moved from a `#[path]` mount to the
  tree's `test-support` feature, so it can name `viewer` types, and
  `plate_delta` (with `corpus_delta` and `ring_delta`) moved into
  `viewer::test_support`; `marks.rs` reads it. `index_of` cannot follow
  it: it names `viewer::session::DocSession`, a DRIVER type, and
  `test_support` is a VOCABULARY module (`crates/viewer/README.md`,
  Module boundaries; `viewer-module-kinds.sh` enforces it). A home for
  it needs either a driver-kind test module — a row in the README's
  driver table, which is a design page — or `index_of` restated over
  the landed pair rather than the session. `marks.rs`'s own `plate()`
  indexes through `evaluate`, deliberately, and is not a copy.
- **Importance**: low. One copy, no oracle.
- **Instrument**: every `PickIndex::build`, `landed_generation` and
  `DisplayTolerance::new` in `crates/viewer/src/`, classified test or
  production by its file's first `#[cfg(test)]` and the test-side ones
  read.
- **Raised by**: the S-DUP lane closing
  `viewer-src-test-modules-restate-the-literal-doors`, 2026-09-26;
  narrowed by its review fix pass the same day.

## Why this sits on S-DUP's slate

The ground is `crates/viewer/src/`, `view`'s; the subject — one
construction spelled in two places — is S-DUP's charter. Any claimant
may take it by `git mv`.

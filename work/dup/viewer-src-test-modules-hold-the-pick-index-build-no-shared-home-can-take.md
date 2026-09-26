---
id: viewer-src-test-modules-hold-the-pick-index-build-no-shared-home-can-take
kind: issue
title: marks.rs and pane/viewport.rs test modules restate plate_delta and index_of, which the mounted src home cannot hold
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

- **Where**: two `#[cfg(test)]` modules in `crates/viewer/src/`:
  - `marks.rs`'s `delta()` — `tests/common`'s `plate_delta`
    (`DisplayTolerance::new(2.0e-4)`), restated with a citation of it.
  - `pane/viewport.rs`'s `plate_index(session, delta)` —
    `tests/common`'s `index_of`, body for body.
  `marks.rs`'s own `plate()` indexes through `evaluate` rather than a
  session, deliberately (its doc: the module is a vocabulary), so it
  is not a copy of `index_of`.
- **Why it is a row and not a fold**: the shared src home batch 6
  opened, `crates/viewer/src/test_support.rs`, is ONE text compiled
  into two crates — the lib's unit tests and, by `#[path]`, the `all`
  test binary — so it may name nothing but external crates; `crate::`
  or `viewer::` would mean a different crate in each. `index_of`
  names `viewer::session::DocSession`, and `plate_delta` names
  `viewer::scene::DisplayTolerance`. It is also a VOCABULARY module
  (`crates/viewer/README.md`, Module boundaries), and `DocSession` is
  a driver type no vocabulary may name. A home for these two needs
  either a second, lib-only test module (declared a driver, which the
  README's driver table would have to list — a design page) or a
  different sharing mechanism for `tests/common`.
- **Importance**: low. One copy each, no oracle.
- **Instrument**: every `PickIndex::build`, `landed_generation` and
  `DisplayTolerance::new` in `crates/viewer/src/`, each classified
  test or production by its line against its file's first
  `#[cfg(test)]` and the test-side ones read.
- **Raised by**: the S-DUP lane closing
  `viewer-src-test-modules-restate-the-literal-doors`, which named
  both as "the same shape of gap", 2026-09-26.

## Why this sits on S-DUP's slate

The ground is `crates/viewer/src/`, `view`'s; the subject — one
construction spelled in two places — is S-DUP's charter. Any claimant
may take it by `git mv`.

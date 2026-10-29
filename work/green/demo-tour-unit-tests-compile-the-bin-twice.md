---
id: demo-tour-unit-tests-compile-the-bin-twice
kind: issue
title: demo-tour's unit tests build the 32k-line bin a second time as its test harness, about 3 min of the demos job's release build
status: open
opened: 2026-10-29
priority: P0
cost: M
parent: the-demos-job-is-pr-cis-long-pole-on-its-release-build
---

## What

`demos/tour` is one bin (`src/main.rs` and 46 sibling modules, about
32k lines). Fifteen of its modules carry `#[cfg(test)]` tests (47 of
them, e.g. `gallery::tests`, `lily::review_probes`,
`projectbox::tests`, `main.rs`'s `scene_body_delta`). They use
`super::*` and private items. `cargo nextest run --release` therefore
builds the bin twice: as the bin, which `tests/all.rs`'s
`eps_regression` spawns through `CARGO_BIN_EXE_demo-tour`, and as its
unit-test harness. Both copies are release builds of every scene
the tests reach.

## Measured (2026-10-29, local, 4 cores, the tour's own units only)

With the integration tests already folded into one `all` binary (#5989):

- bin 531 s, harness 538 s, `all` 361 s, tour phase 538 s;
- the same build without the harness (`--test all`): bin 346 s,
  `all` 239 s, tour phase 346 s.

So the harness costs about 190 s of the tour phase locally. Hosted
builds run 1.0–1.5× those times.

## Shape

The harness goes away only if the unit tests stop being compiled into a
second copy of the scene code:
- split a `demo_tour` lib out of the bin (thin `main.rs`);
- move the `#[cfg(test)]` modules into `tests/` as suites of `all` that
  call the lib, making `pub` what they reach. That includes the lily's
  `#[cfg(test)]` helpers, which then compile unconditionally.

A lib split alone does not help: the lib's own `#[cfg(test)]` harness
is the same second copy. The eight bin rows the `ci` profile names
(`demos/tour/.config/nextest.toml`) move to `binary_id(demo-tour::all)`
with their new paths. Count the tests before and after.

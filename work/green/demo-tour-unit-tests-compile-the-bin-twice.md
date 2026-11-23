---
id: demo-tour-unit-tests-compile-the-bin-twice
kind: issue
title: demo-tour's unit tests build the 32k-line bin a second time as its test harness, about 3 min of the demos job's release build
status: closed
opened: 2026-10-29
priority: P0
cost: M
closed: 2026-10-29
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

## Closed (2026-10-29, GREEN, PR #6039)

`demos/tour` is a library (`src/lib.rs`, the scenes) and a thin bin
(`src/main.rs` calls `demo_tour::cli`). Neither target has a test
harness (`test = false`, `doctest = false`, `bench = false`). The 47
unit tests moved to suites of `all` (`tests/<module>.rs`, one per
source module, under their old names, so `snowman::tests::…` is still
`snowman::tests::…`). The lib made public what they reach. The
cfg(test) helpers moved into their suites (lily's `Lance` closed
forms as free functions in `tests/lily.rs`); `patch_quadrature`, a
test oracle, is a `pub mod` of the lib, where
`the-tour-carries-two-patch-quadratures` wants it. The scene code
compiles once per build.

**No test lost.** Listed from the built binaries before and after
(`--features probe`, which adds the three `k_probe_brackets` rows):
101 before (bin 47 + `all` 54) and 101 after (`all`), the same names;
one ignored row before and after
(`chaintol::tests::the_plain_lanes_reach_is_bisected`). Default
features: 98 and 98.

**Measured on one hosted runner** (a measurement-only CI commit,
reverted: kernel warm, then `cargo test --release --no-run --timings`
for main's layout and for this one, back to back):

| layout | units (s) | tour phase |
|---|---|---|
| main: bin, bin harness, `all` in parallel | 648 + 623 + 407 | 648 s |
| lib, then `all` (the bin is 0.7 s) | 268 + 184 | 453 s |

That is −195 s (−30 %) of wall and −73 % of unit-seconds. Locally (4
cores, a shared box) the same pair measured 605 and 461 s.

**Reading a hosted log.** The lib starts on pncad's metadata, so
`Compiling demo-tour` now prints while pncad is still in codegen. The
time from that line to `Finished` is no longer the tour's own part,
and it overstates it: the PR's two ordinary runs read 12.2 min there.

---
id: loud-skip-marker-says-two-modules-and-there-are-six
kind: issue
title: lib.rs's loud-skip marker says two app-feature modules; the split made it six
status: review
opened: 2026-09-04
branch: view/module-kind-gate
pr: 1848
---


`crates/viewer/src/lib.rs:90` opens the loud-skip marker's doc with

> **Loud skip.** The two modules above, and every unit test inside
> them, are absent from a default-feature build of this crate

**There are six.** `lib.rs:78-89` gates `app`, `drafts`, `forms`,
`gpu`, `pane` and `widgets` behind `#[cfg(feature = "app")]`; before
the 1c split it was `app` and `gpu`. The marker's own body predicted
this exactly:

> Nothing here goes red if the modules it names start running, stop
> existing, or grow a sibling — the enumeration above is kept by hand,
> and a marker that silently went stale would look exactly like this
> one.

It grew four siblings and went stale in the same commit.

## Why the 1c fix pass disclosed it rather than fixing it

The doc sentence is a one-word correction, but the marker's payload is
the `println!` at `lib.rs:112-118`, which names `viewer::app` and
`viewer::gpu` and describes their two kinds of row — and the test is
called `app_lane_skipped_no_chrome_or_gpu_coverage_here`. Whether the
four new modules deserve naming there, whether the sentence should
name a FEATURE rather than a hand-kept list of modules (the form that
cannot go stale), and whether the test's name follows are a decision,
not a typo. That pass was scoped to prose with no decision in it.

## The shape that would not go stale

Name the feature and what it costs — "every module behind
`--features app`" — instead of a count. The census the marker exists
to give a reader is `.github/workflows/ci.yml`'s job list either way;
the hand-kept enumeration buys nothing the feature name does not.

## What landed

The marker at `crates/viewer/src/lib.rs` now names the **feature and
what it costs** — "every module this crate gates behind the `app`
feature" — and enumerates nothing. Its second paragraph no longer
admits to a hand-kept list: the roster is the `#[cfg(feature = "app")]`
block above it, which the compiler keeps, so the marker cannot go stale
when that block gains or loses a module. The `println!` payload got the
same treatment and stopped naming `viewer::app` and `viewer::gpu`.

The row is renamed `app_lane_skipped_no_app_feature_coverage_here`. Its
whole payload is its NAME appearing in a default-feature PASS list, so
the name has to say what the body says; `no_chrome_or_gpu` was the same
two-module enumeration one level up. The `app_lane_skipped_*` prefix is
kept, which is what `.github/workflows/ci.yml`, `local-scripts/ci-local.sh`
and `crates/viewer/README.md` refer to it by.

`crates/viewer/README.md`'s own list of the files carrying these rows
was three of four; `tests/panel_display.rs` was added.

This is one of the eight copies `work/issues/loud-skip-marker-is-a-hand-kept-idiom.md`
tabulates, and it takes that issue's option 2 ("drop the enumeration")
for this copy only. The other seven are untouched and that issue's
table row for `src/lib.rs` now names a row that has been renamed —
reported rather than edited, since the issue is homed outside this
program's fence.

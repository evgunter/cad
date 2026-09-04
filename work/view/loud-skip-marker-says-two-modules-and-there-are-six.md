---
id: loud-skip-marker-says-two-modules-and-there-are-six
kind: issue
title: lib.rs's loud-skip marker says two app-feature modules; the split made it six
status: open
opened: 2026-09-04
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

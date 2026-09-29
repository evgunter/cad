---
id: certification-gate-gaps-3-and-5-have-no-follow-up
kind: issue
title: the certification-doors gate's KNOWN GAPs 3 (a value handed to an unlisted helper) and 5 (macros, include!) are disclosed and unscheduled
status: open
opened: 2026-09-24
priority: P3
cost: H
---

## Finding

`scripts/gates/certification-doors.sh` states six KNOWN GAPs. RING-5's
fix pass (#3174) closed GAP 4's re-export half with the REEXPORT rule
and stated GAP 6 (the lane route) as by design. Two stay disclosed with
nothing scheduled behind them (R2 style S8):

- **GAP 3, a value handed on.** A listed file may pass an `Interval` to
  a helper in an unlisted file that has `Real` in scope; the helper is a
  holder, and holders are the census's population
  (`crates/geom-core/tests/certified_endpoint_census.rs`'s `HOLDERS`),
  which is a hand-kept list — its blind spot 6: a new holder is
  invisible until listed. Telling a certification `Interval` from an
  evaluation one needs name resolution, so the honest instrument here is
  a compiler-side one (a lint, or a certification newtype Ev has so far
  declined), not a text key.
- **GAP 5, macros and `include!`.** `lib.sh`'s reader lexes a
  `macro_rules!` body as written and does not follow `include!`, so a
  path to the trait assembled from token fragments, or a macro-written
  `pub use`, is invisible to UNLISTED and REEXPORT alike. None exists
  today; a guard that the tree has no `include!` under `crates/*/src`
  and no `macro_rules!` naming `certification` would make "none today"
  a checked fact rather than a sentence.

Both are latent: no instance exists at `crates/*/src` today. The row is
here so the gate's header is not the only place they are written.

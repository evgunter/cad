---
id: band-linear-spelling-not-swept
kind: issue
title: Band::linear is the canonical spelling of the linear band and ~20 sites still open-code Band::new(eps, k·eps) — sweep or record why not
status: open
opened: 2026-08-31
github: 1408
refs: [1399]
---

## From GitHub issue 1408

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Class finding from SEAT-1's dual review (PR #1399), filed per the findings-need-a-durable-home rule. Both reviewers converged on it independently.

SEAT-1 made `Band::linear(tol)` the one spelling of the linear decision band at every kernel verb door (the doors now derive it themselves). But the unit swept the *parameter*, not the *spelling*: after it, ~20–26 sites tree-wide still open-code the identical derivation as `Band::new(x.eps, x.k * x.eps)` — including production code, not just test helpers:

- `crates/profile/src/path.rs:1470`
- `crates/profile/src/sugar.rs:481`
- `crates/profile/src/validate.rs:1012`
- `crates/profile/src/path/arc_fillet.rs:480`
- `crates/sweep/src/test_support.rs:91`
- plus the surviving `fn band()` test helpers feeding the six sub-door functions that legitimately still take a `Band` (the offset machinery beneath `shell`, `point_in_solid`, `mate::solve::fold_pair`).

Two spellings of one rule is the Q1 drift shape: a change to the canonical derivation (or to `Tolerance`'s fields) would have to find every inlined twin by grep. A follow-up unit should either rewrite the inlined sites to `Band::linear` (pure spelling, no numeric change — same argument as SEAT-1's) or record per-site why the inline form is load-bearing.

Line numbers are as of `0b291b2` and will drift; the greps `Band::new\(.*eps` / `\.k \* .*\.eps` re-find the population.

## Home

`work/seat/` — SEAT's charter §1 is band derivation at operation entry, and this is SEAT-1's own residue: the canonical `Band::linear` spelling it established, unswept at the remaining sites.

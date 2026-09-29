---
id: fitted-lane-refusal-text-omits-symbolic-and-cites-a-retired-hull
kind: issue
title: FittedLaneUnsupported's Display omits the symbolic tier from its replay list and cites the retired exact-arithmetic-ring hull
status: review
branch: scalar/hygiene
pr: 3449
opened: 2026-09-25
priority: P4
cost: E
---


## What

`geom_brep::PcurveCertifyError::FittedLaneUnsupported`'s `Display`
(`crates/geom-brep/src/pcurve_cache.rs`, the `impl fmt::Display for
PcurveCertifyError` arm) reads: "… its between-samples bound is an
exact-arithmetic-ring hull, and this scalar may not certify one.
Replay the body at f64, the telemetry probe, or the interval scalar to
certify it".

- "exact-arithmetic-ring hull" is vocabulary H5 ruling 1 retired
  (`work/scalar/H5.md`); the bound is a certification-arithmetic (C9)
  hull.
- The replay list omits the symbolic tier, which holds the fitted door
  (`topo::AtRestPolicy for Sym<T>`'s `fitted_lane` arm answers `Some`).

Raised by both LANE-4 reviewers (R1 S7, R2 S2). Not fixed in LANE-4,
whose ruling was that no refusal text moves.

## Proposed

Re-word the text to the C9 vocabulary and derive or list the replay
scalars from the certifying set; re-pin the substrings
`crates/topo/tests/m6_2_fitted_at_rest.rs`'s
`the_dual_refuses_at_check_four_and_says_so` asserts
("dual", "may not certify", not "no bracket") and say in the PR what
text moved.

## Cost

E: one string and the one row that reads it.

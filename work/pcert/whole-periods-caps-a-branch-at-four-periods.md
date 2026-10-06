---
id: whole-periods-caps-a-branch-at-four-periods
kind: issue
title: whole_periods refuses a branch more than four periods out (MAX_BRANCH_PERIODS), a cap no producer reaches and no clause states
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [loop-walk-branch-is-an-opaque-floor-atom]
---


Found by PR 3812's dual review (R1 NOTE-1, R2 claim 2), and disclosed
at the constant's site (`geom_brep::MAX_BRANCH_PERIODS`).

`geom_brep::whole_periods` steps one period at a time from `k = 0`, two
sign decisions per step, and refuses a gap more than four periods out
(`BranchMiss::OutOfReach`). Check 4's fidelity and the loop walk both
turn that into `PcurveCertifyError::BranchOutOfReach`. The search had
to stop somewhere: an uncapped one costs two decisions a period and
over an unbounded box does not terminate.

Four is not derived from a bound. It is room above the branches
producers are known to reach:

- the derivation answers on the principal branch;
- a row's azimuth extent is gated to one period (`AzimuthPeriodExceeded`);
- a closed loop's walk closes through at most one period at its seam.

Before 3812 check 4 pushed the stored image through the chart, so any
whole-period shift certified. A stored image five or more periods out
now refuses, typed. No producer mints one today (there is no helical
producer), and no row exercises a real one; the corruption table in
`pcurve_cache/envelope_lemma_fuzz.rs` pins the refusal on a hand-built
`u + 7τ`.

What would close this: either a statement in C4 that a row's branch
lies within `MAX_BRANCH_PERIODS` of its derivation's, or a branch read
off the row's own winding when a producer that needs it lands.

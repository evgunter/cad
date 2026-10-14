---
id: whole-periods-caps-a-branch-at-four-periods
kind: issue
title: whole_periods refuses a branch more than four periods out (MAX_BRANCH_PERIODS), a cap no producer reaches and no clause states
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [loop-walk-branch-is-an-opaque-floor-atom]
needs_ev: true
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

## The question for Ev

PR 5126 found the cap reached by valid input: a circle carrier may be stated any number of periods off the azimuth its points sit at. Its two reviews then found the same cap refusing downstream. Check 4 is reached through `chart_boundary`'s lifted images, and through the public `certify` on the attach path. The torus point-in-solid walk, `offset_axial::keep_turn` and `chart_region::period_fold` have the same shape. Two designers converged on removing the cap everywhere:
- check 4 certifies modulo the chart's deck (a renaming, not a decision), which reverses PR 3812's refusal of a whole-period-shifted image;
- every walk reads a joint's element from the two marks about the gap's own estimate;
- the harmonic derivation places an image over the edge's span in the chart's principal window.

That adds clauses to C4, so it waits for Ev on the `[ev]` PR that sets this flag. PR 5126 is held until it is answered.

Ev then asked (on that PR) whether an angle type that compares modulo
the period would fix this class. Two more designers converged in three
rounds: two small types, a `Period` per periodic channel that the chart
owns (nets included) and an `Azimuth` exit from `atan2` read through a
stated reference, radians kept, stored coordinates never wrapped. They
also settled the open sub-point (a lifted image is the row plus its deck
element, never re-certified) and dropped the walk's estimate (every
decision's gap is within about two periods once images are placed).


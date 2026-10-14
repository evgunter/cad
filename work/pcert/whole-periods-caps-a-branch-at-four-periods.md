---
id: whole-periods-caps-a-branch-at-four-periods
kind: issue
title: whole_periods refuses a branch more than four periods out (MAX_BRANCH_PERIODS), a cap no producer reaches and no clause states
status: closed
opened: 2026-10-02
priority: P3
cost: E
refs: [loop-walk-branch-is-an-opaque-floor-atom]
closed: 2026-10-14
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

## Measured (PCERT, 2026-10-14)

**Census.** I instrumented `whole_period_count`, which every branch
search calls, and ran geom-brep, topo, sweep, step-import, mesh and
editor-core in release (10555 passed, about 12M calls):

- `split_window_branch` reads k ∈ {−1, 0, 1, 2}.
- `pcurve_fidelity_branch` reads k ∈ {−1, 0, 1, 2} on real rows. It reads
  k = −3 and `OutOfReach` only in the hand-built corruption rows of
  `envelope_lemma_fuzz`.
- The other non-walk callers read |k| ≤ 2.
- `pcurve_loop_branch` reads mostly k ∈ {0, ±1}, and k = ±2 in about 48k
  calls across about 60 suites. That is one whole period on a sphere,
  whose walk counts half periods.
- It reads **k = ±4 in 24 calls**, all in `pcurves::pole_slit_tests`, on
  a minted pole-slit cap.

**The premise is false.** On the sphere the cap was counted in the
walk's half steps, so `4` was two whole periods. The pole-slit cap sat
exactly at it.

**Cause.** The cap's first rim arc is a circle carrier whose parameter
runs a period below the azimuth it covers. The joint gap carries that
offset, and nothing bounds it: an `EdgeCurveSpec` circle may run over any
`[t0, t1]`. The same fixture with the arc 3 periods below refused
`BranchOutOfReach` from plain `mint_pcurves` on a legal body. So valid
input reached the cap; this was not a missing statement.

**Check 4 never sees the offset.** On the sphere slit cap and on a
half-turn cylinder patch, with arcs 0, 1, 3, 4, 5 and 10 periods below,
every fidelity read was `Ok(0)` with the gap exactly 0. The stored image
is minted as its own re-derivation, and the offset lives only in the
walk's joint gap.

## Closed (PCERT, 2026-10-14)

Where the loop walk's branch search from zero runs out of reach, it now
searches again from the gap's own estimate
(`geom_brep::whole_step_count_any`, `geom_brep::branch_estimate`). The
search from zero is kept first, so every answer it already gave, and
every decision behind it, stays as it was. A first cut that started
every search at the estimate moved pinned decision streams in six
editor-core rows, and one answer: a narrow-cone joint whose gap of
exactly π ties the estimate to 1 read its far mark first, in band at
that lever, and escalated instead of answering `OnMark`. The estimate,
`geom_brep::branch_estimate`:
- It is `⌊gap/step + ½⌋`, through `Real::periodic_branch`, read as an
  integer where the scalar's floor is a point bracket.
- Where the gap's bracket straddles a mark, it is the step below that
  mark, so the search decides the mark rather than running out of reach.
- The marks still decide every integer, so any parameter offset is in
  reach.

The cap counts whole periods on every orbit (`whole_step_count`, with
`Orbit::per_period` steps to a period). `MAX_BRANCH_PERIODS`' doc states
what it bounds:
- for the walk, the estimate's error plus the near-pole read's offsets;
- for check 4 and the other callers, which start at zero, the branch
  itself, which valid input leaves at zero.

Check 4 and its corruption rows are unchanged. C4 needs no edit: it says
the walk decides the deck element as an integer with half a step of
room, and names no cap.

Rows:
- `pcurves::pole_slit_tests::a_carrier_stated_periods_off_its_azimuth_mints`:
  the sphere at 3 and 10 periods (mints, re-mints from every anchor,
  reads clean) and the cylinder at 5 and 10 (mints, reads clean). On
  main the sphere refuses from 2 periods and the cylinder from 5; a
  cylinder 3 periods out is inside the cap even on main.
- `pcurve_cache::branch_search_rows`:
  - the cap counted in whole periods on a half-step orbit;
  - a gap 25.3 periods out, decided from its estimate and refused from
    zero;
  - a straddled mark 40.5 periods out, ending `OnMark`.

Mutants:
- The search never retried from the estimate reds the door row.
- `Orbit::per_period` read as 1 on the sphere is not reachable through
  the walk, because the estimate keeps the search within a step. The unit
  row pins the counting in `whole_step_count`.

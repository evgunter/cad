---
id: offset-fit-stall-face-has-no-fixture
kind: issue
title: offset_fit's RefinementStalled face has no fixture at any door, so the verdict-before-budget ordering is unpinned
status: review
opened: 2026-09-12
priority: P3
cost: E
branch: encl/offset-fit-loop-faces
rides_with: offset-fit-budget-face-speaks-for-a-round-whose-bound-rose
pr: 3294
---


## The finding

`OffsetFitError::RefinementStalled` — the face `Refine::Refuse` produces
in `fit_offset_at`'s refinement loop (`crates/geom-brep/src/offset_fit.rs`,
`stall_verdict` and the loop below it) — is reached by no fixture in the
suite, at either door.

Until the `‖E‖` floor read the components together (PROPS
mignitude-floor), the bumpy patch at `d = 1e-6`, `tol = 1e-9` reached it:
the bound fell on round 4, rose on round 5 and rose again on round 6, and
`a_stall_on_the_budgets_last_round_is_the_stall_not_the_budget` in
`crates/geom-brep/tests/offset_fit.rs` asserted the face. With the
tighter bound that request certifies at `7.610e-10` on round 3, and a
hunt over 60 further requests (bumpy / sphere-band / a 1000:1 quarter
cylinder × twelve `d` × tolerances down to `1e-15`) produced only the
budget, cap and not-finite faces — `STALLS=0`.

## What is now unpinned

`stall_verdict`'s own admission set still has its unit rows
(`only_a_both_directions_round_that_gains_nothing_refuses` and
neighbours). What has no row at all is the LOOP's ordering: the stall
verdict is taken before the round-budget test, so a last round whose
strongest step gained nothing wears the stall's face and not the
budget's. That ordering is a live branch with no witness; the
replacement row
(`a_single_non_improving_round_is_the_budgets_face_not_the_stalls`)
pins only the other side of it — a SINGLE non-improving round reaching
the budget face.

The module doc's `# Reachability` section on `stall_verdict` already
records the verdict as "close to unreachable BY CONSTRUCTION" and says
the arm is "pinned by `OffsetFitError::RefinementStalled`'s own row
rather than by a fixture that does not exist". That is now literally
true at both doors, which is what makes this an item rather than a
contradiction: the options are a constructed fixture (a base whose
bound provably rises twice), a `#[cfg(test)]` seam that drives the loop
with a scripted bound sequence, or a ratified decision that the arm
carries no door-level row.

## Not scheduled here

PROPS mignitude-floor's fence is the bound, not the guard.

## A fixture exists (ENCL tight-ε lane, 2026-09-25)

Found by `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`'s
measurement (PR 3272). Both of these reach `RefinementStalled` through
the shipped constants, no scripted seam needed:

- **The twisted-loft saddle wall at `d = ±5e-10`, target 1e-14**
  (`crates/sweep/tests/common/approx.rs`, `twisted_loft(0.3)`, any of
  its four spline walls): `RefinementStalled { rounds: 4, grid: (16, 12),
  achieved: 1.2915e-11 }` from `fit_offset_at`, identical at both signs.
  The loop certifies the same wall at 1e-13 on round 2 (5.4e-14), so
  by round 4 the bound has risen more than two orders past what round 2
  reached; the intermediate rounds were not traced at this `d`.
- **Through the kernel door**, the same request is
  `crates/sweep/tests/offd_r1_probes.rs`'s
  `the_fitted_obstruction_holds_on_a_curved_fit` run with
  `CAD_TOLERANCE_EPS=1e-14`: `replace_face_offset` on that wall at
  `d = 5e-10` refuses at the fit, and the row's `Fit` arm passes.
  `CURVED_FIT_REACH` (1e-13) is what keeps that arm legitimate. No CI
  row is that tight, so the fixture exists but nothing runs it today.

The rise is the Bézier decomposition's insertion width
(`work/props/f64-refinement-inside-an-enclosure-has-five-more-sites.md`,
its `insert_once_ring` row). Under the convex form that row proposes,
the same wall certifies at 1e-14 on round 3, so **this fixture
disappears if that fix lands**. A row built on it would need
re-grounding then, which argues for the scripted-bound seam named above
if the stall's loop ordering is to be pinned durably.

`bowed()` at `d = 0.05` also stalls, but only with the round budget
raised past the shipped 6 (round 7, 2.49e-9), so it is not a
shipped-door fixture.

---
id: certify-and-refine-loops-spell-one-stop-rule-two-ways
kind: issue
title: the offset fit's refinement loop and the SSI hull subdivision spell one stop rule two ways: improvement against the previous round with a per-grid cap, against the lowest bound with an idle-rounds count and a cumulative work budget
status: closed
opened: 2026-10-11
priority: P3
cost: M
branch: encl/one-stop-rule
pr: 4800
closed: 2026-10-12
---


Filed from SHELL's saddle-fit unit (PR 4672), at the reviewer's finding.

Two certify-and-refine loops in `geom-brep` stop on the same question, "is refinement still paying, and is the work bounded?", and answer it two ways.

- **The offset fit** (`crates/geom-brep/src/offset_fit.rs`, `refine_rounds_at` and `stall_verdict`):
  - It compares each round's bound with the PREVIOUS round's.
  - One round that does not fall earns the both-directions step; a second one refuses (`RefinementStalled`).
  - Work is bounded per grid: `OFFSET_FIT_SAMPLE_CAP`, a total sample count, read before every round, the seed's included.
  - Nothing bounds the cumulative work. Each round refits its whole grid, so the round count is bounded only because the grid strictly grows under the cap. The worst measured is 11 rounds, about 18 s in release.
- **The SSI hull subdivision** (`crates/geom-brep/src/ssi/certify.rs`, limb 2's loop):
  - It compares each round with the LOWEST bound reached so far.
  - It stops after `SSI_HULL_IDLE_ROUNDS` (2) rounds in a row that do not lower that bound.
  - Work is bounded cumulatively: `SSI_HULL_WORK` (spans plus each round's cuts) and `SSI_HULL_CUTS`.

Owed: one stop rule with one home, which both loops read. The rule has to settle three things:
- which bound "improved" is measured against, the previous round's or the lowest;
- how many idle rounds are let pass;
- whether work is bounded per round or cumulatively.

Then each loop's rows re-baseline on it.

## Design (2026-10-12, one designer, ENCL orchestrator adopting)

The rule texts in play (C2 limb 2, f562bd87e1; the offset-fit section, 1d242b4917) are agent-written, so this is ENCL's call, not an `[ev]` fork.

Owed, restated: one stop rule shared by the loops, not one loop (`refined-sign-and-limb-two-subdivide-halve-by-two-loops` measured why the loop bodies stay apart). The rule has one home in geom-brep (working name `crate::descent`):

- **Comparison:** improvement is measured against the lowest certified bound so far. No round counts as idle before the first finite bound, since falling from +∞ is convergence.
- **Patience:** stop on the second idle round in a row. This one constant replaces `SSI_HULL_IDLE_ROUNDS` and the offset fit's two-step guard, and its doc says why one idle round is let pass. Each loop chooses its own action on idle 1: the offset fit marks both directions; `subdivide` does nothing.
- **Work:** every loop gets both bounds. A per-object cap (`OFFSET_FIT_SAMPLE_CAP`, `SSI_HULL_CUTS`) bounds what is built; a cumulative budget (`SSI_HULL_WORK`, plus a new offset-fit budget set from measured per-round traces with headroom) bounds the wait.
- **Precedence:** when several stops fire on one round, the stall wins.

Refusal faces:
- The offset fit keeps `RefinementStalled`, now admitted on two idle rounds against the lowest bound.
- `SampleCapReached` and the new cumulative stop become one wall face, naming which wall, with the recourse "split the face".
- `subdivide` may carry why it stopped (optional).

Expected moves:
- Some offset-fit `SampleCapReached` refusals become an earlier `RefinementStalled` (a rise, then a recovery that stays above the lowest).
- The monotone saddle rows do not move.
- `subdivide` almost unchanged.

Measure the per-round bound traces first, across `tests/offset_fit.rs` and the sweep saddle rows.

Out of this unit: `ssi::refine::refine_by_certificate` is a third instance. It compares against the previous round and is bounded only by `SSI_MAX_STEPS`. It belongs to SSIMARCH's `ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall`, whose envelope is Ev's; a note is posted there.

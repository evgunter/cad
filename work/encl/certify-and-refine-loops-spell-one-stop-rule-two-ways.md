---
id: certify-and-refine-loops-spell-one-stop-rule-two-ways
kind: issue
title: the offset fit's refinement loop and the SSI hull subdivision spell one stop rule two ways: improvement against the previous round with a per-grid cap, against the lowest bound with an idle-rounds count and a cumulative work budget
status: open
opened: 2026-10-11
priority: P3
cost: M
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

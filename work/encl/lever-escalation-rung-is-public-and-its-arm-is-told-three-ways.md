---
id: lever-escalation-rung-is-public-and-its-arm-is-told-three-ways
kind: issue
title: LeverEscalation's rung is public beside a private verdict, is_collapsed is dead, and the arm decision is told in three shapes
status: dispatched
branch: encl/lever-escalation-one-shape
opened: 2026-10-08
priority: P3
cost: E
---


## What

These are follow-ups from the delta re-review of PR 3431 at `cd9214980a`. That review approved the PR; these are its notes and style findings.

- **`rung` is public beside a private verdict.** `LeverEscalation.rung` (`crates/geom-brep/src/enters.rs:88-99`) is the one public field that can contradict the `refused` field's doc invariant ("Some on the arm's rung alone"). No path writes it today, so the type holds only by convention.
  - Fix: a private `rung` with a `pub const fn rung()`, the `OutwardNormal` pattern in the same file. About 20 read sites in 8 files: `certify.rs:2539`, `validate.rs:2343`, `boolean/mod.rs:3099`, `ops.rs:2518,2591,6650`, `sectors.rs:590`, the `dihedral.rs` tests, `review_m2_pr3_certify.rs:470,499,505`, and `sweep/tests/must_carry_rule.rs:576,632`.
- **`is_collapsed()` is dead.** `LeverEscalation::is_collapsed()` (`enters.rs:129-132`) is public and has no caller. Delete it.
- **`of_lever`'s body is written twice.** `ops.rs:2523-2538` `seam_refusal` hand-writes `BooleanError::of_lever`'s body (`boolean/mod.rs:3093-3101`), and the census row at `offer_rows.rs:2609` exists only for that second spelling. Fix: one `(rung, diag)` constructor beside `of_lever`.
- **The arm is told in three shapes** (D4 ¶1 (iv)):
  - certify's noun "the edge's length for the angle between its faces" (`certify.rs:259`);
  - `certify_undecided`'s "…long enough to measure the angle between its faces…" (`validate.rs:2769`), which drops "for how its faces curve";
  - the boolean seam's "… is undecided".

  Make them one.
- **Two wording nits on the arm's at-rest text:**
  - The lead's "at this tolerance" sits beside an `at_zero` note that says no tolerance decides it (`validate.rs:3501-3505`, `certify.rs:557-560`).
  - The note's "curving to a point" also fires on a zero extent (`dihedral.rs:114-116`).

  The definite arm's at-rest text is at 74 of 75 words, so any rewording must shorten.

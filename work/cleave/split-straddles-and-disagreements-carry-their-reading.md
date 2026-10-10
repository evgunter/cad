---
id: split-straddles-and-disagreements-carry-their-reading
kind: unit
title: split's straddles and disagreements mint INVALID: a straddle goes on the log as its enclosure, a disagreement is typed (mints C6-C9)
status: open
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [split-escalations-end-a-poisoned-margin-in-the-plane-lever]
---

Unit 2 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## Sites

- **C6, `splitting/containment.rs:311` (`ReadEscalation::straddle`).**
  Two sound bounds on one quantity straddle the band, which is a real
  indeterminacy (design item 6). Its margin is the enclosure `[lo, hi]`,
  and it belongs on the frame's escalation log.
  - `k_stats` has no door for this today, so the unit adds one to
    `geom-core`, which is SCALAR's ground; announce the seam.
  - `MarginDiag::enclosure` is on `scripts/gates/reporting-margin-door.sh`'s
    mint list.
- **C7, `containment.rs:1993` (`point_in_arc_loop_boundary_disagreement`).**
  This walk's row decided `q` on the edge, and the caller's pass decided
  it off.
  - Its comment says only the test-only identity pass parts the two.
  - If that holds, this is an invariant (D9 row 4). Otherwise it is a
    typed disagreement carrying both readings. Read it and decide.
- **C8, `splitting/order.rs:126` (`in_plane_frame`, `split_join_frame_arm`).**
  Every schedule member decided Zero or Negative.
  - Per #3686's correction, log one gate escalation after the loop, not
    one per member.
  - ENCL's scope reads this as unreachable with non-parallel schedule
    rays. If that is shown, it is an invariant instead.
- **C9, `chord_join.rs:907` (`agreed_section`).** Two section readings of
  different classes. Each reading is sound, so carry both, or carry a
  straddle, to `SectionError` and on to `SplitJoinError`.
  `split-escalations-end-a-poisoned-margin-in-the-plane-lever` owns the
  ending.

## Constraint

Same as unit 1: pin each moved arm's ending. `RefusedArm::Straddle` is
the ending door for a straddle.

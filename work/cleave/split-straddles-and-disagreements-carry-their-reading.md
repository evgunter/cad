---
id: split-straddles-and-disagreements-carry-their-reading
kind: unit
title: split's straddles and disagreements mint INVALID: a straddle goes on the log as its enclosure, a disagreement is typed (mints C6-C9)
status: closed
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [split-escalations-end-a-poisoned-margin-in-the-plane-lever]
branch: cleave/split-straddles-carry-reading
closed: 2026-10-10
pr: 4578
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

## A ninth site, found by unit 1's census (2026-10-10)

`splitting/order.rs` has a second mint beside C8: `split_join_frame_axis`.
It hand-mints `invalid_margin::invalid(exact, "split_join_frame_axis")`
when no member left an in-band reading (`order.rs:197` on main
`cc3df241cc`). It came in with `f56de98848`, after the re-scope's census
was taken. It has the same shape as C8, and the re-scope's pass 1 finds
it.

## Built (branch `cleave/split-straddles-carry-reading`)

Measured first on `origin/main` (`19c43d1a4b`) at ε = 1e-9, 1e-6 and
1e-12: C6's two straddles, C8's frame fallback and C9's rim patch each
returned `MarginDiag::INVALID`, and an open `k_stats::Bracket` held no
escalation for any of them. C7 is not reachable without the test-only
identity pass.

- **The doors (geom-core, SCALAR's ground).**
  - `k_stats::straddle(name, lower, upper, band)` takes two `Decided`
    bounds of one quantity. It records their enclosure
    (`MarginDiag::hull`) on the frame. It is `unreachable!` unless each
    verdict is one its margin supports (`MarginDiag::admits_verdict`)
    and the lower bound's verdict is below the upper's.
  - `k_stats::gate_rejection(name, decided, band, passes)` is a gate's
    rejection of a verdict the funnel already reached. It is
    `unreachable!` for a verdict its margin does not support, or one
    `passes` admits.
  - The gate doors mint through the same private `mint_rejection`,
    inside `classify_gated_reported` (#4571's shape), so there is one
    rejection path.
- **C6.** Both `ReadEscalation::straddle` sites hand the straddle door
  the two bounds' decided readings (`decide_reported`,
  `decide_magnitude_reported`). The span's straddle and the hit's carry
  an enclosure and are on the log.
- **C7. The fact is decided once now.** `contfp`'s pre-pass and the
  carrier walk ran the same `LoopEdge::contact` arithmetic under two
  row names, so they could part only through the identity pass.
  `carrier_loop_side` now takes the pre-pass's readings (`OffEdge`),
  and the walk decides none of them again. The disagreement mint is
  gone, and so is its name.
- **C8.** Each member is decided through `decide_magnitude_reported`.
  Where every arm decides Zero, the frame refuses once through
  `gate_rejection`, on the member nearest to passing. This is reachable:
  the arms all read Zero exactly where the points' spread is within the
  zero band of the plane origin.
- **The ninth site, `split_join_frame_axis`** (`axis_plane`, which unit
  1's census found). Its fallback is `unreachable!` now. Three decided
  components always settle whether the plane is an axis plane, because
  two of them share a verdict and the loop returns on the second. So
  the fallback is reached only with an undecided component in hand, and
  that component's own escalation is returned.
- **C9.** `geom_brep::plane_cylinder_section_read` returns
  `pc_axis_plane_parallel`'s `Decided`. `agreed_section` takes one
  `WallReach` (one `at`, two `across` measures) and builds both reaches
  from it. The conic lane and the rulings' lane parting is
  `pc_axis_plane_parallel_straddle`. Any other parting is
  `unreachable!`, with its values:
  - within the rulings' lane the gap datum is one number;
  - within the conic lane, the rows that part it read no reach.

Endings were pinned before and after at the three ε rows.
- Unchanged: the walk's `Boundary` and `ArcSpan` straddles, which end
  through `RefusedArm::Straddle`; `OrderEscalated`; and
  `SplitJoinError::Escalated` at a split and under a Boolean.
- Moved, on paths that read the margin's kind rather than the
  straddle's type:
  - the Boolean germ frame's `Coincide::Section`: from the lever plus
    the unreadable-margin note, to the lever alone;
  - the census crossing cut's `pm_census_ef_cross_arc_straddle`: from
    the defect ending to `too_close`'s menu.

  Both are now the ending `RefusedArm::Straddle` gives.
- Moved, traced in the fix pass, and pinned in `refusal_routes`: a
  `bool_contact_arc_straddle` read through `rim_wedge`'s seam cover
  (`lift` → `departures` → `Coincide::Rim` / `TangentLocus` with
  `Spent(Seam)`). It went from the lever plus the unreadable-margin
  note to the lever alone. A rim spent as another class ends in its
  frontier both before and after.
- Not reached: editor-core's `clearance.rs` `is_invalid()` route. Its
  `source` is the clearance site's own `decide` on the bound's margin,
  so no straddle reaches it.

## Closed (PR 4578, 2026-10-10)

The split's straddles carry their enclosure on the log through `k_stats::straddle`, which takes
two `Decided` bounds. The one gate-rejection mint is `k_stats::gate_rejection`, sharing
`classify_gated_reported`'s path. Disposition per site:
- **C6:** both containment straddles are enclosures.
- **C7:** decided once (`CarrierLoop::place_off`).
- **C8:** one rejection of the member nearest to passing, pinned.
- **C9:** `agreed_section` takes one `WallReach`, and a parting other than conic against ruled is
  `unreachable!`.
- **`split_join_frame_axis`:** `unreachable!` by pigeonhole.

Two endings moved to the lever and the coincidence menu, and both are pinned.

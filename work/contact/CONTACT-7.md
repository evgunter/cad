---
id: CONTACT-7
kind: unit
title: the census touch analysis reads vertex distances from a plane in metres over the touch point's finite star; a Rest is built only by that check
status: closed
opened: 2026-09-28
priority: P1
cost: H
branch: contact/7-metric-touch
closed: 2026-09-28
---


Carries `touch-cone-readings-are-levered-directions-not-face-distances`
(P1), `census-touch-cones-are-a-third-vertex-sector-builder` (P1) and
`zero-dihedral-conflates-flat-with-slit-in-touch-cones` (P3). Spec:
`docs/CONTACT-7-SPEC.md`.

Designed by a designer pair (one Opus and one Fable, identical problem
statements). The two agreed on the reading. The orchestrator settled
their two differences (log, 2026-09-28): A's structural guard is
taken, and so is B's answer that the census builds no fan. No ratified
text moves, so this was not put to Ev.

Review tier: **dual.** It replaces the reading behind the door every
consumer takes as proof of no interference, and every later census unit
stands on it.

## Closed

The census touch analysis reads in metres, over the touch point's
finite star.
- **The piece.** Each star face is read through its visibility polygon
  at `p` (`face_piece`), and every construction choice errs smaller.
- **The verdict.** Every verdict sign is a piece vertex's signed
  distance from a candidate plane, through `mod metric`'s private
  `Distance`.
- **The Rest.** It is minted only by `rest::certify`, and it is neither
  `Copy` nor `Clone`.
- **Deleted:** the fan (`Cone`, `touch_lever`, the census's sector
  building).
- **The source row** `the_touch_analysis_decides_only_through_its_doors`
  pins `Distance`'s one construction and bans every other decider
  spelling in the section.
- **The Rest's tolerance** (S6′) is written at arm 2's loop.

Measured, head against base:
- the obtuse-sector witness flips to not-Rest at 2 to 500× zero;
- the real-corner dihedral reads convex at both ends;
- CONTACT-1's L-bracket rests clear;
- the brick grid, the rebuilt rotated prisms, the crossed ridges and a
  new 927-pose comb and channel sweep give 0 wrong clears, with false
  refusals equal to base pose for pose (12 and 18, the known saddle and
  `AllOn` cases).

Design: a designer pair (one Opus, one Fable) agreed on the metric
reading. The orchestrator took A's structural guard and B's no-fan
home. The lane found that a whole-face reading refuses the L-bracket
rests; both designers then agreed on the visibility-piece amendment.
Not put to Ev: it is internal to `census.rs` and moves no ratified
text.

Review:
- A **dual** on `d176506ec` (DR row in `docs/DUAL-REVIEW-LOG.md`). Both
  reviewers returned APPROVE-WITH-FIXES and found the same MAJOR
  independently: an edge collinear with the probe ray behind `p`
  refused the piece, which grew false refusals on the comb and the U
  channel.
- The fix pass answered the union. It deleted three readings with
  proofs rather than pinning them, so a single delta review checked the
  proofs by attack. They held, with no wrong Rest.
- A last pass tightened the guard.

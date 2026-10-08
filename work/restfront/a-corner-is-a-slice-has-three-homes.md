---
id: a-corner-is-a-slice-has-three-homes
kind: issue
title: "A corner is a slice of its face" has three homes with no shared core: corners_disjoint, the mesher's PinchWedge, and check 9's wedge_holds
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-corner-is-a-slice-of-its-face-tier-3-check]
---


## What

Found by the review of PR 4240 (`REVIEW.md` S2). One rule, a corner at
a point a face's boundary passes several times holds none of that
boundary's other sides, is spelled three times:

- `crates/topo/src/test_support_meeting.rs` `corners_disjoint`, the
  test oracle. It uses a Newell normal and `atan2` angles, groups
  corners by position rounded to a micron, and compares corners
  across loops.
- `crates/mesh/src/planar.rs` `Pinches::id_in`
  (`TessellateError::PinchWedge`). It reads sectors of the CDT round a
  handle, against each pass's two constraint sides, across loops.
- `crates/topo/src/validate.rs` `wedge_holds` and `pinch_corners_at`,
  check 9's corner arm. It groups by point key and uses levered signs
  about the outward normal at the point, within one loop (two loops
  meeting are the contact arms').

No shared core exists, so the three can drift. The oracle and the
mesher read across loops and the arm does not, which is the first such
difference.

## The shape to give

Decide whether the oracle should call the at-rest arm (and keep a
cross-loop half of its own), or stay an independent derivation on
purpose, as an oracle. If it stays independent, say so beside it. The
mesher works in its chart's CDT and will likely stay separate. Say
that beside `id_in`.

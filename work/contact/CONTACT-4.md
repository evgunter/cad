---
id: CONTACT-4
kind: unit
title: contfp reads every loop on its carriers: point_in_carrier_loop replaces the vertex-polygon walk and point_on_arc's pre-pass
status: closed
opened: 2026-09-26
priority: P0
cost: D
branch: contact/4-contfp-carriers
closed: 2026-09-28
---


Carries `contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop` and `point-on-arc-endpoint-zone-compresses-by-sin-half-width`. Spec: `docs/CONTACT-4-SPEC.md`.

Review tier: **single, full**. The carrier walk is well measured (ATREST-9). The risk is the swap under five live callers.

## Closed

`contfp` runs one boundary pre-pass (`LoopEdge::contact`: a segment,
or a conic read on its own carrier with its trim as distances), then
`point_in_carrier_loop` over the outer loop and its rings. The
`LoopShape` dispatch, `disc_side` and `point_on_arc` are gone.
`LoopShape` keeps one consumer, check 9's arm 4. The walk's `None`
becomes a typed `ContainError`, confined to a ball around each spiric
or spline edge.

Reconciled with ATREST-12 (#3288), which reached main after this unit
branched and decided the same four questions. It took main's
`arc_trim`, ray-window trim, in-band retry and span rule. The delta
review of the merge found main's span rule unsound for ellipses:
- an ellipse window over-wound by up to `10ε/b` read as an arc;
- a point on the doubled edge then answered `In` or `Out`;
- the reviewer's probe gave 1,800 wrong readings.
This is the lever class again: the lower speed bound is safe for
"wound" but unsafe for "arc". The fix pass makes the rule two-sided,
reading "arc" only under an upper bound on the edge's speed over the
overlap. The fix also closes check 9's exposure on main; no validation
door can carry an overlap past the band. A carrier end off both of its
stored vertices now escalates instead of reading `Corrupt`.

Review:
- A single full review, and a fix pass.
- A delta review of the ATREST-12 merge (fix first, one MAJOR). It
  also checked 9,315 ray escalations that now answer through main's
  retry against an independent oracle: 9,315 right, 0 wrong.
- The orchestrator read the last pass: the speed bound is sound
  (`|P″| ≤ a`), and an in-band overlap lies inside the end zone, which
  always escalates.

Filed on RESTFRONT's slate:
- `check-9-and-classify-contain-describe-contfps-retired-polygon-walk`
  (re-homed).

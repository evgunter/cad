---
id: a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle
kind: issue
title: path_property's straight-arrival-off-an-arc row escalates in the non-adjacent segment-pair check's carrier_line_circle at eps 1e-6 (proptest find, red on main)
status: open
opened: 2026-10-07
priority: P0
cost: M
---


Found by the fillet-radius P0 lane (PR #4259) at `CAD_TOLERANCE_EPS=1e-6`. The lane confirmed the same red with main's own `crates/profile/src/sugar.rs`, so the fillet change did not cause it.

- **The row:** `crates/profile/tests/path_property.rs:1496`, `a_straight_arrival_off_an_arc_departure_rides_the_authored_ray`, a proptest.
- **The drawn case:** `h = 1.014505150367573`, `r = 0.47328623976475265`.
- **The failure:** validation's non-adjacent segment-pair contact check escalates instead of building: `EscalationSite::SegmentPair(seg 1, seg 3)`, predicate `carrier_line_circle`, margin −5.567e-6 at ε = 1e-6 (inside the band [ε, kε)). It is not the junction verifier.
- **Where it holds:** the same case is green at the default ε, and the escalation is identical with main's own `crates/profile/src/sugar.rs` (both measured by the PR #4259 review).

The proptest draws fresh cases per run, so like the fillet seed this reds some runs of the 1e-6 row and not others. The regression file was not kept. Pin the two values above as a deterministic row first, then root-cause the escalation: either segments 1 and 3 really sit within the line × circle band of each other at that ε and the row's claim is too strong there, or the path places one of them with more error than it has.

---
id: a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle
kind: issue
title: path_property's straight-arrival-off-an-arc row escalates in the junction verifier's carrier_line_circle at eps 1e-6 (proptest find, red on main)
status: open
opened: 2026-10-07
priority: P0
cost: M
---


Found by the fillet-radius P0 lane (PR #4259) at `CAD_TOLERANCE_EPS=1e-6`. The lane confirmed the same red with main's own `crates/profile/src/sugar.rs`, so the fillet change did not cause it.

- **The row:** `crates/profile/tests/path_property.rs:1496`, `a_straight_arrival_off_an_arc_departure_rides_the_authored_ray`, a proptest.
- **The drawn case:** `h = 1.014505150367573`, `r = 0.47328623976475265`.
- **The failure:** the path's junction verifier escalates in `carrier_line_circle` instead of building.

The proptest draws fresh cases per run, so like the fillet seed this reds some runs of the 1e-6 row and not others. The regression file was not kept. Pin the two values above as a deterministic row first, then root-cause the escalation: either the arrival really is in the line × circle band at that ε and the row's claim is too strong, or the verifier reads a straight arrival off an arc departure with more error than it has.

---
id: a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle
kind: issue
title: path_property's straight-arrival-off-an-arc row escalates in the non-adjacent segment-pair check's carrier_line_circle at eps 1e-6 (proptest find, red on main)
status: closed
opened: 2026-10-07
closed: 2026-10-07
priority: P0
cost: M
refs: [validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc]
---


Found by the fillet-radius P0 lane (PR #4259) at `CAD_TOLERANCE_EPS=1e-6`. The lane confirmed the same red with main's own `crates/profile/src/sugar.rs`, so the fillet change did not cause it.

- **The row:** `crates/profile/tests/path_property.rs:1496`, `a_straight_arrival_off_an_arc_departure_rides_the_authored_ray`, a proptest.
- **The drawn case:** `h = 1.014505150367573`, `r = 0.47328623976475265`.
- **The failure:** validation's non-adjacent segment-pair contact check escalates instead of building: `EscalationSite::SegmentPair(seg 1, seg 3)`, predicate `carrier_line_circle`, margin −5.567e-6 at ε = 1e-6 (inside the band [ε, kε)). It is not the junction verifier.
- **Where it holds:** the same case is green at the default ε, and the escalation is identical with main's own `crates/profile/src/sugar.rs` (both measured by the PR #4259 review).

The proptest draws fresh cases per run, so like the fillet seed this reds some runs of the 1e-6 row and not others. The regression file was not kept. Pin the two values above as a deterministic row first, then root-cause the escalation: either segments 1 and 3 really sit within the line × circle band of each other at that ε and the row's claim is too strong there, or the path places one of them with more error than it has.

## Outcome

Neither of the two readings above. The path places every segment to
the ulp; the validator asked a carrier question the segments did not
need answered.

- **The drawn case.** Segment 1 is the fillet arc, segment 3 the
  closing chord `(-3, h) → (5, 0)`. Their carriers miss by
  5.5673001612e-6 (exact arithmetic; validation read
  -5.5673001613e-6, so the placement error is about 3e-17), at the
  point of the fillet circle facing the chord, at -97.2° about its
  centre. The fillet arc spans 6.9° to 90°, and the two segments are
  0.588 apart. `seg::line_arc` decided `carrier_line_circle` first and
  escalated in band without asking the arc.
- **A second draw**, found by the proptest at 200000 cases at 1e-6:
  `h = 3.9999985436786116, r = 0.1`. The arrival side's carrier
  crosses the radius-5 circle 1.94e-6 past the side's end, in band of
  `line_span`, at 126.9° where the departure arc (0° to 52.7°) is
  not. `seg::joint` read the line's span with `?` before the arc's.
- **The fix**, in `crates/profile/src/seg.rs`: an in-band
  `carrier_line_circle` is no contact where `arc_clear_of_carrier`
  certifies the arc clear of the whole line (its span definitely
  excludes the point facing the line, both endpoints definitely on the
  centre's side); `joint` answers no contact on either span's definite
  miss before an indeterminate one escalates.
- **Rows:** `path_property`'s two route-3 rows (the drawn case; the
  second draw with an ε-relative twin) and `rejections`'
  `an_in_band_line_circle_clearance_escalates_only_where_the_arc_holds_the_graze`.
- **Siblings** the same shape still escalates on (line × line, line ×
  circle off the line's span, circle × circle) are filed as
  `validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`.


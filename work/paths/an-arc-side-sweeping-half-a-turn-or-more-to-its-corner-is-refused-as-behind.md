---
id: an-arc-side-sweeping-half-a-turn-or-more-to-its-corner-is-refused-as-behind
kind: issue
title: an arc side that sweeps half a turn or more to its corner is refused BehindIncomingRay, a cap no clause states
status: open
opened: 2026-10-13
priority: P2
cost: M
refs: [the-period-jump-has-no-ruling]
---


Found by the PCERT design fork on `the-period-jump-has-no-ruling`
(both designers, executed; 2026-10-13).

## What happens

`profile::path::arc_fillet`'s `advance_gate` and `reach_gate` read an
arc side's sweep to its corner through `profile::sugar::signed_swept`,
which folds into `[−π, π)`. A corner more than half a turn along the
arc reads negative, "behind", although a circle has no behind: only a
ray does. Repro:

```
Open.arc_fillet(Center { c: (0,0), winding: Cw, p: (0,2) }, 0.2)
    .toward(1, -1)
    .to((x, -3))
```

`x = 1.0` (the corner at half a turn), `0.9` and `0.5` (past half a
turn) give `NoCornerOfPair`, both corners `BehindIncomingRay`; `x = 1.1`
and `1.5` (under half a turn) build. No PATHS-DESIGN clause caps an arc
side's sweep. The fillet setback reads the same way: a tangent point more
than π along a leg longer than π reads as behind.

## What closes it

The gates read the forward sweep in `[0, τ)` on a circular side and
refuse only a zero sweep (the corner on the anchor), as a ray refuses a
corner at its start; the S8 ladder chooses between two admitted
corners. The setback folds about the leg's middle (`m + centred(d − m)`,
`m = extent/2`), its branch a `whole_period_count` literal at the leg's
radius. This is the answer the period-jump `[ev]` PR puts to Ev for
group 3, so it waits on that ruling; the alternative there is to ratify
an "at most half a turn" cap in PATHS-DESIGN.

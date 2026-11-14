---
id: a-steep-ellipses-band-apart-pose-stops-at-the-merges-pcurve-span
kind: issue
title: The steep ellipse's band-apart pose (k = 10) stops at the merge's pcurve span check once its join order is freed
status: open
opened: 2026-11-10
---


Found porting `join::first_pair` onto the decided argmin (`onecopy/first-pair-all-mutual`). That port frees an all-mutual round's in-band pair order, at any `K` and on conic germs.

## What

`sweep::all band_apart_partners_on_a_steep_ellipse` builds two partners `24ε` apart on a steep ellipse's flank (`crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs`).

At `k = 10` the join's pair order reads in band (`bool_join_nearest`). Before the port, `first_pair` escalated there, because its free lane excluded conic germs. The round is all mutual, so the order is now free: the matching is the round's in any order (the theorem on `work/onecopy/an-argmin-by-decided-comparison-is-spelled-three-ways.md`).

The run then reaches the finish. All six ops (∪, ∩ and ∖, both operand orders) stop at the coplanar merge's pcurve mint:
- `Merge(Pcurve { Certify { Escalated { check: ParamSpan, predicate: "pcurve_interval_forward" } } })`;
- the margin is `3.394e-9`, the same at default ε and at `1e-12`.

That is an escalation at a real door, not a wrong body. But the pose's doc expected it to build once the pair order was fixed, and it does not.

`k = 60` is unchanged: `bool_join_arc_clear`, a coincidence of the join.

## What closing it takes

Read why the merged half-edge's pcurve span reads in band at `pcurve_interval_forward` on this pose. It may be the scale-moving door of `work/sect/a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale.md`, met one stage later, or the merge's own pcurve derivation on a short arc. Then either build the pose or name the door's limit, and move the row's `k = 10` expectation with it.

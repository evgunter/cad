---
id: a-feature-inside-a-turns-overrun-is-not-carved
kind: issue
title: blend: a bore or bar wholly inside a turn's overrun refuses, since the overrun's sliver carries no crease, and no item schedules its carve
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

Ev's band ruling (PR 4963) defines a band's result as `S − R` (convex)
or `S ∪ F` (concave), so a through-feature wholly inside the material a
turn's overrun removes or fills goes with it. It is not built. In
`blend_surgery` (`crates/sweep/src/blend/surgery.rs`, the `CapMeter`
list) a cut-off's sliver is `SliverOrigin::CutOff { crease }`, and a
cycle the end-face meter reads inside it becomes a consumable
`reach::CapRing`. A turn's overrun is `SliverOrigin::Overrun`, which
carries no crease, so a cycle read inside it goes to
`Unclear::unconsumed_inside` and the request refuses.

The reason is in `crates/sweep/README.md`, the planar cut-off clause:
the overrun's sliver is read through the far band's triangle, a bound
larger than the overrun, so an inside reading there does not certify a
cycle inside the overrun. Pinned by
`band_turn_overrun_chamfer::a_slot_through_the_overrun_is_never_consumed`.

Found by the review of PR 5099 (step 2 of the ruling: the concave bar).
The ruling's build order (`a-concave-fill-and-a-whole-shell-inside-a-bands-region`,
steps 3 and 4) does not name it.

## Fix direction

Read the overrun's own region (the quadrilateral, or its round twin)
instead of the far band's whole sliver, so an inside reading certifies
the overrun. Then give the overrun its crease, and check that the
carve's prism reading holds over the mitre-cut prism. See also
`a-turns-overrun-meters-the-whole-cut-off-sliver`, which measured the
same bound's cost on the refusal side.

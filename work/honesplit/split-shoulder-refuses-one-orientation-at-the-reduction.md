---
id: split-shoulder-refuses-one-orientation-at-the-reduction
kind: issue
title: A split tangent to a rounded shoulder cuts under one plane orientation and refuses ConsecutiveOnSectors under the other
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Found by the GATHER wedge-end audit's fix pass (PR 3373), on
`crates/sweep/tests/wedge_end_doors.rs`'s rounded-shoulder fixture.

**The fixture.** The profile is `(-1,0) (2,0) (2,2) (0,2) (0,1)`, with
the last segment a CCW quarter arc centred at the origin, running back
to `(-1,0)`. It is extruded by 1. The split plane passes through
`(0,1,0)`, where it is tangent to the arc wall along the ruling through
the vertex `(0,1)`.

**What happens:**
- With normal `(0,-1,0)` the split cuts. Both pieces pass
  `validate_geometric`, and the cut mints one π-seam edge on the
  ruling. That behaviour is pinned as
  `a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`.
- With normal `(0,+1,0)` the split refuses
  `SplitReduceError::ConsecutiveOnSectors` at the vertex on the
  ruling. That is `splitting::rules::apply_rule_b`'s invariant check,
  so the refusal comes before the section is joined.

**Why it matters.** `topo::split`'s docs state the mirror identity
`split(S, n) ≡ swap(split(S, −n))`. The same cut therefore succeeds
under one orientation and refuses under the other. The refusal is
typed, so no bad body ships.

**Probably where to look.** Rule (a) runs the tangent-contact descent
(`tangent_sector_osculation`) and then continues. Under this
orientation it leaves two consecutive ON entries at the ruling vertex,
and rule (b) does not handle that.

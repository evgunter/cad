---
id: a-glued-slit-intersection-refuses-a-classification-invariant-since-5505
kind: issue
title: Since PR 5505, an intersection of a box flush with a slit plate's top refuses ClassificationInvariant where it refused SlitFaceGlued typed
status: closed
opened: 2026-10-21
priority: P1
cost: M
branch: cleave/glued-slit-intersection-regression
closed: 2026-10-21
---


Found by PR 5479's re-review on main `92791b6500`. It holds at every ε, at rest and turned.

**Repro.**
- `x = posed_box([1,2]×[0.5,1.5]×[0.3,1])`
- `y = plate − apex_pyramid(below(lying(0)))`
- `intersect(x, y)`

**Before PR 5505:** typed `SlitFaceGlued` (no lever).

**Since PR 5505:** `ClassificationInvariant` ("a fate cut's end lies in no corner of the dropped face"). That message reads as a kernel defect.

`x − y` builds correctly on the same pair, through `merge_slivers`, the same on main.

**Owed.**
- Find which of 5505's three changes reaches the fate cut here: `glued_slit_roles`, the sliver-Out reading, or `merge_slivers`.
- Either build the intersection correctly, or restore a typed construction-gap refusal before the fate stage. A false defect ending is a regression of 5505's.
- Pin the case at three ε.

## Found (branch `cleave/glued-slit-intersection-regression`)

It is 5505's ring order and sliver-Out reading. The box's top is flush,
with the same sense, under the plate's top, which carries the ring.
The join cuts the box's top along the ring, and the sliver reads Out of
`y`. ∩ keeps In, so it drops the sliver and keeps the box's top, with
the ring in it. The cut through the dropped plate top then ends at the
ring's tip, which the fate cut does not read (`fate::cut`, "a fate
cut's end lies in no corner of the dropped face"). `merge_slivers` does
not reach it.

5505's construction is complete only where the op keeps the sliver:
there the output stage merges the slit's two walls across it. So the
sliver-Out reading now refuses `SlitFaceGlued` where the op drops the
sliver, before the fate stage (`join.rs` `resolve_roles_geometric`).

Measured at rest and turned, the box `[1,2]×[0.5,1.5]×[lo,1]` with
`lo ∈ {0, 0.3, 0.6}`, all six ops:

| op | before 5505 (`df1ed8d476`) | main | branch |
|---|---|---|---|
| `x ∩ y` | `SlitFaceGlued` | `ClassificationInvariant` | `SlitFaceGlued` |
| `x − y`, `y ∪ x` | `SlitFaceGlued` | builds | builds |
| `y ∩ x` | `ClassificationInvariant` ("a fate cut leaves a slit in the dropped face") | the same | the same |
| `x ∪ y` | `ClassificationInvariant` ("…end lies in no corner…") | the same | the same |
| `y − x` | builds | builds | builds |

`y ∩ x` and `x ∪ y` predate 5505 and none of its rules reaches them.
They are FLUSH's fate cut at a slit's tip; the evidence is on
`work/flush/a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab.md`.
The box raised off the top (`hi = 1.4`) or sunk under it (`hi = 0.99`)
builds every op, as on main, except `x − y` at `hi = 1.4`, which
refuses `LineWedges` before 5505 too.

Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_box_flush_under_a_ringed_top_builds_in_every_op` (named `…_kept_once_refuses_typed` then)
at rest and turned, CI's three ε.

## Since `cleave/fate-cut-ends-at-a-slit-tip`

The fate cut reads a slit's tip as its face's full turn, so the guard
above is gone: all six ops build at every `lo`, at rest and turned,
tier 3 and 3′ at their closed-form volumes
(`a_box_flush_under_a_ringed_top_builds_in_every_op`). The evidence is
on `work/cleave/a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line.md`.

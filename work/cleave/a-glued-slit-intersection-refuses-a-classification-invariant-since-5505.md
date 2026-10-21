---
id: a-glued-slit-intersection-refuses-a-classification-invariant-since-5505
kind: issue
title: Since PR 5505, an intersection of a box flush with a slit plate's top refuses ClassificationInvariant where it refused SlitFaceGlued typed
status: open
opened: 2026-10-21
priority: P1
cost: M
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

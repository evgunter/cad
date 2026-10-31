---
id: main-red-two-tori-subtract-refuses-on-tangent-carriers-not-on-no-section-classification
kind: issue
title: main red: germ_torus_doors' two-tori row refuses its subtract on tangent or near-tangent carriers, where it pins 'has no section classification'
status: closed
opened: 2026-10-28
priority: P1
cost: E
closed: 2026-10-28
---

Found by SPLICE's rim-exact-image lane while running sweep's full slow
set. It is red on clean `origin/main` at `003915b5` with
`--all-features`, by the same name, so the lane's branch did not cause
it. Nothing on the board recorded it. Hosted CI does not run this
slow-set row, so it should show in the nightly.

## What

`sweep::all germ_torus_doors::subtract_and_intersect_refuse_where_union_does`
panics at its assertion (`germ_torus_doors.rs`, the `what.contains(says)`
check). The row `("two tori", &d, &far, "has no section classification")`
gets

`two tori, A ∖ B: a curved face and a face of the other solid have
tangent or near-tangent carriers and no crossing layer saw an event; the
section's component count is not certified there`

The refusal is still `FallbackExtentUnsupported`, but its words are the
`dumbbell` row's.

## What is open

Unmeasured: which merge moved it. The likely candidate, not bisected,
is main's latest code merge, PR 5886 (`sect/torus-certificate`). Only
its branch name ties it to tori; the lane did not read its diff. Then decide whether the new
refusal is the right one for two tori that do not cross, and re-pin or
fix accordingly.


## Closed

Fixed by SECT, PR 5925 (`f56c009fca`). SECT bisected the red to its PR 5869, not 5886: the parallel-tori arm refused the donut pair on an exact tie in its own construction. The kernel is fixed and the row is re-pinned to its honest R-loop classification. `germ_torus_doors::subtract_and_intersect_refuse_where_union_does` is green on main.

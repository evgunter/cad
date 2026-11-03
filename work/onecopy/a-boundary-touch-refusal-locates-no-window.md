---
id: a-boundary-touch-refusal-locates-no-window
kind: issue
title: The boundary pass's Touch refusal carries only an Indeterminate and no located window, so boundary_cuts refuses a true contact a located window would settle
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [6451]
---


Found by the delta review of PR 6451 (`onecopy/curve-pair-reads`, head `43391c2fab`).

PR 6451 removed `boundary_cuts`' schedule fallback, which answered a silent `Ok(false)` where the boundary pass declined: a P0 on main, reproduced in the review. In its place a face whose boundary pass refuses now refuses typed (`FaceUnread::Boundary`). That is loud and never wrong. But it refuses a real contact that main answered correctly. Example: an unknown-cuts face U with its spike at x=3 and a wide known face K; the oracle is true, and the reading is `Err`.

The refusal exists because `geom_brep`'s `Touch` / `RootRefusal::Touch` carries only an `Indeterminate`. No finder locates the in-band window, so `boundary_cuts` has no cut to offer. That cut would be the two ends of the window, which is what the old double root gave in effect.

Fix: make the boundary pass's touch refusal carry the parameter window it is undecided over (the finder knows where it was looking), and have `boundary_cuts` take its ends as cuts. Then the reading is exact where the touch is located, and refuses only where it is not.

Related: `census.rs` `verdict` counts any `Err` as agreement, so the `d*` rows cannot catch a refusal that replaced a correct answer. Give this row's fix a pin that asserts `Ok(true)`.

---
id: loft-parameters-count-a-corner-the-body-pins-as-moving
kind: issue
title: loft_parameters reads a corner pinned only up to rounding as moving, where loft_body writes it one point and its row abstains
status: open
opened: 2026-11-22
priority: P3
cost: E
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


Filed by CHARTTAIL's `charttail/collapsed-column` lane, which made the
loft door write a corner it decides pinned as one stored point before
the skin (`crates/sweep/src/skin.rs`, `pin_strips`). The row of one
stored point then abstains from Eq. 10.8 (`chord_length_parameters`).
`sweep::loft_parameters` runs no door and reads no band, so a corner
whose two placements differ only by rounding counts there as moving.
Its chord shares are the ratios of that rounding, averaged in. On such
a loft the door's answer and `loft_body`'s `section_params` differ.
Its docs used to call them "literally the computation `loft_body`
runs". They now say where they part.

## Fix

Run the door's corner classes in `loft_parameters` (it holds the
tolerance), or reword its contract to promise agreement only on lofts
with no pinned corner. The first keeps its read-back promise whole.

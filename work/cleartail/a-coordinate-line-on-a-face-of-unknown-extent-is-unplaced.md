---
id: a-coordinate-line-on-a-face-of-unknown-extent-is-unplaced
kind: issue
title: A coordinate line on the periodic axis of a face whose outer loop has a curved row has no sheet, so departure across it refuses at depth 0
status: open
opened: 2026-10-29
priority: P2
cost: M
---

## Finding

From CURVEDEP's sweep (`departure-presents-no-curved-shared-edge`).

`present_chart` places a coordinate line held on a periodic axis on the face's sheet (`placed`), and the sheet comes from the face's extent (`sheets`).
- `Boundary::extent` is `None` whenever a row of the outer loop is not a monotone coordinate line (`clearance.rs`, `extent_of`). Every face whose outer loop holds a curved edge is one.
- On such a face `placed` answers `None` for a line of constant azimuth, and `present_chart` returns `Unpresented::Structural` (`crates/topo/src/injective.rs`, `present_chart`'s "A line on a periodic axis whose sheet is unknown has no place").

So departure across a ruling of such a face refuses at depth 0. One example is a cylinder band cut off by an oblique plane between two rulings, the shape `fillet_h7_transverse_cap` builds by its own description (read from the code, not run here): its ellipse row leaves the extent unknown.

## What a fix needs

The curved presentation reads its curve on the chart's cover, every lift by whole turns, and asks that exactly one lift reach the cell. The same reading places a coordinate line with no sheet: take every lift of the line that meets the cell and require one.

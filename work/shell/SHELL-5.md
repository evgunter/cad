---
id: SHELL-5
kind: unit
title: shell of a hollow body thickens every boundary — one thin solid per operand shell
status: dispatched
opened: 2026-09-08
branch: shell/5-hollow-operand
refs: [shell-of-hollow-body-thicken-every-boundary, 1056, 1048]
---


Ev's ruling on issue record 1056, verbatim in
`shell-of-hollow-body-thicken-every-boundary`: *thicken every
boundary — offsetting only the outer shell is explicitly rejected.*
`shell` / `shell_open` on an operand whose one solid carries void
shells erode the outer shell inward and dilate every void outward by
`t`, and return one thin solid per operand shell (`k + 1` solids,
`2(k + 1)` shells); `OperandAlreadyHollow` is retired; the record
gains `thickened` (result solid ← operand shell). Spec
`docs/SHELL-5-SPEC.md`. Pre-draw difficulty **M**, task class
**STRUCTURAL** (one ownership re-partition op in `topo`, the
construction otherwise unchanged; the numbers are closed forms of
existing fixtures) — logged AFTER block SHELL-B1's byte was drawn
(the block was drawn at SHELL-1's dispatch), so the covariate is
contaminated for this row and is disclosed as such.

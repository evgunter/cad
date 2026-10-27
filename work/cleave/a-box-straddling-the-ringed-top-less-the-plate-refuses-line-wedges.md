---
id: a-box-straddling-the-ringed-top-less-the-plate-refuses-line-wedges
kind: issue
title: A box straddling the ringed top, less the plate, refuses LineWedges Overlap
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges]
---

Found by the flush-partners-at-a-slit lane (`cleave/flush-partners-at-a-slit-build`),
the same on main `be2b7a139c`, at ε default, 1e-6 and 1e-12, at rest,
turned, and flipped and turned.

## What

`y` is the plate `PLATE` less the pyramid lying under its top
(`apex_pyramid(below(lying(0)))`), its top carrying the pyramid's ray
as a zero-area ring from `MEET` to `MEET + bearing(60, 0.5, 0)`. `x` is
the box `[1, 2] × [0.5, 1.5] × [0.3, 1.5]` straddling the top over the
whole ring. `x − y` is the box's part above the top and the void, which
touch along the ring's line: the void's wedge below the line and a
face of the upper part continuing across it. It refuses
`LineWedges { fault: Overlap }` at the result gate (`boolean/ops.rs`,
`gate`): the result holds coincident edges on the line whose wedges
overlap. The other five ops build, tier 3 and 3′, as the closed form
says.

`LineWedges` is the typed refusal of a re-pairing the kernel has not
built, so this is a construction gap, not a wrong body.

## Owed

Build `x − y` as the material decides: one edge, the void's crease,
touching the upper part's face along the line, or refuse it before the
gate.


---
id: a-ring-partner-inside-the-ring-unions-result-invalid
kind: issue
title: The plate with a ring ∪ a flush partner whose edge lies wholly inside the ring, plate first, refuses ResultInvalid
status: open
opened: 2026-10-25
priority: P1
cost: M
---


Found by PR 5545's review probe (`cleave/fate-cut-ends-at-a-slit-tip`);
the same on main `42af7c0b48` and on the branch, at ε default, 1e-6
and 1e-12, at rest, turned, and flipped and turned.

## What

`y` is the plate less the pyramid lying under its top, its top carrying
the ray from `MEET` along bearing 60 as a zero-area ring of length 0.5.
`x` is the tetrahedron with apex `MEET + bearing(60, 0.1, 0)` over
`[bearing(60, 0.3, 0), bearing(150, 0.6, 0), bearing(20, 0.6, −0.7)]`
(and its mirror, `bearing(−30, 0.6, 0)`, `bearing(100, 0.6, −0.7)`), so
its edge along the ray, from 0.1 to 0.4, lies wholly inside the ring,
and one of its faces lies flush with the top. `y ∪ x` refuses
`ResultInvalid`; the other five ops build, tier 3 and 3′.
`ResultInvalid` claims a kernel defect.

## Owed

Build it, or refuse it typed before the result gate.

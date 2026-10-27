---
id: a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction
kind: issue
title: A box flush under the ringed top with a corner or edge at the ring's tip refuses ClassificationInvariant: two crossing germs of a vertex pair lie along one direction in one sector entry
status: dispatched
opened: 2026-10-25
priority: P1
cost: M
branch: cleave/flush-partners-at-a-slit-build
---


Found by PR 5545's review probe (`cleave/fate-cut-ends-at-a-slit-tip`);
the same on main `42af7c0b48` and on the branch, at ε default, 1e-6
and 1e-12, at rest, turned, and flipped and turned.

## What

`y` is the plate `PLATE` less the pyramid lying under its top
(`apex_pyramid(below(lying(0)))`), its top carrying the pyramid's ray
as a zero-area ring from `MEET` to `tip = MEET + bearing(60, 0.5, 0)`.
`x` is a box flush under the top (`[.., ..] × [.., ..] × [0.3, 1]`) or
standing on it (`× [1, 1.5]`) with a corner or edge at the ring's tip:

| box | refuses |
|---|---|
| `[1, tip.x] × [0.5, tip.y] × [0.3, 1]` (corner at the tip) | `x − y`, `x ∩ y`, `y ∪ x` |
| `[1, tip.x] × [0.5, 1.5] × [0.3, 1]` (edge through the tip) | `x − y`, `x ∩ y`, `y ∪ x` |
| `[1, tip.x] × [0.5, tip.y] × [1, 1.5]` (above, corner at the tip) | `x ∪ y`, `y ∪ x` |

each `ClassificationInvariant` "two crossing germs of a vertex pair lie
along one direction in one sector entry" (`boolean/recl.rs`), a claimed
kernel defect. The box's vertex pairs with the ring's tip, where the
top's corner is the full turn round the tip.

## Owed

Build these, or refuse them typed as a construction gap.

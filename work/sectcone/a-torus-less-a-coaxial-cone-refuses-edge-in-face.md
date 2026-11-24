---
id: a-torus-less-a-coaxial-cone-refuses-edge-in-face
kind: issue
title: The torus less a coaxial solid cone crossing its tube refuses EdgeInFace, while the five other runs of the pair build
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by PR 7395's fix pass, in a row the dual review asked for.

## Measured

`crates/sweep/tests/a_cone_on_a_torus_meridian_in_every_order.rs`
`a_coaxial_cone_against_the_torus_ends_as_pinned` uses the torus
`R = 2`, `r = 1` about `y` and the solid cone `(0, 0), (3, 0), (0, 3)`
revolved about `y`. The cone's wall crosses the tube, and its base
disk lies in the torus's equator plane. In CI on PR 7395 (head
`7362e37602`), at the default ε, 1e-6 and 1e-12 alike:

- ∪ and ∩ build in both orders;
- cone ∖ torus builds;
- torus ∖ cone refuses `BooleanError::EdgeInFace`.

The row pins that outcome. The `EdgeInFaceGap` was not read: the pin
records only the variant. That gap says whether this is a parting not
built (a limit) or the kernel's own defect.

## What the taker owes

Read the gap. If it is a defect, fix it. If it is a parting not built,
name the parting and file it where the parting lives. Then re-pin the
row to the outcome, and hold a built ∖ to an independent volume, as
the meridian row's ray oracle does.

---
id: a-turned-lens-keeps-the-door
kind: issue
title: The turned lens builds only with its discs declared Rest, which D10 retires
status: parked
opened: 2026-10-02
priority: P1
cost: M
blocked_on: [intent-stage4-is-built]
---

## What

The lens of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
(`a_lens_of_two_domes_builds_with_its_discs_declared_rest`): the dome of
radius `√2` standing on its rim circle, unioned with the bowl that
mirrors it. Turned about the axis by `π/7` or `π/2`, it builds since
PR 4148 (`tang/lying-on-arc-splits-at-a-ruling`). Both member orders give
`2 · V(dome)` to `1e-15`, tiers 3 and 3′ pass, the census is
`(4, 8, 6, 1)`, and there are no contact records. The row pins it.

What remains is the declaration. The build needs the two coincident
discs declared `Rest`. Undeclared, the turned lens refuses
`UndeclaredCoincidence` on the discs at every turn, the unturned lens
included. D10 retires the declared-contact seats, so this row waits on
it: once D10 lands, the discs' coincidence is said its way, and the row
moves to that.

## How the crossing layer passes it

Each dome rim semicircle lies on the bowl's sphere and runs along parts
of two bowl arcs. `reduce::lying_on`'s certificates (a) and (b) decline
on the whole semicircle. The interior question that PR 4148 added splits
it at the bowl's rim vertex. One half is then answered by (b), its ends
paired along a chain of the bowl's arcs. The other half is answered by
the interior question's certified absence (measured by the PR's
reviewer, with instrumentation).

Measured again on 2026-10-08 (TANG, turned-hemisphere sweep). With the
discs declared `Rest`, it builds in both orders at 0.000003°, 0.0001°,
0.01°, 0.03°, 7°, 30°, 45°, 90°, 173° and 359.99°, at `2·V(dome)`.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: it already builds; what remains is the discs' Rest declaration and the UndeclaredCoincidence refusal without it, both retired when booleans glue on Zero. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

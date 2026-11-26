---
id: a-revolved-cone-cut-along-two-rulings-through-its-apex-refuses-the-pinch
kind: issue
title: A revolved cone cut along two rulings through its apex refuses ResultPinchedAtConeApex where main built it
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7585
---

Found by the LATHE one-band lane (2026-11-23, PR 7414's review fix pass,
the slow set's base/head diff).

## What

`crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`,
`a_cut_along_two_rulings_through_a_wrap_tipped_apex_refuses_the_pinch_by_name`:
the box `[−0.3, 0] × [0.95, 1.2] × [−0.3, 0.3]`, its face `x = 0`
through the apex, against the full-revolve cone of half angle 15°, 30°
and 45° (`revolved(alpha)`). On main (two π-bands) the revolve's
second meridian parted the two lobes the cut leaves into two faces, and
`c ∪ b`, `b ∪ c` and `c ∖ b` built (the box holding half the cone's tip
of height `0.05`, read by volume and point membership). With the wall
one band (PR 7414) the cut leaves ONE face whose loop is two lobes
meeting only at the apex, and each op refuses
`ResultPinchedAtConeApex`, as the tipped cone always did. A capability
regression against main.

## Cause

Props cannot measure a cone loop pinched at its apex
(`work/spiricarm/props-measures-a-cone-loop-pinched-at-its-apex.md`),
and the Boolean refuses such a result before the gate. The tipped cone
was the only witness; the one-band revolve makes every full-revolve
cone one.

## Wanted

The pinched loop measured (the spiricarm row), which builds both cones;
or the result split at the apex into its two lobes before the gate.
Then the row's revolved arm returns to its closed forms (`vc + vb − tip`,
`vc − tip`, the region lattice).

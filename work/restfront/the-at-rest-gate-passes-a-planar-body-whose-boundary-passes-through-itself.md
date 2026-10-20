---
id: the-at-rest-gate-passes-a-planar-body-whose-boundary-passes-through-itself
kind: issue
title: The at-rest gate passes a planar body whose boundary passes through itself where no aimed ray's closest crossing enters the region it winds −1
status: open
opened: 2026-10-20
refs: [SHELL-3, tier3-never-checks-a-planar-loop-is-simple, a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass]
priority: P3
cost: H
---


Found by FLUSH's fin-crown unit (`flush/fin-crown-winding`), at ε 1e-9.
Filed here because the gate's battery is `validate.rs`'s.

## What happens

`test_support_meeting::crown_body(&fin(), [0.0, 0.2, -0.6], pose, tol)`,
the fin crown on its old bottom, is no solid. The bottom fan's face on
the fin's long edge `c0 c1` leans across the slit and cuts through the
fin's far face (`c1 c2 MEET`) and the top face beyond it
(`c2 c3 MEET`). An exact rational segment–triangle test on its
triangles finds three crossing pairs. A generalised winding number
(`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`) reads −1 near
the slit's mouth.

Check 7 now reads every aim that decides, so the gate refuses this
crown (`OrientationOpposed`) at three poses of five (`at rest`,
`turned about z`, `turned`). At the two flipped poses every aimed ray
reads `Out` and the crown passes tier 3, because no ray's closest
crossing enters the thin −1 region (about 4.6e-7 across).
`a_crown_wound_against_itself_refuses_at_rest_by_two_opposed_rays`
pins the three poses.

Check 7 reads a side and is no self-intersection test, and no other
check of the gate refuses the crown at those two poses. The README's tier-3 paragraph
(`crates/topo/README.md`, "Global self-intersection / minimum clearance
is the interval clearance engine's") gives that to the clearance
engine, whose body half is still in `editor-core`
(`work/clear/SHELL-3.md`), and the at-rest gate does not run it.

## What to find

Whether the at-rest gate should refuse a boundary whose faces cross
away from the cells they share, and with which instrument: the
clearance engine once it lives in `topo` (SHELL-3), or a cheaper
planar face–face crossing test. The sibling row
`tier3-never-checks-a-planar-loop-is-simple` is the same class at one
face. No verb is known to build such a body; the witness is a fixture.

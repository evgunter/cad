---
id: an-arc-lying-in-a-curved-faces-interior-refuses-at-the-reduction
kind: issue
title: An arc of one operand lying in a curved face's interior of the other refuses CurvedPierceUnsupported at the reduction, so its slit form is never reached
status: closed
closed: 2026-10-31
opened: 2026-10-26
priority: P1
cost: M
---


Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`,
building witnesses for `a-boolean-edge-lying-in-a-curved-face-is-not-read`.
P1 for verb breadth: the geometry is ordinary, a V-groove revolved into a
bore.

## What

A 270° revolve about `y` whose inner wall is the cylinder `r = .5` (or
the sphere `|p| = 2`), against a V revolved 150° about `y` whose tip arc
lies on that wall (`sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`,
`an_arc_resting_in_a_cylinder_or_sphere_face_refuses_typed`). Every op
in both orders, turned 60° or −20°, refuses `CurvedPierceUnsupported`
naming the V's tip arc and the wall, at every ε. The refusal comes from
the reduction's curved arm (`topo::boolean::reduce`, `lying_on` and its
callers, the covered-arc rung at `reduce.rs:2155` and the `(Zero, Zero)` arm at `reduce.rs:2577`):
the arc lies on the wall's carrier with its ends in the wall's interior,
and `lying_on` returns no event, which keeps the frontier.

Past the reduction, the parting reads the arc as lying in the wall
(`topo::boolean::slit::on_carrier`, a conic on a quadric at five points)
and refuses it `EdgeInFace` (`Curved`): two coincident arcs no record
backs, the same gap as the planar curved keel
(`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
So the rider form does not ship either way; the body is never built.

## Fix

Place the arc's ends on the wall at the reduction where its interior is
certified on the carrier, as `lying_on` does a covered arc, so the op
reaches the parting; then the arc's slit form waits on the curved-slit
row above. The witness's pin moves from `CurvedPierceUnsupported` to
`EdgeInFace` (`Curved`) at the first step.

## Closed (2026-10-31, CLEAVE, PR 5793)

Main's reduction now places the witness's arc: merged into PR 5793,
every op of both scenes but `A ∖ B` builds, valid at tiers 3 and 3′
(`B ∖ A` empty where the V turned 60° lies inside the slab), and
`A ∖ B` reaches the parting, which refuses `EdgeInFace` (`Curved`), as
this row's Fix asked. The witness is re-pinned and renamed
(`an_arc_resting_in_a_cylinder_or_sphere_face_refuses_only_its_slit`).
The slit form waits on
`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`.

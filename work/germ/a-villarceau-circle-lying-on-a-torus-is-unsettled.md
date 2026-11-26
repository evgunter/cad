---
id: a-villarceau-circle-lying-on-a-torus-is-unsettled
kind: issue
title: A Villarceau circle lying on a torus is the F≡0 case the circle×torus root door still answers Uncertain
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [a-torus-meridian-lying-on-a-torus-is-unsettled, the-villarceau-identity-certificate-becomes-the-poses-conic-torus-lies-rung]
---


## What

A Villarceau circle of a torus (radius `R`, centred `r` from the
torus centre in a bitangent plane tilted `asin(r/R)` off the
midplane) lies on the torus exactly, so `F ≡ 0` along it. The circle ×
torus root door (`geom_brep::curve_face::circle_torus::circle_torus_roots`)
answers it `CircleRoots::Uncertain`: its pole is on the torus at every
anchor, and the ladder cannot read the identically-zero quartic. So
`reduce::curved_face_arm`'s `(Zero, Zero)` arm keeps its frontier
door on an edge along a Villarceau circle that lies on a partner
torus. The row that pins it:
`circle_torus::tests::circles_lying_on_the_torus_are_meridians_or_uncertain`,
its odd poses.

## Why it is reachable now

The pcurve mint images a Villarceau circle on a torus chart
(`Pcurve::FocalSection`, `pcert/torus-villarceau-route`, PR 4227), so
a torus face can be bounded by one.

## What would fix it

A rung like the meridian rung that TANG's
`a-torus-meridian-lying-on-a-torus-is-unsettled` added
(`circle_torus::meridian_deviation`, `circle_torus_meridian`):
one margin bounding every carrier point's distance from the torus,
which is the sum of the point deviation of each Villarceau condition
(the centre's distance `r` from the torus centre in the midplane, the
plane's bitangent tilt, and the radius `R`). Each term needs its own
derivation, because the tilt here is not the meridian's second-order
cost. No consumer reaches it today, which is why this is P3.

Moved from HONEROOT to GERM on 2026-11-20, by `git mv`, when HONEROOT merged into GERM: HONEROOT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GERM carries.

## The pose's general not-`Lies` rung (U3a, PR 7226)

The curve × face pose (`curve_face::pose`) now reads a circle × torus
past its coaxial and meridian rungs through `curve_face_pose_not_lies`:
one span point whose residual is decided past the band, else an
escalation. A Villarceau circle reads every point in the zero band, so
it escalates `InBand(NotLies)` (the gate's rejection of the zero
reading) rather than answering `Uncertain`
(`curve_face::pose_rows::each_rung_escalates_on_itself`). The row still
owes the rung above, under which it would read `Lies`.

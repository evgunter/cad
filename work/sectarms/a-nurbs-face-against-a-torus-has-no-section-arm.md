---
id: a-nurbs-face-against-a-torus-has-no-section-arm
kind: issue
title: A NURBS face against a torus face has no C5 section arm, so lily wall 8 (the leaf grown out of the stem) cannot join once its spline seams cross
status: open
opened: 2026-10-29
priority: P1
cost: H
refs: [a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite]
---


Filed by SPLICE's lily-one-loft lane (2026-10-29), which re-spelled lily
wall 8 as the leaf growing out of the stem (`[ev]` PR 5990, question 3).

## What

Lily wall 8 (`demos/tour/src/lily.rs`, `wall_probes`) unions the lower
stem (`lily_stem`, a `tube_along_arc` torus segment, ring 5, tube 0.06)
with the long leaf (`lily_leaf_a`, one loft of lens sections) where the
scene stands the leaf's base in the tube. The tube's wall crosses the
leaf's NURBS ridge and keel walls in one loop about the base section,
steeply all round (the crossing's sine is 0.95 or more off the authored
numbers), and cuts the inner margin's two tip seams.

Today the union refuses first at the curved crossing lane, a spline
seam of the leaf against the torus wall:
`EdgeCarrierUnsupported { operand: B, face_of: A, site: CurvedCrossing }`
at ε 1e-9 and 1e-6 (SPLICE's
`a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`).
Behind it is this pair's face × face section. C5's table
(`crates/geom-brep/README.md`, the rung list) has rung-3 arms for
plane×NURBS and NURBS×NURBS and the torus pairs with sphere, cylinder
and torus; NURBS×torus is among "every other pair", which refuses.

## Open

Not yet measured, since the crossing lane refuses first: whether the
section is what refuses once SPLICE dispatches the spline span. How
the section is certified is the same open design read as the cone and
sphere row's (`a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`);
the torus certificate and the critical gap (`pair_section::gap`) are the
torus side's existing pieces.

## Covered (2026-10-31)

Ev decided the NURBS × implicit arm on PR 6121
(`a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`, `## Decided`),
and the torus is in its first roster. This row stays open as lily wall 8's
consumer pin, closed by the unit that builds the curved partners.

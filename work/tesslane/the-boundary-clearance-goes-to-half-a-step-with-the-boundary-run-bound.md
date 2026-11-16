---
id: the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound
kind: unit
title: BOUNDARY_CLEARANCE becomes half a step, the value nobody chooses, once a boundary run has its bound
status: parked
opened: 2026-11-16
priority: P2
cost: M
blocked_on: [one-containment-lemma-for-an-oblique-boundary, the-lattice-keeps-clear-of-the-boundary-and-every-winding-is-resolved]
---



Ev's answers on PR 6966 (2026-11-16): "1/4 or something now with a plan for 1/2 later could work", and, shown that 1/4 refuses faces main meshes, "ah sure the specific temporary value doesn't matter". This row is the plan.

## What it is

`trimmed::BOUNDARY_CLEARANCE` is 1/16 of a lattice step, a temporary quality constant (`crates/mesh/README.md`, the constants of `[mesh.sizing]`). At 1/2 the rule needs no number: a lattice node is not laid where the boundary crosses the node's own cell. The unit sets it to 1/2 and removes the constant's "debt" wording from the page.

## Why it waits

A triangle beside the boundary reaches `1 + BOUNDARY_CLEARANCE` steps, and nothing bounds a triangle on a boundary run but the headroom. Measured by the unit that builds the rule, over 550 analytic trimmed faces on main as of 2026-11-16, the worst certificate in cell bounds, where `UNPROVED_HEADROOM` allows 2: 1.81 with no clearance and at 1/16, 1.93 at 1/8, 1.98 at 1/4, where plain cylinders cut by a steep plane refuse `CertificateExceeded`. (The three designers' table, on their own 486 faces, read 1.06 with none and 2.21 at 1/2: their corpus missed the faces main meshes closest to the limit.) `one-containment-lemma-for-an-oblique-boundary` writes the bound as a function of the reach, and then the lattice is sized for a reach of 1.5 steps beside the boundary by the theorem and not by a factor.

The constant is not raised toward 1/2 in steps: each step buys refusals of faces that mesh today and proves nothing.

## Done when

The constant is 1/2 and stated on the page as the rule's own value; the table is re-measured over every trimmed face the suite builds with no face refusing; the goldens that move are named with why each new mesh is right.

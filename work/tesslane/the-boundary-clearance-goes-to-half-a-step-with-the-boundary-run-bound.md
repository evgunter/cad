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



Ev's answer on PR 6966 (2026-11-16): "1/4 or something now with a plan for 1/2 later could work". This row is the plan.

## What it is

`trimmed::BOUNDARY_CLEARANCE` is 1/4 of a lattice step, a named quality constant (`crates/mesh/README.md`, the constants of `[mesh.sizing]`). At 1/2 the rule needs no number: a lattice node is not laid where the boundary crosses the node's own cell. The unit sets it to 1/2 and removes the constant's "debt" wording from the page.

## Why it waits

Measured over 486 analytic trimmed faces (`work/tesslane/log.md`, 2026-11-14, "Design fork, three designers"), the worst certificate in cell bounds, where `UNPROVED_HEADROOM` allows 2: 1.06 with no clearance, 1.22 at 1/16, 1.39 at 1/8, 1.66 at 1/4, 1.98 at 3/8, 2.21 at 1/2 with six faces refusing `CertificateExceeded`; a second designer's 50 meshes refuse seven at 1/2. A triangle beside the boundary reaches `1 + BOUNDARY_CLEARANCE` steps, and nothing bounds a triangle on a boundary run but the headroom. `one-containment-lemma-for-an-oblique-boundary` writes that bound as a function of the reach, and then the lattice is sized for a reach of 1.5 steps beside the boundary by the theorem and not by a factor.

## Done when

The constant is 1/2 and stated on the page as the rule's own value; the 486-face table is re-measured with no face refusing; the goldens that move are named with why each new mesh is right.

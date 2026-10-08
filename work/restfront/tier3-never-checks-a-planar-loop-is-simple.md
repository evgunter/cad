---
id: tier3-never-checks-a-planar-loop-is-simple
kind: issue
title: tier 3 never checks a planar loop is simple - a section face whose loop runs through its own vertex passes every tier
status: open
opened: 2026-10-01
---


Found by the REACH plane × cone split fix pass (PR #3688, 2026-10-01).

## What happens

`topo::validate_geometric` checks a planar face's loop winding sign
(check 6) and its rings' nesting (check 9), but never that the loop is
SIMPLE — that its edges meet only at the vertices they share. A loop
that passes through itself bounds a region of winding 2 somewhere, and
every other tier-3 check reads it as an ordinary face.

## Evidence

Before the split's apex-window fix, the upright cone (`revolve` of the
triangle `(0,0), (1,0), (0,1)` about `y`) cut through `(0, 0.4, 0)` with
normal `(sin 0.5, cos 0.5, 0)` gave a lower half of volume 1.0815, more
than the whole cone (`π/3`), which passed tiers 1, 2 and 3. Its planar
section face carried two edges on one ellipse with parameters
`(π, 7.539)` and `(1.256, π)` — together the whole ellipse — plus a line
chord: the loop ran through its own vertex `(0.732, 0, 0.681)` in the
middle of the first ellipse edge. The split defect is fixed
(`chord_join::run_azimuth_window`); this check would have caught it.

## Acceptance

- Tier 3 refuses, with a typed `ValidationError` naming the face and
  the two edges, a planar face whose loop has two edges meeting
  anywhere but a shared end vertex (edge interior against edge or
  vertex), on line, circle and ellipse carriers; an in-band contact
  escalates.
- A row builds the self-touching section face above (or the same loop
  by hand) and sees the refusal; the shipped corpus stays green.
- Spline and spiric loop edges are named as the check's residue.

Related: the curved-surface half of the same statement is
`validate-tier3-curved-boundary-containment`.

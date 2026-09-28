---
id: torus-onto-the-subtract-and-intersect-roster
kind: issue
title: Torus onto revert_arm_exists: subtract and intersect still refuse a torus operand at the front door
status: dispatched
opened: 2026-09-25
refs: [torus-operand-gate-admission]
priority: P2
cost: D
branch: germ/torus-ops-and-chord
---

## What

`reduce.rs`'s `boolean_arm_exists` has a torus now: the union's
crossing layer has its torus arms (circle rung with carrier identity,
line×torus quartic, chart containment, sector and pierce normals).
`revert_arm_exists`, the roster ∖ and ∩ read at the front door
(`ops`), still lists `Plane | Cylinder | Sphere`, so a torus operand
under `Subtract` or `Intersect` refuses before any of those arms runs.
`crates/sweep/tests/mate7a_torus_rest.rs` pins it ("∖ and ∩ keep their
roster verbatim").

What admitting it needs is unmeasured. The seam lane ∖ and ∩ run
(the revert of the other operand and its seam) has not been walked
with a torus operand. Measure first, as the union's doors were.

## Measured (GERM torus-ops PR, 2026-09-28)

With `Torus` added to `revert_arm_exists` in a scratch tree, ∖ (both
operand orders) and ∩ were run over the shapes PR 3265's reviewers used
against ∪, plus the ∪ fixtures that have torus faces:

- a bar through the tube (near-perpendicular, belly, chord across the
  hole; 60 seeded random rods under ∩): `CurvedSectorSideUnsupported`, the
  sagitta charge ∪ stops at, or `CurvedPierceUnsupported` /
  `FallbackExtentUnsupported`; no body;
- a slab cutting a face-interior oval, a cube in the hole, two tori
  meeting in an oval: `FallbackExtentUnsupported` (the no-crossings
  extent gate runs for every op);
- the dumbbell (torus and cylinder handles) and MATE-7a's socket, peg,
  coincident, chain and kissing fixtures: the same typed doors as ∪ or
  the extent gate;
- a donut inside a big cube: `Voided`, `OperandB` and empty — but those
  pass on main already, since no torus box meets a face of the cube.

One shape returns a wrong body: the half donut and bracket of
`torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`,
where ∩ dropped the oval's lens. ∪ returns the same wrong body on main.
So the admission is blocked on that row, and the roster stays as it
is; `subtract_and_intersect_refuse_an_oval_their_crossings_cannot_see`
pins the refusal that keeps ∖ and ∩ off the wrong answer.

The revert seam lane itself (`Body::revert`, the sense bit on a torus
face, `point_in_solid`'s torus arm reading `sense`, the fallback's
cavity door) is kind-generic and was not what refused anywhere.

## Home

GERM, beside `torus-operand-gate-admission`.

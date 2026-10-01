---
id: torus-onto-the-subtract-and-intersect-roster
kind: issue
title: Torus onto revert_arm_exists: subtract and intersect still refuse a torus operand at the front door
status: closed
opened: 2026-09-25
priority: P2
cost: M
branch: germ/torus-subtract-intersect
pr: 3416
refs: [torus-operand-gate-admission, torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
closed: 2026-09-29
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
- a slab cutting a face-interior oval, two tori meeting in an oval:
  `FallbackExtentUnsupported` (the no-crossings pass runs for every op).
  A cube in the hole refused there too when measured; since the section
  certificate (PR 3372) replaced the extent gate, the cube in the hole
  answers the disjoint union under ∪ (π² + 0.5), and its ∖/∩ now stop
  only at this roster;
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

## Parked (2026-09-28)

Measured by PR 3330 and not admitted: under ∖ and ∩ the interior-oval class returns wrong bodies. It waits on the section certificate, (b) on `torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`.

## Unparked (2026-09-28)

It was parked on the interior-loop row, which the section certificate
(PR 3372) closes: the half donut and bracket now refuses at the guard
(R-loop) under every op, so admitting the torus to this roster no longer
extends a wrong answer to ∖ and ∩. The admission's own measurement is
still owed: re-run the shapes above with `Torus` on the roster, against
the certificate.

## Admitted (2026-09-29)

Re-measured with `Torus` on `revert_arm_exists`, against the section
certificate, at the witness, `1e-6` and `1e-12` rows: ∖ in both
operand orders and ∩ over every shape above, the corner bar, the
MATE-7a fixtures declared and undeclared, the cylinder-handled
dumbbell, and 60 seeded random rods through the tube. Every body
returned was checked against its operands (volume identities, tier 3,
`point_in_solid` on a grid against both); none was wrong, so the torus
is on the roster.

What answers: the cube in the hole, the pin-only bracket and the wedge
clear of the carrier, in closed form (`germ_torus_doors`,
`germ_interior_oval`). The donut in a big cube answered already. What
refuses does so where ∪ does: the bars at the sagitta charge or the
pierce door, the slab at R-loop, two tori and the grazing cylinder at
R-reach, the declared dumbbell, socket and peg and coincident pair at
R-tan, the chain and the kissing pair at the rim routing, and every
undeclared torus pair at the crossing layer. The half donut with the
bracket refuses at the guard (R-loop) under every op.

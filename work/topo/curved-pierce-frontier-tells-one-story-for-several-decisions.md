---
id: curved-pierce-frontier-tells-one-story-for-several-decisions
kind: issue
title: topo: CurvedPierceUnsupported offers the declaration from every arm of curved_face_arm, including arms that read none, and the radius guards' decided arm renders as a join desync
status: parked
opened: 2026-09-30
blocked_on: [3990]
---


(TOPO, the §5 second pass of PR 3513: the definite siblings of its new
decisions; premise corrected in the fix pass, the review's m-2.)

## What

`BooleanError::CurvedPierceUnsupported` (`crates/topo/src/boolean/mod.rs`)
is raised by `boolean::reduce::curved_face_arm`'s `frontier` closure
for every shape the crossing layer cannot take, and ends in "declare the
coincidence, or move the geometry" (`geom_core::DEFINITE_COINCIDENCE_RECOURSE`).

**At the arms with an endpoint on the carrier the offer is right.** The
review executed it (`probe_wall_roots_in_band_with_and_without_declaration`,
`curved_face_arm` against a unit cylinder wall): a line in the exactly
tangent plane `x = 1` with one end at the tangency refuses as
`CurvedPierceUnsupported` undeclared, and with its side face declared
`Tangent`, which the door verifies definite (`contact_pair_verdict`),
returns `Ok(Recorded)` through the declared-cover rung. A declaration
changes that verdict, so D4 ¶1 (i) offers it.

**The root lanes' in-band arms offer none for a different reason**, not
a fork: the declaration that would settle them is one the real door
refuses in band. At the same line moved an in-band distance off the
wall (`WallRoots(WallRung::Discriminant)` escalating), the door answers
a `Tangent` declaration of that plane `Escalated` (a gap of `5e-9`, in
band) or `Contradicted` (`1.1e-8`, past it), and `Rest` `Contradicted`
(`carrier_kind`); only a `DeclaredPairs` built past the door reached
`Ok(Recorded)`. So "move the parts so the edge clearly crosses the wall
or clearly misses it", with no declaration, is that decision's honest
in-band ending, and the definite frontier's declare offer at a
zero-endpoint arm is not its sibling story.

What remains is that the frontier offers the declaration from arms
whose verdict no declaration reads:

- `(Positive, Negative) | (Negative, Positive)` (a straddle): a root set
  that does not account for the straddle (`Constant`, `Miss`,
  `NoInterior`, `Unsettled`) refuses, and no `covered` arm exists for it;
- `(Positive, Positive)` against a cylinder or sphere: the belly arm's
  `Constant` and `Unsettled`, and against a torus the roots' `Constant`
  and `Unsettled`;
- `(Zero, Zero)` undeclared, whose roots came back other than
  `NoInterior | Elsewhere` (the `covered` twin reads the declaration,
  this arm does not);
- the circle rung's `Ok(Sign::Negative)` for a non-torus circle (an
  arc that crosses the wall), and every carrier the rung has no arm
  for (an ellipse, a NURBS edge). The rung's `Ok(Sign::Zero)` is not
  on this list: a declaration settles it (below).

A second definite sibling of the same shape: the declared-coaxial
cylinder × sphere arm's radius guards (`BooleanDecision::Radius`). Their
decided arm, `SectionError::DegenerateOperand`, reaches
`join::cs_pair_frame`'s catch-all and renders as `BooleanError::JoinDesync`
("A/B lockstep invariant violated … kernel bug"), while the in-band arm
names the radius lever. No in-tree caller passes
`CoaxialEvidence::Declared` yet, so neither arm is reachable from a
public door today.

## Repair shape

Carry on `CurvedPierceUnsupported` which arm refused (a closed type the
`frontier` closure's call sites set): an arm whose `covered` twin reads
the declaration keeps "declare the coincidence, or move the geometry";
an arm that reads none ends in its own lever (the root lanes' "clearly
crosses or clearly misses", the straddle's roots). A row per kind runs
the arm undeclared and declared through the real door, as
`reduce::declaration_order_rows` does. Route `cs_pair_frame`'s
`DegenerateOperand` to the radius decision's decided arm.

## Since (PR 3513's second fix pass)

The circle rung's two decided arms were executed through
`curved_face_arm`, a sheet of a cylinder against the canonical unit
wall, under each posture
(`reduce::declaration_order_rows::the_circle_rungs_zero_arm_takes_a_declaration_and_its_crossing_arm_takes_none`,
adopted from the review's `zz_coincfr_rows.rs`):

- **`Zero`** (a sheet of radius `1 + zero/2`): undeclared, both rims
  refuse at the frontier; declared `Rest`, which the door verifies,
  with the pair called one carrier, the carrier-identity rung reads
  the clearance zero and both rims are recorded. A declaration settles
  this arm, so the frontier's declare offer is true there.
- **`Negative`** (a sheet whose axis is offset by 0.3, its rims
  crossing the wall): the frontier under every posture. The door
  refuses `Rest` (`Contradicted`, `carrier_cyl_axis_offset`) and
  answers `Tangent` `NotCertifiable` (no witness supplied); a pair
  called one carrier past
  the door still refuses at the frontier, since the rung's covered
  arm reads only a zero clearance. No declaration settles it, so the
  offer is false there.
- An arc definitely one side of the wall (radius 0.9 or 1.1, coaxial)
  reads `Positive` and is eventless under every posture.

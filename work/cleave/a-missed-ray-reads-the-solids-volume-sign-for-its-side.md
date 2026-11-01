---
id: a-missed-ray-reads-the-solids-volume-sign-for-its-side
kind: issue
title: A containment ray that misses the body takes its side from the solid's volume sign (margin V/A), so a definite thin body refuses as a disjoint operand
status: closed
opened: 2026-10-11
priority: P1
cost: M
branch: cleave/missed-ray-side
closed: 2026-10-12
pr: 4725
---


Found by FLUSH's thin-hollow design fork (fork log row 113). Both
designers measured it on main at ε = 1e-9.

## Measured

The slab `brick((0,1),(0,1),(0,1.5e-8))` is built at rest. Every pair
is definite, and it passes tier 3 and the census.
- `slab ∪ brick (5..6)³`, either order, and `slab ∪` a brick beside it
  refuse `ShellWitnessExhausted`: 26 of 26 witnesses read in band at
  7.5e-9.
- `point_in_solid(slab, (5.5, 5.5, 5.5))` refuses `Escalated`; the
  predicate is `bool_point_in_solid_infinity`.
- `big ∪ slab` builds, where the slab is absorbed and its sign never
  read.

## Cause

`boolean/solid_contain.rs` `at_infinity_side`, reached when a ray meets
no face, takes the far side from the solid's signed volume. Its margin
is "scaled to a mean thickness (V / surface area)", 7.5e-9 here, which
is in band. A ray that hits the slab would decide: its two crossings
are 1.5e-8 apart, and the nearer one's face orientation says which side
holds material.

## The shape to give

On a ray that misses, cast a ray at an interior point of one of the
body's faces and read the sign of the closest crossing. Keep V/A only
as a fast path where it is definite.
`a-built-sliver-is-not-a-legal-operand` (parked here) has the same
symptom on an unmeasurable sphere lens, and its hypothesis section is
superseded by this row's site.

## Built (branch `cleave/missed-ray-side`)

**Measured on main** (`4c517b4511`), at ε 1e-9, 1e-6 and 1e-12 alike,
for a slab `t` thick:

- `t ∈ {10, 12, 15, 20}ε`: `point_in_solid` far away refuses `Escalated`
  at `bool_point_in_solid_infinity`, with margin `t/(2 + 4t)`. The
  reverted slab refuses the same way.
- `slab ∪ brick (5..6)³` refuses `ShellWitnessExhausted`, with 26 of
  26 witnesses in band.
- `t = 40ε` decides.
- Below `10ε` the prism door refuses the slab itself
  (`interval_span_forward`), so the family starts at `10ε`.

**What decides the side at infinity.** The representation does not.
`point_in_solid` takes any `Body`, and a reverted body (`∖`'s
transient operand, a complement the door answers for) has infinity
inside its material. At rest, D1's positive-volume invariant exempts an in-band volume. An
inside-out slab in this window finishes `Validated` (filed as
RESTFRONT's
`an-inside-out-slab-thin-enough-for-its-volume-to-be-in-band-finishes-at-rest`).
So the side has to be measured, and the row's face-ray shape is the
measurement.

**Changed** (`solid_contain.rs`):
- `cast_ray` returns `None` for a ray that crosses no face.
- `missed_side` keeps the certified volume as the fast path, read once
  per query.
- Where the volume does not decide (in band, uncertified or zero),
  `aimed_side` casts rays from a point twice the selection's reach out
  along that missed ray. Each ray aims at a point certified strictly
  inside a planar face (`stands::planar_interior_point`, the ladder's
  rung 3, computed once per query), and is read to its closest
  crossing. A ray's fault is that ray's alone. Where no aimed ray
  decides, an in-band aimed reading is the refusal; otherwise the
  volume's.
- Every missed ray aims from its own far point, because a missed ray
  that runs along a thin body's carrier leaves its far point on that
  carrier.

**Pinned** at every ε (`crates/topo/tests/missed_ray_side.rs`), each
red on main:
- slab `t ∈ {10, 12, 15, 20}ε` far away reads `Out`, and its reverted
  body reads `In`;
- `slab ∩ brick` is empty in both orders, and `brick ∖ slab` is the
  brick;
- `slab ∪ brick` passes containment and refuses at the result gate on
  the slab's shell (D10);
- a thin-walled hollow (a void) reads `Out` far away and in its cavity,
  and `In` at both once reverted, and `hollow ∪ brick` builds;
- two thin slabs in one body read `Out` between and beside them, mid-plane
  included, and `In` once reverted;
- each body's `|V/A|` is asserted in band, so the aimed rays are what
  decide;
- the reverted slab as a user operand refuses `InsideOutOperand`
  (`SliverInsideOut`) in every op and both orders, against a far brick
  and one across it.

**The inside-out operand (review blockers).** With the aimed read, an
inside-out sliver that finished at rest gave silent wrong answers
(`brick ∩ rev` the brick, `brick ∖ rev` empty). So did a reverted thin
tube at ε 1e-12, whose one gate point read in band, and a reverted
slab at a dual, which skips the at-rest gate.

The refusal is now made where the reading is:
- A selection read as given (`SolidFaces::read_as_given`, the
  Boolean's witness reads of the other operand through
  `point_in_operand`) whose side at infinity reads `In`, by the volume
  or an aimed ray, refuses `PointInSolidError::InsideOut`. The Boolean
  types that as `InsideOutOperand` with `SliverInsideOut` per solid.
- The operand gate (`gate_sliver_orientation`) reads the at-rest
  slivers, and every solid of a dual operand, at one point outside its
  faces' reach boxes. That catches the routes that never read a far
  side. A reading that does not decide, or a face with no reach box,
  refuses typed.

Pinned at every ε:
- the slab, `Validated` and at a dual, in every op and both orders;
- the reviewer's tube (`crates/sweep/tests/an_inside_out_thin_tube_operand.rs`):
  `InsideOutOperand` at 1e-12, and an in-band refusal at the wider
  rows, where it does not build.

The evidence is on RESTFRONT's row, now P1.

## Closed (PR 4725, 2026-10-12)

A missed ray whose volume reading is in band now aims at a point certified inside a planar face and reads the closest crossing. An operand the caller handed in that reads infinity inside refuses `InsideOutOperand`, wherever the reading is made and for every scalar. Three reviews found no wrong side at any ε, and no operation that built on main now refuses except genuinely inside-out operands. The fix also builds strictly more valid operations than main, which itself answered reverted slivers wrongly.

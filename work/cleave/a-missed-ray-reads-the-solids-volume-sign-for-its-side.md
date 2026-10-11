---
id: a-missed-ray-reads-the-solids-volume-sign-for-its-side
kind: issue
title: A containment ray that misses the body takes its side from the solid's volume sign (margin V/A), so a definite thin body refuses as a disjoint operand
status: open
opened: 2026-10-11
priority: P1
cost: M
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
`point_in_solid` takes any `Body`, and two kinds of body have infinity
inside their material:
- a reverted body, which `∖` hands the door as a transient operand;
- a cavity wall's own selection (`SolidFaces::of_shell`).

At rest, D1's positive-volume invariant exempts an in-band volume. An
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
  inside a planar face (`stands::face_interior_point`, now
  crate-visible), and is read to its closest crossing.
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
  included, and `In` once reverted.

The sliver lens has no planar face, so it keeps the volume reading. Its
row waits on FLUXTAIL.

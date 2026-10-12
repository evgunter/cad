---
id: a-widened-rotation-angle-refuses-on-the-plain-interval-lane
kind: issue
title: a widened rotation angle refuses on the plain Interval lane at the first Node::Transform: cos^2+sin^2 is a bracket around 1 and transform_rigid_col0_unit is what notices
status: dispatched
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, plain `Interval`, on a chain of
ONE link whose single joint carries σ = 0.01 rad:

```
node 7 — the transform op refused: transform: the map's linear part is
not an isometry at tolerance — predicate transform_rigid_col0_unit
refused, definitely or in-band
```

and the same at two, three and four links, always at the FIRST
transform. MEASURED, and pinned per link count by `chaintol.rs`'s
`the_certified_table_says_what_the_header_says`.

**The mechanism below is a reading of that refusal, not a second
measurement**, and is written as such: `eval::wire`'s `transform_map`
builds `Mat3::rotation_about(axis, angle)`, whose columns are built out
of `cos(angle)` and `sin(angle)`; on an interval angle those are two
independent brackets, so `cos² + sin²` would be a bracket AROUND 1
rather than 1, and that is what a column-unit check would refuse on.
It fits the predicate's name and the map's construction, and nobody has
yet read the column norm's own enclosure at the refusal to confirm it.
What IS established without it: the refusal is at the first transform,
at every link count, and it is not a geometry error — the same document
certifies whole on the symbolic lane at one link.

**The symbolic tier discharges exactly that**, and that is the whole
difference between the two lanes on this document: the same one-link
chain CERTIFIES whole at `Sym<Interval>` in 0.16 s, with
`symbolic_zero: 1760`. So the finding is not "the kernel is wrong", it
is that **the plain interval lane cannot carry a rotation by a widened
angle at all**, at any width, and every consumer of a parametric
placement is on the symbolic tier or nowhere.

Worth stating because the tier is a *performance and reach* dial
elsewhere (`DriveConfig::symbolic`, `SymBudget::none()`) — on this
construction it is load-bearing for the answer existing.

## What answers it

First, cheaply: the column norm's enclosure at the refusal, read out —
which turns the reading above into a measurement or replaces it.

Then, either an interval-lane rotation that keeps the Pythagorean identity
(the rotation built so the column norms are exactly 1 by construction,
rather than recomputed from two independent brackets), or the
statement, where `transform_map` is written, that a widened
`SlotId::RotationAngle` is a symbolic-tier-only construction and the
plain lane's refusal is the designed answer.

## Home

SYM — the tier's reach on a construction Ev asked for.

## Question, and Ev's answer

What the plain interval lane owes a rigid placement whose angle is an
interval, and where rigidity is known.

SYM-17's Phase 1 measured the cause; it is on branch
`sym/17-rotation-readout`, with the probe on `sym/17-probe`.
`topo::transform_rigid` re-derives rigidity from the matrix entries.
`cos` and `sin` are independent brackets, so column orthogonality
misses by about ±6σ. The plain lane therefore carries only an angle
box narrower than about ε/2 rad. On main the symbolic lane certifies
the one-link chain over the whole study at σ = 0.01.

Phase 1's own pick was (c): state that a widened angle is
symbolic-tier-only, and have the refusal name that cause.

Two designers weighed the question independently and converged after
one round. Both recommend a rigid-motion witness type `Rigid3<T>` in
`geom-core`'s linalg, which `transform_rigid` takes:
- It is rigid by construction from rotations about a decided axis,
  translations, products and transposed inverses. Each of those
  constructors decides only finiteness.
- Today's `check_rigid` becomes its one decided constructor, used for
  a map that arrives as twelve numbers (a literal frame, a STEP
  placement).
- An undecided margin there stays undecided.
- A product of admitted frames is not decided again.

Both reject (c). E12 already states the plain lane's scope for every
identity in one parameter, and a refusal naming the angle would be a
cause written from theory. The witness lands on its own; SHELF's
`transform-rigid-recertifies-images-enclosed-apart`, carrying edge
certificates across the map, is its own question. No ratified decision
changes.

**Ev's answer (2026-10-12): the witness, merged with `OrthoFrame`, and
closed under the rigid group's action.** Ev asked two follow-ups first:
whether the idea generalises (mates especially), and why the two
designers' unifications could not be combined. The answer is the
combination:
- **One type.** `OrthoFrame` merged with the rigid-motion witness. It
  keeps the frame's accessors (`origin`, `u`/`v`/`w` as `UnitVec3`)
  and its decided mints. It gains the motion's mints by construction:
  rotation about a `UnitVec3`, rotation about an axis through a point,
  translation, exact half-turns, compose, and the inverse as the
  transpose. Each mint decides only finiteness. One decided door,
  today's `check_rigid`, admits a raw `Affine3`, and returns an
  undecided margin as undecided.
- **The action.** The type acts on `UnitVec3` and on itself, so
  revolves (`revolve/partial.rs`), sweeps (`swept.rs`) and patterns
  (`eval/wire/stepped.rs`) mint rigid motions too.
- **Callers.** `topo::transform_rigid` takes the type and asks nothing
  about rigidity. Placements (`Step::Rigid`, `chain_motion`) and the
  mate solve's representatives and inverses mint it. A literal `Frame`
  goes through the decided door. `editor-core`'s persisted `Frame`
  stays a general linear part, so an improper map stays representable
  and can be refused.
- **Products of admitted frames** are not decided again.
- **No general structure-group mechanism.** The kernel acts with one
  group. Mirror is built inside its own door (MIRROR-DESIGN P2), and
  nothing scales.

The remaining identity losses are a relation between two separately
enclosed objects, and one parameter reaching a result by two paths.
Both are the symbolic tier's (E12), and SHELF's
`transform-rigid-recertifies-images-enclosed-apart` is its own
question. The mates' re-measurement is retired by INTENT stage 3
("a mate places and never checks"). The type should land before
stage 3 rewrites `coset.rs`.


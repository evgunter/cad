---
id: a-plane-touching-a-bore-rim-splits-into-a-pinched-side-under-one-normal-and-refuses-under-the-other
kind: issue
title: a plane touching a tube's bore rim at one point splits into a pinched side under one normal and refuses NotAlternating under the other
status: open
opened: 2026-10-07
priority: P3
refs: [a-rim-touching-split-escalates-on-the-side-of-plane-band, split-sides-are-not-finished-bodies]
---


## What

Found by the sibling sweep of `cleave/rim-touch`, on `origin/main` at
`dfcd7f35`, at ε = 1e-9, 1e-6 and 1e-12 alike. The fixture is the tube
of `split_across_a_revolve_seam.rs`: `TUBE` revolved a full turn about
`y`, with the bore at r 0.5. The plane passes through the bore's top rim
point `P = (0.5 cos a, 1, 0.5 sin a)` with normal
`s·(ŷ cos 0.2 − (cos a, 0, sin a) sin 0.2)`. So the bore rim lies on the
plane's `+ŷ` side and touches it only at `P`, and on the top cap the
section line is tangent to the hole at `P`.

The cap-side half is therefore pinched at `P`. Near `P`, the material
above the plane is `−v² ≤ u < 0` (u radial, v tangential): two cusps
that meet at one point. The outer rim's touch (the parent row) leaves a
single region there, `u ≤ −v²/2`, and builds.

The two normals of the same plane answer differently:

- **`s = +1`** refuses at 95 of 96 azimuths with
  `Join(SectionCrossings { face, case: NotAlternating })`, from
  `splitting::join::conic_pairs` at its final alternation check. Only
  azimuth 0, the seam vertex, builds. `conic_pairs`' own doc says no
  shipped fixture reaches `NotAlternating`.
- **`s = −1`** builds at every azimuth, and tiers 1 and 3 pass. The
  cap side holds the wedge's closed form, 0.264337438 (by quadrature of
  `tan t ∫∫ (0.5 − d·x) dA` over the annulus where `d·x < 0.5`).
  - The cap side holds **two vertices** within 1e-6 of `P`.
  - Tier 3′ refuses both halves at every azimuth but azimuth 0:
    `UndeclaredContact { VertexOnFace { vertex 8v3, face 10v1 } }`
    below, and `{ vertex 9v1, face 12v1 }` above.
  - The witness is the bore's seam crossing, `(0.5, y, 0)`, e.g.
    `(0.5, 0.98292, 0)` at a = 6π/32. It is not `P`.
  - Why the contact names the seam crossing is unmeasured.

## Where to look

The `s = +1` refusal reads two crossings at one point (`P`, where the
section ellipse enters and leaves the bore wall's face) in insertion
order. `ConicCrossingsCase::NotAlternating`'s doc names that case
itself. So which arm a pose reaches is set by insertion order, not by
the geometry.

The `s = −1` build is the pinch side `split-sides-are-not-finished-bodies`
describes: a side whose touching pieces carry contacts the split
declares nowhere.

What the right answer is depends on D10's coincidence work. That is
whether the split builds the pinched side and reports its contact as an
`unproven-coincidence` finding, or refuses the pinch as one decision
with one recourse, as rule (a)'s knife edge does. Under D10's intent
hold, no declaration channel should be built for it. What does not
depend on D10 is that one plane under its two normals must give one
answer.

## Reproduce

In `split_across_a_revolve_seam.rs`, take
`a_section_touching_a_rim_splits_at_the_closed_form` and change the
touch point to `Point3::new(0.5 * d.x, 1.0, 0.5 * d.z)` with
`lean = 1`. Then run
`split(&finished(tube), &plane, tol)` at azimuth 0.3. `s = 1` refuses,
and `s = −1` builds halves that fail `validate_pseudomanifold`. The
sweep that found it ran 96 azimuths, `2πi/96 + 0.0123·(i mod 3)`.

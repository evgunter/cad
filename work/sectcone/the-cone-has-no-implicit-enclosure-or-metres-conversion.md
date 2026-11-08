---
id: the-cone-has-no-implicit-enclosure-or-metres-conversion
kind: issue
title: The cone has no implicit enclosure and no metres conversion, so every rung-3 pair with a cone (cone × cone, cone × sphere, cone × torus, NURBS × cone) refuses
status: open
opened: 2026-11-03
priority: P0
cost: H
design: true
refs: [cone-torus-rung-three-arm-waits-on-the-cone, a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, cone-germ-pairs-against-a-curved-face-have-no-section-frame]
---


Filed 2026-11-03 from the curved-boolean bookkeeping pass
(`work/splice/curved-demo-joins-build.md`). Several rows name this gap as
"the cone's own unit" or "SECTRUNG's cone unit", and no row owned it.

## What

The ℝ³ certificate's analytic halves have no cone arm:

- `crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` (line 388)
  and `implicit_gradient_enclosure` (line 438) answer
  `Interval::refused()` for `Surface::Cone`;
- `crates/geom-brep/src/ssi/certify.rs`, module docs: the composite's
  units convert to metres exactly for the plane, sphere and cylinder, and
  per span for the torus (`ToMeters::Torus`); "the cone has no
  conversion and refuses".

So `crates/geom-brep/src/intersect.rs` `route` answers
`implemented: false` for `(Cone, Cone)`, `(Cone, Sphere)` and
`(Cone, Torus)`, each note naming the cone's metres conversion.

## What waits on it

- `cone-torus-rung-three-arm-waits-on-the-cone` (P0, lily wall 2, the
  flower welded onto the stem), parked on this row.
- `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` (P0): Ev's
  ruling on PR 6121 builds "the cone after SECTRUNG's cone unit", and
  klein wall 3's partner is the flare cone.
- `cone-germ-pairs-against-a-curved-face-have-no-section-frame` (P1):
  its classification does not wait on this, and its march does.
- Cone × sphere in general pose. No row names it; the pair is in the
  same `route` arm.

## Open

The torus's conversion reads its residual through the quartic's own
factor (`F/(2r·(A + 2Rρ))`). The cone's implicit form carries `tan² α`,
and the rows above call its exactness the open question: whether the
half-angle enters as an exact rational, its tangent, or an enclosure,
and what that does to the "exactly" the other kinds' conversions claim.
That is why `design` is set. Weigh it with the designer pair before a
lane builds it.

---
id: the-cones-quadric-is-spelled-in-seven-scalars
kind: issue
title: The double cone's quadric cos²α·ρ² − sin²α·h² is spelled at seven sites in five algebras, each with its own angle crossing
status: open
opened: 2026-11-22
priority: P3
cost: M
---


Filed by SECTCONE (`sectcone/cone-tube-arm`, PR 7358's fix pass), at
its reviewers' request. The tube's cone arm added the seventh spelling.

## What

The double cone's quadric `Q = cos²α·ρ² − sin²α·h²` is the cone's one
polynomial form. Its sign is the double cone's, and it factors as
`(ρ cos α − |h| sin α)(ρ cos α + |h| sin α)`. These sites spell it
separately:

| Site | Algebra | Disposition |
|---|---|---|
| `crates/geom-brep/src/ssi/enclose.rs` `ConeParts::quadric` | `Interval` over a box, in certification arithmetic | the tube's exclusion; new in PR 7358 |
| `crates/geom-brep/src/implicit.rs` `conic_cone_harmonics` (`cc`, `ss`) | trigonometric harmonics along a conic, at `T` | the circle-carrier door |
| `crates/geom-brep/src/implicit.rs`, its reach charge (`cc * radial_reach…`) | scalar bound at `T` | the same door's charge |
| `crates/geom-brep/src/carrier_resolution.rs`, the cone arm (`dd.sub(&hh).mul(&cc).sub(&hh.mul(&ss))`) | exact rationals `Q` | the carrier's resolution |
| `crates/geom-core/src/spline/compose.rs` `CanonicalSurface::Cone { cos2, sin2 }` | composite of a carrier net | limb 2 |
| `crates/geom-brep/src/curve_face/composite.rs` `framed_form` | builds that composite's coefficients | the curve × face door |
| `crates/geom-brep/src/props/curved.rs` (`cos2`, `sin2`, the bilinear `m`) | polar form at `T` | measures |

`crates/geom-brep/src/curve_face/conic_quadric.rs` documents the same
form for the cone arm of the conic door.

No two of these run in the same algebra, so none can call another
today. What each does repeat is the coefficient pair `(cos²α, sin²α)`
and the angle crossing it starts from. That crossing is where they
disagree: some take `half_angle.sin_cos()` at `T` and then cross, and
some cross first (`work/pcert/the-cone-composites-cross-a-sine-taken-at-the-lane-scalar.md`).

## What would fold them

The one shared piece is a coefficient source: the cone's `(cos²α, sin²α)`,
or its slope's `(1, k²)`, at the scalar each site needs, crossed one
way (`implicit::cone_sin_cos`). The forms themselves stay per algebra.
`work/sectcone/the-cone-stores-its-slope.md` rewrites every one of these
sites, and it is the natural place to make that source. Until then this
row is the census.

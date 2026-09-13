---
id: sphere-union-sphere-refuses-though-the-section-is-closed-form
kind: issue
title: sphere u sphere refuses CurvedPierceUnsupported although intersect::route mints the exact circle
status: open
opened: 2026-09-09
---


Found while surveying demo coverage: a snowman — two overlapping balls
unioned — is not authorable, and the refusal is one layer above where
the pair's geometry actually stops.

## Measured

Two balls of revolution on distinct centres (`revolved_about_y` of a
semicircle, `Revolution::Full` — the canonical `revolve_ball` shape:
two half-bands on ONE sphere key, the class
`m5_pr9c_sphere_doors.rs` calls closed), radii 1.0 at `y = 0` and 0.8
at `y = 1.4`, so the two spheres meet in a real circle:

```
topo::boolean::union(&a, &b, Tol::witness())
  -> CurvedPierceUnsupported { operand: A, face: FaceKey(1v1),
                               edge: EdgeKey(1v1),
                               band: Band { zero: 1e-9, escalate: 1e-8 } }
```

## Why this pair is worth a row of its own

Everything below the crossing layer is already built for it:

- `reduce::boolean_arm_exists` admits `Sphere` (it is one of the four
  kinds on the roster, with `Plane`, `Cylinder` and `Nurbs`), so the
  pair-scoped operand gate does NOT refuse it — the operation gets past
  the gate and dies later;
- `revert_arm_exists` admits `Sphere` too, so `∖` and `∩` have their
  seam lane;
- `geom_brep::intersect::route(Sphere, Sphere)` is implemented
  (`crates/geom-brep/src/intersect.rs:267`) and
  `sphere_sphere_section` returns an exact `Curve3::Circle` with the
  three-way `Empty` / `TangentPoint` / `Circle` classification already
  decided against the band.

So the section is closed-form and certified, and the operation still
cannot complete. That makes this a *pierce-substrate* row rather than a
missing-arm row: the same substrate
`boolean-refuses-on-arc-carrier-not-arc` needs for its parallel- and
coaxial-cylinder rows (both of which land on this same
`CurvedPierceUnsupported`), reached here by a pair whose section needs
no new geometry at all. Whoever specs that substrate should carry this
fixture, because it is the cheapest one in the family: no chart window,
no azimuth run, one circle.

## What it blocks outside the kernel

The sphere×sphere blend arm (`BlendArm::SphereSphereTorus`,
`crates/sweep/src/blend/battery.rs:1075`) is implemented and carves on
either material side, but every body that reaches it today is a
REVOLVE of a multi-arc meridian (`sweep/tests/verbs_arms3.rs`'s
lentil). A body with two sphere rims that are not coaxial — the shape
the arm's own docs describe as a snowman's waist — has no constructor,
because the union that would build it is this refusal.

## Repro

`crates/sweep/tests/` — build two balls as above and call
`topo::boolean::union`. No fixture in tree covers the pair today.

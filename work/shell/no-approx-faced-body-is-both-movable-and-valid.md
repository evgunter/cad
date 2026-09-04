---
id: no-approx-faced-body-is-both-movable-and-valid
kind: issue
title: no Approx-faced body is both movable and tier-3 clean - the iso seam class needs a spline carrier, and a spline carrier is what transform refuses
status: open
opened: 2026-09-04
---


Found while writing SHELL-2's acceptance rows (`docs/SHELL-2-SPEC.md`
§3), whose rows 1, 2, 3 and 6 all assume the OFF-C consumer suite's
`prism_with_approx_walls` body can be moved by `topo::transform_rigid`.
It cannot, and neither can any substitute this tree can build. The
transform door's `Approx` arm itself is fine — what is missing is a
body to exercise it on end to end.

## The three walls, measured

**1. The lofted prism cannot move at all, and never could.** Its four
vertical wall seams carry `Curve3::Nurbs`, which
`crates/topo/src/transform.rs`'s `map_carrier` refuses
(`TransformError::NurbsPlaceholder`) whatever any face carries.
`transform_rigid(&prism(), &translation, witness)` refuses on the
UNCONVERTED prism too, so this is not about the `Approx` surgery. That
arm is `transform-rigid-refuses-described-nurbs` (issue record 1346),
already filed and still open.

**2. An `Approx` face's edges cannot be re-described onto its own
chart with straight carriers.** Replace a box cap's surface with a
certified `Approx` (`set_face_surface(face, FaceSurface::New(...))`,
which mints a fresh key and so staleds every description naming the
old one) and neither re-description available lands:

- remapping to `Intersection { approx, wall }` refuses at the attach
  gate — "a Nurbs described surface, or a Nurbs carrier under a
  conventional description, cannot be certified in this build" — which
  is right: an approximating surface's implicit layer is poison, so
  there is nothing for an `Intersection` residual to measure against;
- re-describing as `Chart` + `Pcurve::IsoLine` refuses in the pcurve
  pass with `IsoUnsupported { what: "a seam-class iso line over a
  non-spline carrier — no construction mints one" }`
  (`crates/geom-brep/src/pcurve_cache.rs:3775`). Two of a quad cap's
  four edges hold `u` constant while `v` traverses, which is the SEAM
  class, whose control-difference hull compares the carrier against the
  chart's own boundary ROW and therefore needs the carrier in that
  spline space. The CAP class beside it (`v` constant, `u` traversing)
  is the one that admits a `Curve3::Line`, and a quadrilateral chart
  boundary always has two edges of each.

So the movable fixture SHELL-2 landed
(`crates/sweep/tests/common/approx.rs::box_with_approx_cap`) carries
four permanent `DescriptionNotAdjacent` findings, held constant either
side of the map.

**3. Without pcurve caches the props lane will not read the face.**
`mass_properties` on that body refuses `QuadratureUnsupported { what:
"NURBS face half-edge carries no stored pcurve cache — the loft
assembly mints them; a body that lost its caches must re-mint before
mass properties" }`, which is leg 2's consequence rather than a
separate gap.

## What it costs

- SHELL-2 §3.1's tier-3-clean claim on a moved body, §3.2's mass-
  properties invariance, and §3.6's "every half-edge of a mapped
  `Approx` face carries a re-derived cache" are all unassertable. The
  rows that landed instead read the DIFFERENCE either side of the map
  (same tier-3 finding set, same props verdict, same pcurve verdict),
  which is the honest claim available.
- More generally: an `Approx` face is reachable today only on a lofted
  body, and a lofted body is immovable. The first consumer that wants
  to place a shelled or offset part will meet both walls at once.

## What would lift it

Either leg alone is enough for SHELL-2's rows:

- leg 1 (`transform-rigid-refuses-described-nurbs`) would make the OFF-C
  prism movable, and its `Approx` walls already carry iso caches;
- leg 2 would need the seam class to admit a straight carrier — the
  hull argument would have to be made against something other than the
  chart's boundary row — or the face-replacement primitive
  (`topo::replace_face_offset`'s re-description machinery) exposed for a
  surface swap that is not an offset.

Leg 1 is the cheaper of the two and is already scheduled.

---
id: the-region-arm-is-silent-on-spline-faces-and-off-admits-edges
kind: issue
title: Tier 3's region arm is silent on spline faces, on edges its doors do not admit, and on a torus cycle with no stored joint
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [validate-tier3-curved-boundary-containment]
---

## What

Tier 3's region arm (`crates/topo/src/one_region.rs`, `region_errors`)
reads each surface cycle of a curved face against the others through
`boolean::cycle_side` (`crates/topo/src/boolean/contain.rs`), the
point-in-face doors cut to one cycle. Where no door reads the face, the
face is exempt, silently, as validate's not-yet-checked list says
(`validate_geometric`'s "Face-boundary containment" bullet):

- **A spline chart** (`Nurbs`, `Approx`): `cycle_side` answers `None`.
  The spline door reads a face's whole trim at once
  (`boolean::spline_region`, `Trim::of` and `Trim::with_rings`): it lifts
  each loop, refuses a loop that winds, and places each ring in the outer
  loop's cover. A ring it cannot place inside makes the trim unreadable,
  so a spline face's ring outside its outer loop is unread rather than
  refused, and there is no one-cycle entry to give the arm.
- **An edge its door does not admit** (each door's `admits`, read by
  `region::boundary`): a cylinder or cone edge other than a line, circle
  or ellipse (`ruled_region::ruled_face_region`); a sphere edge other than
  a circle, or one with no certified curve (`sphere_region::sphere_face_region`,
  `Scaffolding::Refused`); a torus edge other than a circle or a spiric
  (`torus_region::torus_face_region`). The same faces are the
  chart-singularity arm's residue now that it reads the side through the
  doors (`the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band`), where `walk_loop` used to read a cone loop or a
  sphere loop with a non-circle edge.
- **A torus cycle with no stored joint element**: `cycle_winding`
  (`crates/topo/src/surface_boundary.rs`) answers `None`, and
  `one_region::torus_units` exempts the face, since a contractible cycle
  cannot be told from an essential one.
- **A cycle no vertex of which its door places**, none in band: no path
  settled (`RayExhausted`); an arc the door reads as wound past a turn
  (`RegionRefusal::WoundPastPeriod`, carried as `CorruptFace`), a definite
  reading that bounds no region and so is no side; or an edge whose
  carrier's reading cannot cross it (`RegionRefusal::Unreadable`, carried
  as `EdgeCarrierUnsupported`). `one_region::read_cycle` sets each aside
  vertex by vertex. Refusing `WoundPastPeriod` typed is the stronger
  form, and owed here. It is not distinguished today because it reaches
  the arm as `ContainError::Curved(PointInSolidError::CorruptFace)`, the
  same value a door builder's unresolved key gives (one tiers 1 and 2
  refuse before tier 3 runs). Telling them apart needs a variant of the
  public `ContainError` or `PointInSolidError`, whose every match across
  the workspace would move with it: a change for its own unit, not a
  side effect of this arm.

## Owed

A one-cycle reading of a spline face's trim (the chart's own cycle
placement, one loop at a time), and the doors' carrier inventories
widened where a body the kernel makes carries such an edge. Filed by the
region arm's implementer lane (unit 2 of
`validate-tier3-curved-boundary-containment`).

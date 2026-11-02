---
id: a-cone-by-curved-carrier-section-through-the-apex-has-no-germ-frame-or-certified-count
kind: unit
title: A cone × curved-carrier section through the apex has no germ frame or certified count
status: open
opened: 2026-11-02
priority: P1
cost: H
design: true
parent: a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex
---


Filed by SPLITDOOR unit B stage B2 (`splitdoor/cone-apex-edge-face`), as
the cone-apex work that B2's scope leaves (the parent row's "Scope"
section).

## What

The sweep now reads a conic arc through a cone's apex
(`boolean::conic_quadric`, "Through the apex"). Past the sweep, a cone's
apex against a CURVED partner meets two frontiers that the vertex lane
does not reach:

- **A cone × curved-carrier section through the apex.** Where a curved
  face passes through the apex, the section curve passes through a
  singular point of itself. Its germ there is the cone's rulings in the
  partner's tangent plane, but its branches are curved. The join has no
  germ frame for it, and the section certificate does not certify its
  count.
- **The curved-face VF pierce.** Unit A's pierce reads an apex against a
  plane only, so an apex inside a curved face refuses at that gate.

## Witnesses (measured on B2's head)

The 30° and 45° cones, both forms (one wrap edge to the apex, and
`sweep::revolve`'s), every op:

- **A sphere through the apex along its seam meridian**
  (`ball_poled_y`, centre `(0.3, 1.2, 0)`, radius `|(0.3, 0.2)|`):
  - at 45° the join refuses `GermFrameUnsupported { Cone, Sphere }`;
  - at 30°, `FallbackExtentUnsupported` (the section's component count
    is not certified).
- **The apex inside a sphere face** (centre `(0.25, 1.3, 0.2)`, radius
  its distance from the apex): `CurvedBooleanUnsupported { Sphere }`,
  unit A's curved-pierce gate.

## Where to look

This is a design question before it is a build: what germ frame a
section has at a singular point of its own curve, where it leaves along
two directions, and how the section certificate counts components
through such a point. It is left for a designer pair
(`memories/orchestration-model.md`) before anyone builds it.

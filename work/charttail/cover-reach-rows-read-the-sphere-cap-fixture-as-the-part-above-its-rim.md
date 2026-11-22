---
id: cover-reach-rows-read-the-sphere-cap-fixture-as-the-part-above-its-rim
kind: issue
title: the cover-reach sphere-cap rows read the fixture as the sphere above its rim, which holds the part below
status: open
opened: 2026-11-20
priority: P3
cost: E
refs: [sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over]
---


## What

`test_support_fixtures::sphere_cap(z, tol)` builds the sphere face BELOW
its rim: the face holds the south pole and the body is the unit ball
below the plane `z` (the plane's face points `+z`). Measured on
`charttail/sphere-scaffolding` at `z = 0.3` and `z = −0.5`: the sphere
region reads a point near the north pole outside and one near the south
pole inside; `point_in_solid` reads `(0, 0, ±0.9)` `Out`/`In` and
`z ∓ 0.01` on the axis `In`/`Out`; tier 3 (`validate_geometric`) reports
`ChartSingularityNotAVertex { point: South, lies: Interior }`. The
fixture's doc now says so.

Two rows in `crates/topo/src/cover_reach_rows.rs` read it as the part
ABOVE the rim:

- `a_sphere_cap_past_a_hemisphere_reaches_its_rims_antipode` ("the cap
  above latitude −30°"): the face below `z = −0.5` lies within `√3` of
  its rim point `(√0.75, 0, −0.5)` (the opposite rim point), not 2. The
  row's `far >= 2` holds only through the reach's looseness, so it pins
  nothing about the case it names.
- `a_sphere_caps_axial_range_reaches_its_pole` ("reaches the north
  pole, 1.5 up the axis"): the face reaches nothing above its rim, so
  `above >= 1.5` is the bound's slack, not the pole.

## Owed

Build the upper cap for those two rows (flip the face's sense and the
plane's, or a `sphere_cap` variant that mints the other side), so each
row reads the face it describes; check each assertion can go red (a
mutant that drops the interior pole from the reach).

Also seen: `validate_geometric` on the fixture reports two
`JoinableVertexAtRest` (its two half-circle edges' vertices), so the
fixture is not at rest at tier 3; its callers read it below tier 3.

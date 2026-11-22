---
id: a-circle-lane-search-draws-a-hyperbola-section-and-panics-in-its-fixture
kind: issue
title: The circle-lane envelope search draws a cone section that is a hyperbola on some seeds and panics in its fixture
status: open
opened: 2026-11-21
priority: P2
cost: C
---


Found by ONEHOME (2026-11-21) while building the check-1 frame half.
`geom-brep` `pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual`
is a seeded counterexample search, and on some seeds its fixture panics
before it reads anything: `cone_section_circle`
(`crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs:95`) unwraps
`section_circle`, which answers `None` where the drawn plane cuts the
cone in a hyperbola ("the plane cuts the cone in an ellipse"). Two seeds
that do it, on main's code: `CAD_FUZZ_SEED=0xf3b95075cd2e11f0` and
`0xd8ecb7ce953180d3`.

No answer is wrong; the search is red on a draw it should skip or
redraw, so CI goes red on those seeds whatever the code under test does.
The fix is the draw's: take `section_circle`'s `None` as a draw to skip
(or bound the tilt by the half-angle), not a panic.

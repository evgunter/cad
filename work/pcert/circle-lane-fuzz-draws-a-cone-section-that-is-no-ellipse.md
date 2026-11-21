---
id: circle-lane-fuzz-draws-a-cone-section-that-is-no-ellipse
kind: issue
title: circle_lane_rows' projected-envelope fuzz draws a cone section tilted past the opening and panics in its own fixture
status: open
opened: 2026-11-20
---


Found by GERM's conic × quadric door move (2026-11-20), running
geom-brep's `curve_face` and `rows` tests locally on a branch that does
not touch `pcurve_cache`.

`pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual`
(`crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs`) panics at
`CAD_FUZZ_SEED=0xea6d83d1bac4eb9e` (effort 1), in its own fixture, not
in the code under test: chart arm 0 (a cone) hands
`cone_section_circle` a tilt `tau` it drew, and that fixture panics
"the plane cuts the cone in an ellipse" because `section_circle`
returns `None` — the drawn plane is tilted past the cone's opening and
cuts a hyperbola (its own doc: "a wide cone tilted past its opening
cuts a hyperbola, whose minor semi-axis reads NaN"). The draw is
`alpha` log-uniform up to `1.567` with either `cone_tilt_for(..)` or
`range(b.zero(), b.escalate()) / (|h|·tan α)`; near `α → π/2` either can
pass the opening.

Repro:
`CAD_FUZZ_SEED=0xea6d83d1bac4eb9e cargo nextest run -p geom-brep -E 'test(=pcurve_cache::circle_lane_rows::a_circles_projected_envelope_covers_its_residual)'`.

The fix is the draw's: skip (loudly) or redraw a pose whose section is
no ellipse, as `section_circle`'s other callers do, rather than panic.

---
id: offset-lane-has-no-conic-carrier
kind: issue
title: the offset lane cannot carry a hyperbolic edge: a partial-revolve cone's wall meets its moved meridian cap in a conic the kernel has no carrier for (C5 R1)
status: open
opened: 2026-09-08
---



The design question behind
`partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement`,
filed separately so that item can park on it. An offset meridian cap
is its plane translated `t` off the axis; a plane parallel to a
cone's axis at distance `d` cuts the cone in the hyperbola
`x² = r(z)² − d²`. So the wall/cap generator of ANY partial-revolve
cone (a wedge, a frustum, at any turn angle) is a line at rest and a
hyperbola after the offset, and the axial door's carrier law — "the
old carrier's kind and conventional frame with its position
re-solved" (`crates/topo/src/offset_axial.rs`, module doc and
`mint_carrier`'s two-chart `Line` arm, which says so in its own
comment) — has no carrier to mint. The endpoint meter refuses
`TogetherEdgeDisagreement` at millimetre gaps (0.64 mm on the conical
wedge, `sf2b_r2_probes`; 1.34 mm on the three-quarter frustum,
`shell7_seam_corner`). The kernel's stance is ratified at C5 R1
(`crates/geom-brep/src/intersect.rs`): the plane×cone generic tilt
routes to rung 3 "explicitly and permanently — the conic trio
(parabola/hyperbola) does NOT land in M5", a documented decision
"permanent until a PR moves it". `Curve3` carries `Ellipse` (M5 PR 5)
and `Nurbs`; a hyperbola arc is a rational quadratic, so either an
`Ellipse` sibling or the existing `Nurbs` carrier could hold it
exactly, and the offset lane could mint it in closed form (both
corners are solved already; the conic is determined by the moved cone
and the moved plane). That is a fork for Ev, not a SHELL unit: it
moves a C5 decision and adds a carrier kind every downstream reader
(pcurve mint, tier 3, mesh, STEP) would have to take. Until it is
ruled, the refusal stands and is loud; the in-fence improvement
(decide the class at the door's gate, typed and named, before the
corner solve) rides the next unit that touches the `Line` arm.
Signed (SHELL orchestrator).

---
id: offset-lane-has-no-conic-carrier
kind: issue
title: the offset lane cannot carry a hyperbolic edge: a partial-revolve cone's wall meets its moved meridian cap in a conic the kernel has no carrier for (C5 R1)
status: closed
opened: 2026-09-08
priority: P0
cost: H
closed: 2026-10-28
pr: 5897
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
(`crates/geom-brep/src/intersect.rs`'s plane×cone route): the tilted
plane×cone ellipse is exact (Ev, 2026-10-01: "exact ellipses are
certainly allowed there"), and a parabolic or hyperbolic section
refuses naming its conic — "parabola and hyperbola are outside the
conic inventory by decision, not by omission", a documented decision
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

## Ruling (Ev, PR 5652, 2026-10-25)

Five designer rounds; Ev approved the converged answer: *"i'm still not
totally sure about having special cases for the parabola and hyperbola,
but sure, we can run with it"*. C1 and C2 of `crates/geom-brep/README.md`
state it:

- **No `Hyperbola`/`Parabola` kinds.** A plane×cone parabola or hyperbola
  is minted from the pair in closed form as an exact rational-quadratic
  arc (`Curve3::Nurbs`, weights `(1, w, 1)`), never marched.
- **The certificate is derivation-blind.** Limbs 1–2 are owed by every
  carrier of every kind; the tube is owed unless the pair's section
  function states, for the pose, a crossing-angle floor over the
  carrier's stretch and the stretch's clearance from every other piece of
  the ε-overlap. Both designers called that tube exemption severable (an
  optimisation, not what settles the kinds); given Ev's reservation the
  build lands it last and alone, so it can be dropped without touching the
  rest.
- The axial door takes the moved pair's own section.

The build is cut into units on this slate
(`work/offset/plan.md`); this row closes when they land.

## Closed

Its three units have landed: limb 2 for every carrier (PR 5689), the
exact plane×cone arc and the axial door's moved section (PR 5692), and the
section's tube bounds (`a-classified-pairs-section-states-the-bounds-that-retire-the-tube`).
A partial-revolve cone's moved wall/cap edge is now the moved pair's exact
hyperbolic arc, and it rests: the conical wedges at a quarter and a third
of a turn and the three-quarter-turn frustum hollow to their closed forms.
Residue filed: `a-cone-wedges-apex-corner-beside-two-moved-caps-is-read-as-a-pole`
(a whole cone's wedge), `doors-that-read-the-plane-cone-section-do-not-carry-its-open-conic`
(the other doors still refuse the conic),
`a-family-scalars-cone-pose-too-wide-for-the-tube-bounds-refuses-rather-than-bisecting`.

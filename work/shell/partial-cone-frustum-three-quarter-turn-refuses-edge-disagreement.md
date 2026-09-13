---
id: partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement
kind: issue
title: a three-quarter-turn partial revolve of a cone frustum refuses TogetherEdgeDisagreement at its wall/cap generator
status: parked
opened: 2026-09-08
blocked_on: [offset-lane-has-no-conic-carrier]
---


Measured by the SHELL-7 R1 review lane (`crates/sweep/tests/shell7_r1_diff.rs`,
`wedge 3/4 turn frustum`) and pinned in
`crates/sweep/tests/shell7_seam_corner.rs`,
`a_three_quarter_turn_cone_frustum_refuses_edge_disagreement`: the
cone frustum `(0,0) → (1,0) → (0.5,2) → (0,2)` revolved
`Partial(3π/2)` and shelled at `t = 0.05` refuses
`ReplaceFaceError::TogetherEdgeDisagreement` at the axial door
(`crates/topo/src/offset_axial.rs`, `param_on`'s endpoint meter) with a
gap of about `1.34e-3` m, on the merge base and on SHELL-7's head
alike. `sf2b_axial` only turns its frustum a quarter, so no shipped row
saw this. The shape: a partial revolve of a cone with two meridian
caps, whose wall/cap generator is a LINE between a cone and a moved
cap; the line carrier is moved perpendicular to itself from its start
corner, and the two corner solves (each `[cone, cap, station plane]`
against the moved surfaces) do not agree about the edge between them
by a millimetre. Whether the corner solve or the line carrier's mint
is the one that is wrong is not measured here; the refusal is loud,
no wrong body passes, and the row keeps it visible.

## Record corrected at SHELL-8's close (SHELL orchestrator, 2026-09-08)

Two premises above are wrong, and the question the item leaves open
is answered by the door's own text. (1) `sf2b_axial`'s quarter-turn
`wedge` is a CYLINDER (`r` constant), not a cone frustum — its
`cone_frustum` is a full revolve — so the turn angle is not the
variable; the surface kind is. (2) It IS measured which side is
wrong: `crates/topo/src/offset_axial.rs`, `mint_carrier`'s
two-chart `Line` arm says it in its own comment — "a meridian plane
meets a CONE in a hyperbola, so a straight edge between those two is
straight only where the operand made it so, and the moved pair need
not carry a line at all" — and pins the same class on a conical
wedge at 0.64 mm (`sf2b_r2_probes::r2_a_conical_wedge_meridian_edge`).
The geometry: an offset meridian cap is the cap's plane translated
`t` off the axis, and a plane parallel to a cone's axis at distance
`d = t` cuts it in the hyperbola `x² = r(z)² − d²`; the corner solves
sit on that hyperbola and the line carrier does not, by about the
variation of `d²/(2r)` along the generator — `1.25 mm` to `2.5 mm`
of sagitta on this fixture's `r ∈ [0.5, 1]`, the measured `1.34 mm`
gap. The refusal is the meter doing its job; the line mint is the
false posture, and no line would be right.

What would close it is a hyperbolic edge carrier, and that is a
ratified decision rather than a SHELL unit: `crates/geom-brep/src/intersect.rs`'s
C5 table routes the plane×cone generic tilt "to rung 3 explicitly and
permanently — the conic trio (parabola/hyperbola) does NOT land in
M5" (R1, "permanent until a PR moves it"). Parked on that decision:
the SHELL program does not carry conics on its own, and the standing
refusal is loud on every partial-revolve cone at any turn angle. If
Ev wants partial-revolve cones to shell, the fork is C5 R1's — a
rational conic carrier (an `Ellipse` sibling, or `Curve3::Nurbs`)
that the offset lane could then mint in closed form; the axial door's
`Line` arm would gain a `(Cone, Meridian)` sibling that mints it.
Until then a cheaper improvement is available inside SHELL's fence
and not taken as a unit on its own: deciding the class at the door's
gate (a cone wall meeting a meridian cap, named typed before the
corner solve, as the torus's spiric rim is) rather than letting the
endpoint meter discover it — it rides the next unit that touches the
`Line` arm.

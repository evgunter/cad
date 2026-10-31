---
id: a-cone-wedges-apex-corner-beside-two-moved-caps-is-read-as-a-pole
kind: issue
title: the axial door reads a cone wedge's apex corner, between the cone and two moved meridian caps, as the axis's pole, so a whole cone's wedge refuses to hollow
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by `a-classified-pairs-section-states-the-bounds-that-retire-the-tube`,
which lets the axial door mint a moved cone/cap hyperbola: a quarter-turn
wedge of the whole cone `(0,0) → (r0,0) → (0,h)` shelled at `T` now passes
the conic gate and refuses `TogetherEdgeDisagreement` with a gap of exactly
`T/sin(turn/2)` (pinned in
`crates/sweep/tests/sf2b_r2_probes.rs`,
`r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`).

The apex vertex meets the cone and both meridian caps. In
`crates/topo/src/offset_axial.rs`'s corner solve the cone contributes one
`Profile::Line` (the generator) and the caps none, so the
`profiles.len() < 2` branch decides the old corner a pole
(`offset_axial_pole`, `ρ_old = 0`) and keeps it on the axis at the moved
apex. The meridian-pair arm that puts a corner on the moved caps'
crossing line, `cap_pair_corner`, is taken only for a `Profile::Circle`
beside two caps; a generator line beside two moved caps has no arm, so the
pole answers it, `T/sin(turn/2)` from where the two moved caps and the
moved cone meet. The edges into the apex then disagree with their moved
corners, loudly. The fix is `cap_pair_corner`'s station read off a line
profile as well as a circle (the caps fix the radial line, the generator
the station).

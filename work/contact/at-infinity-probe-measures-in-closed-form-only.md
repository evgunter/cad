---
id: at-infinity-probe-measures-in-closed-form-only
kind: issue
title: point-in-solid's at-infinity probe measures in closed form only, so an obliquely trimmed wall refuses VolumeUncertified
status: open
opened: 2026-10-01
---


## What

`point_in_solid`'s no-hit arm reads the at-infinity side from the sign
of the selection's volume (`crates/topo/src/boolean/solid_contain.rs`,
`at_infinity_side`), and measures it with
`props::mass_properties_closed_form_of` — the closed form only, because
the door is `T: Decide` and holds no quadrature lane. A cylinder wall
trimmed by an ellipse has no closed form, so the probe refuses
`Containment(VolumeUncertified)` on a body that the certified door
(`topo::mass_properties`) measures exactly.

Measured (REACH, branch `reach/volume-backstop`, 2026-10-01): the rod
of `crates/sweep/tests/axis_lap.rs` split at `z = 0.5 + tan 20° · y`
and `z = 3.5 + tan 20° · y`, then flatted by `brick((-1, 1), (0.2, 1),
(-1, 5))`: the probe's props refusal is
`Face { source: NotIsoRectangle { what: "cylinder boundary carries an
ellipse arc (curved cut) — …" } }`. Pinned at that outcome by
`axis_lap.rs` `an_oblique_cap_flats_through_its_ellipse_arc`.

## The shape of a fix

The boolean's volume backstop had the same gap and now measures
through the scalar's own lane, `AtRestPolicy::quad_lane` (certified
quadrature at `f64`/`Probe`/`Interval`/`Sym`, the closed form at a
dual). The probe can do the same, but `point_in_solid`,
`point_in_solid_of` and `point_in_solid_faces` are public at
`T: Decide` with some two hundred callers, so the lane has to reach
the probe either through those signatures (an `AtRestPolicy` bound)
or as a parameter from the boolean's own call sites. The probe only
needs a SIGN, so the sign walk (`props::sign_walk`) and its bracket
ends (`ShellRole::decided_at`) are the reading to take, not the
reporting midpoint.

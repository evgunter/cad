---
id: a-cones-chart-junction-gap-is-wide-over-a-box-on-its-axial-position
kind: issue
title: A cone wall whose position along its axis is boxed refuses props_chart_junction_closed at the symbolic box scalar, where every premise of its closed form is proven
status: open
opened: 2026-11-22
---


Found by BOXFINISH's volume-premises lane (branch
`boxfinish/volume-premises`, 2026-11-22), building a cone fixture for
`work/boxfinish/the-volume-readings-interval-re-derivation-re-asks-a-closed-forms-premises-on-the-value-channel.md`.
The cause is not traced: what follows is the payload and the raising
site.

## The body

A cone frustum through the revolve door at exact numbers
(`sweep::test_support::revolved_about_y_at`, the profile
`(0, 3.5) → (0.5, 3.5) → (1, 4) → (0, 4)` about the sketch `y` axis: a
45° cone whose apex is at `y = 3`), then its stored geometry restated
(`topo::test_support::restate_geometry`) so that every stored `3`,
`3.5` and `4` reads `a`, `a + 0.5` and `a + 1`, `a` one parameter over
`[2.95, 3.05]`: the body moved along its own axis by a boxed amount.
No door builds it over a box (each refuses at edge certification).

## Measured (`Sym<Interval>`, ε = 1e-9)

Check 7's volume role walk (`topo::test_support::volume_role_reading`):

- every premise the cone's closed form asks decides at the walk's
  scalar: `props_circle_axis_class`, `props_rim_fit`,
  `props_cone_circle_tilt`, `props_cone_circle_off_axis`,
  `props_meridian_on_surface`, `props_meridian_generator`, each twice;
- the walk then refuses
  `Face { 3v1, Escalated { "props_chart_junction_closed", [0, 0.19999999699999994] } }`,
  raised in `geom-brep` `props/closure.rs` `require_junction_closed`,
  called at `Sym<Interval>` from the role walk's round 0
  (`SignCertificate::at_round_zero`), not from the interval
  re-derivation;
- at `f64` the same body reads its volume at `a = 2.95`, `3` and `3.05`
  (`[0.9162978572970213, 0.9162978572970248]` at each), and at
  `Interval` with `a` exact.

The gap's enclosure is twice the box, the shape of one boxed
coordinate less itself across a junction's two readings.

## Unmeasured

Which two readings of the junction the gap is formed from, and whether
the symbolic form of the gap is zero (so that the refusal is the value
channel's alone). A torus of boxed minor radius and a seamless cylinder
of boxed radius pass the same walk
(`crates/sweep/tests/volume_premises_over_a_box.rs`), so it is not
every boxed curved wall.

---
id: the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite
kind: issue
title: The side reader refuses a body thin in two directions whose pairs are all definite: no aimed ray reads it and its volume lies in band
status: open
opened: 2026-10-14
priority: P1
cost: M
needs_ev: true
---


Found by PR 5119's reviews (FLUSH `flush/shell-role-reader`, R1 and R2).
A design fork owns the answer; this row holds the evidence.

## The class

A body thin in two directions: a needle `w × w` across. Every pair of
its cells is `w` apart, definite above `Kε`, yet the side-at-infinity
reader (`crates/topo/src/side_reader.rs`) reads no side of it, so check 7
refuses its orientation:
- below about `20ε` no face is wide enough to offer an aim (a point
  certified more than `Kε` inside a planar face);
- above it every aimed ray lands within `Kε` of a face's boundary (its
  crossing is set aside, `SetAsideWhy::Crossing`, `point_in_loop_boundary`
  in band), or crosses a sharp corner in band (`SetAsideWhy::Corner`);
- the volume over area, `≈ w/4`, lies in band, so the fallback refuses
  too.
The reader reads rays set aside only where the ratified letter allows
(the volume only where no face offers an aim); whether the volume may
answer after rays are set aside is the fork's other question.

## Measured (branch `flush/shell-role-reader`, ε 1e-9 and 1e-6)

**At rest.** `brick (0,1)×(0,w)×(0,w)` (review R2's `r2_needles`):

| w | right side out | inside out |
|---|---|---|
| 10.5ε, 12ε, 15ε, 19ε | `OrientationUndecided`, volume in band, no face offers an aim | the same, volume negative |
| 21ε | refuses, all six rays set aside (crossings in band of a boundary) | the same |
| 25ε | passes | `NegativeVolume` |
| 30ε | refuses, every ray set aside | the same; at ε 1e-6 the ray aimed at 4v1 is set aside at the corner of 1v1 and 5v1 |
| 40ε | passes | `NegativeVolume` |

The refusal is not monotone in `w`: 25ε reads and 30ε does not.

**At a boolean's door.** `brick ∩ brick` leaving a needle (review R1's
`r1_a_needle_result_at_the_door`, three poses): at 1.2Kε and 1.5Kε the
door refuses earlier (`Containment`); at 2Kε every pose escalates
`SolidSide` (no face offers an aim, the volume in band); at 3Kε the axis
pose escalates at ε 1e-9 and the oblique pose at ε 1e-6, the others
build; at 5Kε all build.

**Sweep's needles.** `fillet_h6_cap_rim` (above the crossover, K = 3) and
`review_blend_k_rk_probes` (the all-transverse rims): each extrudes a
needle a few `Kε` across, which refuses at rest
`OrientationUndecided { Volume { margin: None, aimed: 0, set_aside: [] } }`
— no face offers an aim, and its volume straddles zero by its own
rounding.

**A face boundary.** `shell7_r2_probes::p3b` at ε 1e-12: the
revolve-door torus `R = 1e-10, r = 2e-11` (20ε thick) refuses at rest
`OrientationUndecided(Volume)`. Its two torus faces (one per arc of the
profile) aim at the same point, `(−8e-11, 0, 0)`: the inner equator,
which is their shared boundary, where the aim's default minor-window
middle (`v = π`) falls. Each ray's crossing there is set aside in band
(`bool_ray_torus_split`, a quadratic factor's discriminant of the
ray × torus quartic, 8.0e-12 < Kε); review R2 reads the same point with
a ray along the face's normal. Its volume over area lies in band too.

## Pins

`crates/topo/tests/shell_role_reader.rs` (`a_needle_refuses_its_orientation_at_rest`,
`an_inside_out_needles_corner_is_set_aside_not_a_pair`,
`a_boolean_leaving_a_needle_escalates_its_side`),
`crates/sweep/tests/fillet_h6_cap_rim.rs`,
`crates/sweep/tests/review_blend_k_rk_probes.rs` and
`crates/sweep/tests/shell7_r2_probes.rs` pin today's refusal, so the
fork's answer shows where it moves.

---
id: the-half-angle-ladder-certifies-in-band-configurations
kind: issue
title: The shared half-angle ladder certifies Miss (and roots) for configurations inside the zero band, where the trilean says escalate
status: open
opened: 2026-10-02
---


Found by the dual review of PR 3752 (TANG's circle × cylinder cell),
which routes circles tilted to a cylinder wall through the shared
ladder (`crates/topo/src/boolean/circle_roots.rs`, `half_angle_roots`).
The ladder is the circle × torus door's too, so the finding is the
shared core's.

## What

D1's trilean says a configuration inside the zero band is escalated,
never answered. The ladder decides the count on a discriminant and its
depressed-quartic rows rather than on the residual's range, and two
in-band configurations get definite answers:

- **A crossing by LESS than the zero band is certified `Miss`.** Tilted
  circles dipping into a wall by up to 4.07e-10 m at ε = 1e-9 were
  answered `Miss` on 1,287 of 7,770 poses (review 1,
  `circle_cylinder_probe.rs`, `probe_sub_band_crossing_read_as_miss`, a reviewer probe outside the tree).
- **An exact tangency reads `Miss`** once the circle is tilted past the
  band, at tilt ≥ 1.01ε (review 1, `probe_focus_tangent_ladder`).
  Square to the axis the cylinder door takes its first-harmonic arm,
  which escalates both: it decides on the exact range.
- **Accept-in-gap certifies roots off by more than ε.** At ρ = 1500,
  roots certified 1.09–1.23e-9 m from the oracle at ε = 1e-9, with the
  noise reading in the gap (review 2, `topo_dr2_probe.rs`,
  `dr2_gap_noise_root_error`, a reviewer probe outside the tree). That is the posture
  `circle-torus-meters-accept-an-unreadable-reading` holds open.

None of these answers is wrong outside the band. Across about 13k
probe poses, both reviews found no wrong answer at a configuration
outside the band.

## What would close it

Decide the ladder's Miss and tangency verdicts on the residual's range
at the certified extremes (as the first-harmonic door does), or charge
the discriminant's in-band reach and escalate it. Until then, a caller
needing an in-band tangency escalated must not route it through the
ladder. The cylinder door's square arm is that routing for circles
square to the wall
(`crates/sweep/tests/tang_circle_cylinder.rs`,
`a_rim_circle_tangent_to_a_parallel_wall_keeps_the_pierce_door`).

---
id: the-half-angle-ladder-certifies-in-band-configurations
kind: issue
title: The shared half-angle ladder certifies Miss (and roots) for configurations inside the zero band, where the trilean says escalate
status: closed
opened: 2026-10-02
priority: P2
cost: M
closed: 2026-10-10
branch: germ/half-angle-meters
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

## Outcome (2026-10-02, REACH's ellipse lane, PR 3805's fix pass)

The ladder no longer answers. `circle_roots::half_angle_roots` runs it
for its escalations, then takes its answer from
`circle_roots::certified_subdivision`, which decides on the residual
itself: a piece is root-free when `|F(m)|` exceeds the most `F` can
fall over it, monotone when the least `|F′|` on it is provably
positive, and a monotone piece's root is bisected and must read ON the
surface. A piece neither clear nor monotone, split down to the band,
holds a double root and answers `Uncertain`. So each of the three
readings above is answered `Uncertain` (or by an escalation the ladder
raises first), and a certified root reads on the surface. The REACH
review's measurements, on the same door: certified roots up to 1.9e-6 m
off the wall at ε = 1e-6 (an eccentric millimetre ellipse); `Miss` at
in-band grazes of 50 m and 500 m walls (circle) and a 5 m wall
(ellipse); `CountDisagrees` across definite 1e-8–2e-8 m crossings. Rows:
`conic_quadric::circle_wall_rows::a_graze_is_read_by_its_depth`,
`conic_quadric::ellipse_rows::a_graze_is_read_by_its_depth`, and
`conic_quadric::ellipse_rows::fuzz_rows` (a true-distance oracle at three bands, with
the counterexample's seed pinned). This row can close with that PR.

## Closed (branch `germ/half-angle-meters`, 2026-10-10)

Re-measured on main through the current door (`conic_quadric_roots`,
the ladder for its escalations, `certified_subdivision` for the
answer), against the double-double oracle (`conic_oracle::exact`), at
ε = 1e-6, 1e-9 and 1e-12 (`Band::new(ε, 10ε)`). Circles tangent to a
wall of radius 0.01–100 m at a chosen signed depth, ρ 0.01–2000 m, at
tilts drawn over the whole turn, 6,000 poses per ε:

- **Sub-band crossings** (|depth| ≤ ε, about 1,500 per ε): no `Miss`,
  no certified count. Every one answered `Uncertain` or escalated.
- **Exact tangencies** (depth 0, about 1,500 per ε): no definite
  answer. Every one `Uncertain` or escalated, at every tilt, the review's
  1.01ε included.
- **Gap depths** (ε < |depth| < 10ε): the same, no definite answer.
- **ρ = 1500 against a unit wall** (3,000 poses per ε, depths 0.01ε to
  0.1 m): at ε = 1e-9, where the noise reads in the gap on every pose,
  the worst certified root is 3.5e-10 m from the oracle (the review saw
  1.09–1.23e-9 m). At 1e-6 the worst is 1.9e-9 m. At 1e-12 it is
  1.33e-12 m, on two roots that are 0.64 and 4 ulp of θ from the
  oracle's: that is the f64 parameter's own grid at this radius
  (ρ·ulp(2π) = 1.3e-12 m), not the door. No `Miss` on a true crossing
  at any ε.

What decides these is not the width row. A graze's in-band stretch of
arc, about √(Kερ), is far wider than the band's arc. The walk refuses
there because no split share of the piece reads a definite sign. The
slack meter refuses a root it cannot place: made to accept its `Err`,
it certifies roots 1.08–1.10e-9 m off at ε = 1e-9, which is the review's
reading back.

Rows, in `crates/topo/src/boolean/conic_quadric/in_band_sweep.rs`,
each with the mutant that turns it red:

- `no_pose_inside_the_band_is_answered` (fuzz): red when a piece with no
  definite split share reads as clear (`continue` for the `Uncertain` at
  the split in `certified_subdivision`).
- `an_exact_tangency_is_escalated_at_every_tilt` (edge table): red under
  the same mutant, with 159 tangencies answered.
- `a_circle_of_radius_1500_places_its_roots_within_the_zero_band`
  (fuzz): red under the same mutant (72 wrong), and when the slack meter
  accepts `Err` (50 wrong, up to 1.10e-9 m at ε = 1e-9).

---
id: circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12
kind: issue
title: At eps 1e-12 the circle x sphere root-slack meter refuses a near-tangent snowman the default band builds
status: closed
pr: 3847
opened: 2026-10-01
branch: reach/circle-sphere-slack
refs: [circle-torus-root-slack-crowds-the-zero-band-at-1e-12, sphere-union-sphere-refuses-though-the-section-is-closed-form, f64-cannot-place-a-shallow-crossing-within-the-finest-band, circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound]
closed: 2026-10-02
---

The sphere instance of `circle-torus-root-slack-crowds-the-zero-band-at-1e-12`,
found by the PR 3659 review: the same meter shape, crowding the band
at the finest ε row, but here it REFUSES rather than flagging.

## Measured (review of PR 3659 at `7ac4b0ce`)

Two full-revolve balls, r 1.0 at `y = 0` and r 0.8 at `y = 1.8 − δ`
(the near-tangent snowman, `crates/sweep/tests/snowman.rs`):

- at the default band (ε 1e-9) every op builds down to `δ = 1e-6`, and
  `δ = 1e-8` escalates on `bool_vertex_face_side` (pinned by
  `the_near_tangent_family_builds_to_1e_6_and_escalates_at_1e_8`);
- at ε 1e-12, `δ = 1e-7` refuses at the pierce door. The roots exist
  (the extremes are definite), but `bool_circle_sphere_root_slack`
  — `ρ·noise / √(A₁² − c₀²)`, with the slope shrinking as `√δ` near
  the pole — is no longer definitely inside a band of 1e-12, so
  `circle_sphere_roots` answers `Uncertain` and the arm keeps its door.

So the build window narrows at the finer band instead of widening,
which is the opposite of what a finer tolerance should buy. The noise
the meter charges is `f64` rounding (`NOISE_ULPS` half-ulps of the
terms), a fixed absolute quantity, held against a band that shrinks
with ε: at 1e-12 the rounding meter rather than the geometry decides.

## What a fix has to look at

Whether the charge is tighter than `NOISE_ULPS · u · terms` for this
first harmonic (its chain is a squared norm and a dot product), or
whether the root's position needs a refinement step (one Newton step
on the exact residual) before the slack is metered — the torus row
asks the same question of its quartic.

## Evidence (2026-10-02, the dual review of PR 3752): the cylinder instance

The circle × cylinder door's square arm takes the same first-harmonic
meters (`circle_roots::first_harmonic_roots`). At ε = 1e-12 it certifies
none of 8,120 near-tangent poses (review 1, `circle_cylinder_probe.rs`,
`probe_tilt_band_and_near_tangency`, run at `CAD_TOLERANCE_EPS=1e-12`; a reviewer probe outside the tree). Every one is refused
`Uncertain`, none is answered wrongly. It is the same crowding: the
root-slack charge at a shallow crossing is in the gap of the finest
band.

## Closed (2026-10-02, PR 3847)

At ε 1e-12 the near-tangent snowman now builds at δ 1e-5 and 1e-6 under
every op, checked against the cap closed form and against the section
circle read off the radii. At ε 1e-9 the ×1e3 and non-unit-radius
near-tangent pairs main refused also build. The meter charged half-ulps
of the whole harmonic's term bound, and computed the root as
`acos(−c₀/A₁)` near −1. It now reads the extremes factored as
`(D∓ − r)(D∓ + r)/2r`, each with a running rounding bound
(`geom_core::running`), plus a charge for the frame's own defect. The
slack is the near extreme's error over the slope, and the root is
measured from the extreme nearer zero.

δ 1e-7 at ε 1e-12 is the f64 floor, not this meter. It is filed with its
design question as `f64-cannot-place-a-shallow-crossing-within-the-finest-band`
(P3). Siblings filed:
- `circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound`;
- `work/hone/line-roots-carry-no-root-slack-meter`.

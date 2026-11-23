---
id: a-built-sliver-is-not-a-legal-operand
kind: issue
title: A boolean result a sliver thick passes the result gate though its at-infinity side cannot be decided, so it is not a legal operand
status: parked
blocked_on: [a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area]
opened: 2026-10-09
priority: P1
cost: M
refs: [4335]
---


Found by PR 4344's dual review (r2 m5), measured on its fix pass
(`join/sphere-pair-whole-circle`, release, default ε).

## Measured

The unit ball at the origin against `ball(50)` on the `z` axis, both
`y`-poled, `2·10⁻⁶` from tangency:

| pose | op | result |
|---|---|---|
| `d = 51 − 2·10⁻⁶` (inside external tangency) | a ∩ b, b ∩ a | a lens about 1.2·10⁻¹¹ m³ and 2·10⁻⁶ thick: tier 2, tier 3′ and the certificate pass, the volume is right |
| `d = 49 + 2·10⁻⁶` (outside internal tangency) | a ∖ b | the same shape of sliver, the same readings |

Uniting either sliver with a brick far away refuses
`ShellWitnessExhausted { in_band: 26, first_in_band: Escalated {
bool_point_in_solid_infinity_enclosure } }`: the at-infinity probe's
enclosure of the volume is about ±1·10⁻⁵ around a value of 1·10⁻¹¹, so
no ray decides which side is outside. The result gate computes the
volume (`VolumeUncomputable` would refuse) but never asks whether its
sign is decided, so it ships a body the next boolean cannot read.
`crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
`a_large_ball_near_external_tangency_is_pinned`, pins the ∩ as
`operand=false`.

Main refused this pose `SpheresMeet` before PR 4344; the sliver
appears through the cut-in and the re-chart alike, so it is the gate's,
not the door's.

## What a fix owes

The result gate refuses a body whose at-infinity reading it cannot
decide (the reading `point_in_solid` and the containment fallback spend,
`solid_contain::at_infinity_side`), as `Escalated` with that reading's
margin. That is the shape [ev] PR 4335 argues for in-band results
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`).
The rows are this pose's ∩ and a ∖ b, and the near-tangent batteries'
slivers.

## Measured (2026-10-10)

CLEAVE lane `cleave-sliverop` (branch `cleave/sliver-not-an-operand`),
main `9192ca4d3a`, release.

**The sliver is not in band. The kernel cannot measure it.** The table
is the pose above at ε = 1e-9, against the textbook lens (mpmath, 40
digits).

| gap | true V/A | kernel f64 V, a∩b / b∩a | certified V/A | operand |
|---|---|---|---|---|
| 2e-6 | 5.0e-7 (50 Kε) | −54% / −86% | [−9.2e-6, 9.7e-6] | false |
| 3e-6 | 7.5e-7 | +29% / +8% | straddles 0 | false |
| 5e-6 | 1.25e-6 | +15% / +7% | straddles 0 | false |
| ≥ 1e-5 | ≥ 2.5e-6 | +6% / +14% | decides | true |
| ≤ 1.5e-6 | | | | the op refuses earlier (`ps_center_gap`, or `VolumeUncomputable` on `props_rim_level`) |

`a ∖ b` at `49 + gap` reads the same. At ε = 1e-6 the pose refuses
`Escalated` at `bool_sphere_sphere_gap`, which is honest. At ε = 1e-12
it refuses `CurvedPierceUnsupported`. So the sliver reaches the gate
only at the default ε.

**Root cause.** The role read re-derives each face's flux about the
shell's corner. The unit sphere's faces enclose theirs to 2e-15. The
`R = 50` faces enclose theirs to ±3.5e-10, around about 1e-11.
`geom_brep::props::curved::sphere` takes `flux = ±R·area + (c − o)·A⃗`
(`curved.rs:2674`), two terms of about `R·A` that cancel by `h/R ≈ 8e-10`.
Its `area = R²·Δu·(hi − lo)` also cancels at the chart's pole, where the
re-chart puts the cap. That is Q1's conditioning defect, filed as
FLUXTAIL `a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area`.

**This row's premise is corrected.** The fix this row first named, a
gate refusal `Escalated` with the at-infinity margin, is not the shape
[ev] PR 4335 gives. That PR's honesty condition (designer B, and Q1 at
`docs/DESIGN.md:1872`) says an amplified kernel error "typed Escalated
would tell the user their input is ill-conditioned when the kernel's
arithmetic is". Here the true `V/A` is 50 Kε, definite. The undecided
enclosure is the formula's.

**The three questions.**
- **Where the gate reads this.** A single-shell solid's sign is decided
  only by check 7, off the f64 closed form with `volume_pad = 0`.
  Check 10's certified role read skips single-shell solids
  (`validate.rs` `shell_winding_errors`). Where it does run and leaves an
  enclosure undecided, `ops.rs` `finding_arm` already types it the
  kernel's (`ShellRoleUndecided`, no certified sliver).
- **The at-infinity reading is the right test.** It is the same
  certified role walk as `classify_shells`, and both refuse with the
  identical enclosure. A decided f64 volume sign does not imply it:
  check 7 read 2.3e-7 and 7.2e-8 "definite", off wrong values. Once the
  closed form is re-posed, the gap 2–5e-6 slivers should decide at
  50–125 Kε and become legal operands, with no gate change.
- **Would a gate refusal cost readable results?** That was not
  measured, because no gate change is made. On this body a ray that
  meets the boundary still answers (`point_in_solid` at
  `(0, 0, 0.999999)` reads `In`).

The certified role read on single-shell results at the gate, and how
it should be typed, is put on TALLY's
`a-threshold-straddling-in-band-shell-is-typed-the-kernels`.


## Measured (2026-10-11, `flux/sphere-arm`, PR 4573)

The sphere arm's Gauss–Bonnet form reads a region of at most a
steradian by a fan of triangles inside it (`props::curved::solid_angle`),
whose enclosure carries the region's own digits. On this pose, release,
ε = 1e-9, against `lens(1, 50, d)`:

| gap | a ∩ b | b ∩ a | operand |
|---|---|---|---|
| 2e-6 | +1.8e-9 | −9.0e-10 | true, both |
| 3e-6 | −2.9e-9 | +1.3e-9 | true, both |
| 5e-6 | +1.5e-9 | +2.9e-10 | true, both |
| 1e-5 | −8.0e-10 | +2.5e-3 | true, both |

`spheres_crossing_off_every_edge::a_large_ball_near_external_tangency_is_pinned`
now pins the ∩ `OK SOUND … operand=true`. The `b ∩ a` residue at
1e-5 is the flux's own cancellation, `R·Area` against `c·A⃗`, which
the fan does not touch: FLUXTAIL's
`a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area`
still owns it.

## Planar witnesses, and a hypothesis about the margin (FLUSH, 2026-10-11)

These are from the FLUSH thin-hollow design fork (fork log row 113),
measured by a designer lane on main at ε = 1e-9 after PR 4636. Each
body below was built at rest, then unioned with a cube at `(5..6)³`:
- a slab `t = 1.5e-8` thick refuses `ShellWitnessExhausted`;
- a closed hollow with walls `δ = 1.5e-8` refuses the same way;
- a 1 m cube carrying one fin of the same thickness builds, and a bar
  cut through the fin's wall builds too.

The designer reads the failing witness's in-band margin as `t/2`, half
the wall's thickness, for a test point 5 m from the body. The question
"which side is that point on" is not in doubt there.
**Hypothesis (not traced line by line):** `point_in_solid` / the ray
walk takes its margin from the operand's wall thickness, not from the
point's distance to the boundary. That would be the same family as
`point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face`
(closed by PR 3866). If so, a body thin everywhere is not a legal
operand because of a measuring defect, not because of its geometry.
FLUSH's fork row 113 depends on the answer.

## Measured (CLEAVE `cleave/missed-ray-side`, 2026-10-11)

The FLUSH hypothesis above is the at-infinity reading
(`solid_contain.rs` `at_infinity_side`, margin `V/A`), as
`a-missed-ray-reads-the-solids-volume-sign-for-its-side` traced it. On
that branch, where the volume does not decide, a ray aimed at a planar
face reads the side. The slab and the thin-walled hollow are sided at
every ε, and `hollow ∪ brick` builds (`crates/topo/tests/missed_ray_side.rs`).
The sphere lens has no planar face, so it keeps the volume's reading,
and this row still waits on FLUXTAIL.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). it is HELDRECORDS' charter (every door gates what it returns); id and body unchanged. (CLEAVE orchestrator)

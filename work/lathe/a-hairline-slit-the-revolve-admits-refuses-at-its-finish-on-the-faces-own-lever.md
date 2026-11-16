---
id: a-hairline-slit-the-revolve-admits-refuses-at-its-finish-on-the-faces-own-lever
kind: issue
title: The revolve admits a slit window on r_max that tier 3 meters at each face's own lever, so a hairline lune refuses at the finish as VolumeUncomputable
status: open
opened: 2026-11-14
priority: P3
cost: M
design: true
---


Found by BOXFINISH's measurement of its `f64` hairline row (2026-11-14,
main `b363f5fa`). That row asked for the bool5r hairline revolves to
"assert their body again"; the measurement says the finish's refusal is
an honest reading of geometry in the band, so that ask is withdrawn and
what is left is this question.

## Measured (default ε = 1e-9, the `sweep::revolve` door, `f64`)

- **`bool5r2_wedge_reach`'s hairline** (a dimpled ring, l = 100, a
  sphere dimple R 0.01, θ = 0.5ε/R = 5e-8): the door refuses
  `NotFinished [VolumeUncomputable { source: Face { face: FaceKey(3v1),
  source: SphereLoop { what: "props_sphere_loop_cusp" } } }]`, one
  finding. Raised by `require_cusp_free`
  (`crates/geom-brep/src/props/curved.rs`) from
  `SignCertificate::at_round_zero`: the decision is `Zero` on
  `Value(5e-10)`, lever `Lever::curvature_radius(R)`, the sphere's 0.01.
  The true value is `2·sin(θ/2)·R` = 5.0e-10 m: the dimple's two
  meridians are 5e-10 m apart at its equator.
- **`bool5r1_probes`** (dimple r 0.1, w 5): a slit of τ − angle = 6.0e-9
  refuses the same way (margin 5.99999985e-10, true 6.0e-10); 3.0e-8
  escalates in band (margin 3.000000006e-9); 5e-7 builds.
- The aimed rays read `Side(Out)` on every one of these bodies: check 7's
  side is decided, and `VolumeUncomputable` is the certificate walk's.

## The question

The revolve's own admission meters the slit at the profile's `r_max`
(100 on the ring), where it is wide, and admits it. Tier 3 meters each
face at its own lever, and the dimple, which sits near the axis, is in
the band. So the user is told `VolumeUncomputable` with a sphere-loop
cusp, at the finish, about a slit the door's own admission passed.
Whether the door should meter its slit where tier 3 will, or say
nothing and leave the finish to refuse, and what the refusal should
name as the lever (the angle), is open. The three pinned tests
(`bool5r2_wedge_reach` ×2, `bool5r1_probes` ×1) pass as written, and
`bool5r2_wedge_reach.rs`'s doc still offers `LaminaWedge` as a possible
first finding, which is not reported today.

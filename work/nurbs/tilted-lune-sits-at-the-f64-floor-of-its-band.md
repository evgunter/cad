---
id: tilted-lune-sits-at-the-f64-floor-of-its-band
kind: issue
title: mesh8_coherence's tilted_lune fixture sizes its radius so the certification residual sits two ulps of R from the band, so whether it builds turns on ulps; it went red on hosted CI but green locally under one eval spelling, which D9's bit-identical libm should not allow
status: closed
opened: 2026-10-10
priority: P3
cost: E
closed: 2026-10-12
branch: nurbs/tilted-lune-margin
pr: 4802
---

Both designers of the 2026-10-10 eval-spelling fork found this independently.

- `crates/topo/tests/mesh8_coherence.rs` `tilted_lune` sizes its sphere's radius from ε: `0.75·ε/3.2e-16`, which is 2.3e6 m at ε = 1e-9. The test's own comment says it does this so that a few ulps of R sit at the certification band.
- That design means the fixture measures the f64 floor, not a spelling's merit. A respelled `RevolvedPoint::eval` that is 1–2 ulps of R different (2.7–3.2 ulps against 1.4–1.8) stops certifying at some azimuths.
- PR 4441's lane reported this row went RED on hosted CI and stayed GREEN locally under the `I − R` spelling. Under D9 (pure-Rust `libm`), f64 should be bit-identical across platforms. So either the two runs were on different heads, or something in the path is nondeterministic. The second would be a real defect.

To do:
1. Reproduce the hosted/local split on one head, or show it was a head difference.
2. Give the fixture margin, for example R ≈ 1e6 (pole offset ≈ 0.32 ε, against the row's non-vacuity floor of 0.2 ε), or edges whose build does not turn on ulps.

(NURBS orchestrator, from designer reports on `nurbs/fork2-*`)

## Closed (NURBS, branch `nurbs/tilted-lune-margin`)

**1. The hosted/local split was a head and ε difference, not
nondeterminism.**
- PR 4441's red run (37975344172, head `538d4e2`, the `I − R`
  spelling) failed in CI's ε = 1e-6/1e-12 step with "the fixture must
  build; only 0 did".
- Its default-ε step failed too, on a row its retained log window does
  not show.
- At that spelling's head `212c9cdafa`, locally, the row is green at
  1e-9 and 1e-6 and red at 1e-12 with the same message, both at the
  dev profile and at CI's `opt-level = 1`. Local and hosted agree on
  the same head and ε. The "green locally" was a default-ε run.
- On main (`360cf55eb2`) the fixture is deterministic. A probe ran
  every lune's vertex bits and finding count at 1 and 4 threads, twice
  each, in two processes at each ε, and every reading was identical.

**2. The fixture no longer turns on ulps, and its non-vacuity is now
real.**
- **The pole offset now drives the build.** Measured on the old
  ε-sized fixture, the build fails once a pole vertex sits about 0.6·ε
  off the pole, and the old aim, 0.75·ε / 3.2e-16 of radius, landed
  them at 0.6·ε. Already on main, an eleventh azimuth (θ = 1.3 at
  ε = 1e-6) did not build. The four azimuths the row used happened to
  avoid it.
- **R ≈ 1e6 alone was not enough.** It moves the vertices to about
  0.3·ε, but the lever (`radial`, 0.03–0.09·ε, float noise) then
  leaves the suppressed quantity under the 0.2·ε floor at some ε
  (0.171 at 1e-12).
- **The floor itself was vacuous.** With the exemption deleted from
  `topo::coherence`, the old row stayed green at all three ε, because
  its suppressed quantity never cleared the door's own band
  (`gap·lever < ε`).
- **The new fixture places the pole vertices instead.** It uses a 1 m
  sphere with each pole vertex 0.5·ε off the pole, perpendicular to
  the axis, on the far side from the lune.
  - Every offset up to 0.99·ε builds at every ε, at eleven azimuths.
  - The row now requires all four lunes to build and all 16 pole
    junctions to fall inside the band.
  - It asserts a junction past the band (`gap_is_noise` false,
    `widest ≥ ε`).
  - With the exemption deleted, it reports a closure finding at every
    ε.

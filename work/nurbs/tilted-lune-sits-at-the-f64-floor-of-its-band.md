---
id: tilted-lune-sits-at-the-f64-floor-of-its-band
kind: issue
title: mesh8_coherence's tilted_lune fixture sizes its radius so the certification residual sits two ulps of R from the band, so whether it builds turns on ulps; it went red on hosted CI but green locally under one eval spelling, which D9's bit-identical libm should not allow
status: open
opened: 2026-10-10
priority: P3
cost: E
---

Both designers of the 2026-10-10 eval-spelling fork found this independently.

- `crates/topo/tests/mesh8_coherence.rs` `tilted_lune` sizes its sphere's radius from ε: `0.75·ε/3.2e-16`, which is 2.3e6 m at ε = 1e-9. The test's own comment says it does this so that a few ulps of R sit at the certification band.
- That design means the fixture measures the f64 floor, not a spelling's merit. A respelled `RevolvedPoint::eval` that is 1–2 ulps of R different (2.7–3.2 ulps against 1.4–1.8) stops certifying at some azimuths.
- PR 4441's lane reported this row went RED on hosted CI and stayed GREEN locally under the `I − R` spelling. Under D9 (pure-Rust `libm`), f64 should be bit-identical across platforms. So either the two runs were on different heads, or something in the path is nondeterministic. The second would be a real defect.

To do:
1. Reproduce the hosted/local split on one head, or show it was a head difference.
2. Give the fixture margin, for example R ≈ 1e6 (pole offset ≈ 0.32 ε, against the row's non-vacuity floor of 0.2 ε), or edges whose build does not turn on ulps.

(NURBS orchestrator, from designer reports on `nurbs/fork2-*`)

---
id: a-translated-arc-prism-panics-the-area-gauge-through-mass-properties
kind: issue
title: mass_properties panics (A2 area gauge, perimeter arm) on a small arc prism translated far from the origin
status: open
opened: 2026-09-24
---


Found and executed by ATREST-3's first dual reviewer (PR #3191),
filed by that unit's fix pass; not reproduced by the filer.

**Repro.** The `tcost_k3_certificate` arc prism
(`crates/sweep/tests/tcost_k3_certificate.rs`, `arc_prism_at`: two
`arc_section(s)` stacked and lofted at v-degree 1) at `s = 1e5·ε`,
translated `4e9·ε` along `+x` (4 m at the default ε), then
`topo::mass_properties(&body, Tol::witness())`.

**What happens.** A PANIC on a rayon worker inside the certified
quadrature: the A2 area tripwire's perimeter arm
(`crates/geom-brep/src/props/quad.rs`, `debug_assert_area_gauge` →
`area_gauge_ok` / `area_gauge_failure_message`) fires at about 1.33×
its ceiling. The same prism at the origin measures cleanly; the only
change is its distance from the origin relative to its size.

**Why it matters.** It is a panic through a public door
(`topo::mass_properties`, and every tier-3 door that reaches the
certified lane), not a typed refusal — fail-loud is right, but the
voice is wrong: either the gauge's calibration does not cover a body
far from the origin relative to its size (a false trip), or the
quadrature's area enclosure genuinely degrades with translation (a
real loss the gauge is correctly catching, which should then refuse
typed). Which of the two is the finding to settle first. `debug_assertions`
builds only (the `cfg!` guard), so a release build silently continues
past whatever the gauge saw.

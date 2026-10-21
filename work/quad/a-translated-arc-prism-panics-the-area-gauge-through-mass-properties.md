---
id: a-translated-arc-prism-panics-the-area-gauge-through-mass-properties
kind: issue
title: mass_properties panics (A2 area gauge, perimeter arm) on a small arc prism translated far from the origin
status: closed
opened: 2026-09-24
priority: P2
cost: M
closed: 2026-10-21
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

## Re-measured on main 258dc6c7c2 (QUAD, 2026-10-21)

**Verdict: gone. #5442 (FLUX's `shell_reference`, `topo::props`)
removed it.** The quadrature now reads each face's net about a vertex
of its shell, so the translation never reaches the lane. The
`ec32861499` run also settles this row's open question: it was a real
loss of the area enclosure, not a false trip of the gauge.

Probe: `crates/sweep/tests/quad_frame_remeasure.rs`
`row4_translated_arc_prism`. It builds this row's arc prism at
`s = 1e5·ε` (1e-4 m), translated along +x, and runs
`topo::mass_properties(.., Tol::witness())` under `catch_unwind`. The
build is release with debug assertions, which is the workspace's
`[profile.release]`, so the gauge is live.

| translation | main `258dc6c7c2` | main before #5442 (`ec32861499`) |
|---|---|---|
| 0 | V 4.570802e-12 ± 4.10e-15, A 1.736306e-7 ± 5.89e-10 | V ± 3.09e-15, A ± 5.74e-10 |
| 4 cm | identical to 0 | A ± 8.39e-9 |
| 40 cm | identical to 0 | A 2.284e-7 ± 5.98e-8 |
| 4 m | identical to 0 | **PANIC**, A2 gauge perimeter arm, width 1.15e-6 |
| 40 m / 400 m / 4 km | identical to 0 | PANIC, width 1.14e-5 / 1.14e-4 / 1.14e-3 |

On main every translation from 0 to 4 km gives the same volume, area
and pads to every printed digit. Before #5442 the area pad grew about
linearly with distance from 4 cm on, and the midpoint drifted 32 %
by 40 cm. That pre-#5442 drift is what this row asked to settle: the
quadrature's area enclosure degraded with translation, and the gauge
caught it correctly.

The gauge's calibration was never tested by this body. At the door,
the degradation survives as
`quad-rational-patch-cross-grows-with-translation`, where a caller
hands the lane a net far from the origin.

## Closed

Re-measured on main `258dc6c7c2` (section above): the arc prism at
s = 1e-4 reads the same volume, area and pads to every printed digit from
0 to 4 km, and the gauge no longer trips. FLUX's #5442 (`shell_reference`)
removed it: `topo`'s quad lane now hands each face's net to the lane shifted
to a vertex of its shell. On the commit before it, `ec32861499`, the area
pad grows from 4 cm out and the gauge panics from 4 m, so the enclosure
really did degrade and the gauge was right. What remains is the lane's own
frame dependence at its public door, which
`quad-rational-patch-cross-grows-with-translation` carries.

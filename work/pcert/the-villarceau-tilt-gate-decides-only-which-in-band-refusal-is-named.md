---
id: the-villarceau-tilt-gate-decides-only-which-in-band-refusal-is-named
kind: issue
title: The Villarceau tilt gate's Zero is observable only in a 0.2%-wide sliver where both lanes refuse in band, so it decides only which refusal is named
status: open
opened: 2026-11-23
priority: P3
---


Found by ONEROUND's PR 7268 (`oneround/unsourced-stand-ins`) and its delta review (2026-11-23). The gate is `pcurve_torus_villarceau_tilt` in `geom_brep::pcurve_cache::torus_oblique_circle`, read in the identity ladder after the chart's incidence test.

- **Where it can decide.** A linear program over every first-order move of a Villarceau circle (centre, plane, radius) at the incidence test's 65 samples, on `R = 2`, `r = 0.7`, gives the least sampled departure that puts each gate at `m`: `0.9966·m` for the tilt, `0.9985·m` for the plane, `m` for offset, equator and radius, tending to `m` as the samples densify. So the tilt gate's verdict is not the incidence test's only in a sliver about 0.3% wide at the band's zero. The move that reaches it (the centre `0.508·m` in toward the axis, the plane turned `0.534·m` about the centre's direction, the radius `0.488·m` short) is written down in `pcurve_villarceau::a_tilt_just_past_the_band_takes_the_general_lane`.
- **What it decides there.** The lane: at `1.0015·ε` the tilt reads in band and the circle takes the general lane (a projected image); at `0.99·ε` the closed form. The delta review certified both lanes' images of that row's circles at ε 1e-6, 1e-9 and 1e-12: at `0.99·ε` the closed form refuses (`MapResidual` escalated, about 1.02 to 1.18 ε, in band), at `1.0015·ε` the general lane refuses (`EnvelopeTerm(Incidence)` escalated, about 2.64 ε, in band). So in every outcome measured the gate changes only which in-band refusal is named; over the geom-brep and topo suites an arm `10⁶` times too short changes nothing.
- **The question for this program.** Whether the gate earns its place: whether it should stay as the lane's guard (and its row stay the pin of a lane choice), or go, with the incidence test and check 1 as the decision. PR 7268 keeps it and pins its lane choice; it does not decide this.

---
id: a-fitted-wall-has-no-section-with-a-moved-cap
kind: issue
title: C5 routes no Approx x Plane section, so every lofted shell refuses at its first moved wall: may a fitted chart's section be its fitted spline's, with the fit certificate as an error bound?
status: open
opened: 2026-10-08
priority: P1
cost: H
refs: [shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam]
---

Filed by the unit 9 designer pair. Once a moved cap's corner is solved
on the slanted seam (unit 9), `shell` moves each spline wall of a loft,
whose offset is a fitted (`Approx`) chart, and the cap–wall edge is a
plane × fitted-surface section. C5 routes no `Approx × anything` pair
(`crates/geom-brep/README.md`, C5: the composition "is not a ratified
rule"; O4's last sentence), so the edge refuses
`NeighborPairUnroutable` / `FittedBoundaryUnsupported`: every lofted
shell (twisted loft, vase; `crates/sweep/tests/encl_curved_loft_shell.rs`)
stops there, one face later than today. The fit's budget at the default
ε is untouched.

The question, a design fork for a designer pair and then Ev: may a
fitted chart's section be the section of its fitted spline, with the
fit's certificate counted as one term of the error bound? This decides
whether a loft shells at all. ENCL's
`a-rigid-map-can-still-refuse-…` stays parked on it.

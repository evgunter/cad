---
id: a-spiric-plane-near-miss-shift-is-measured-in-the-ovals-plane-not-off-the-cutting-plane
kind: issue
title: The spiric × plane door's near-miss reread shifts L by the band's escalation inside the oval's plane, which is a height off the cutting plane only times the tilt sine s: a sub-ε miss decides once s is below about 0.1
status: open
opened: 2026-11-23
priority: P1
---


Filed by GERM's `germ/co-surface-table` lane (PR 7431). This is the
residual of its review's MINOR 2, from the confirmation review at
`7f72931e9d`.

## Finding

`geom_brep::curve_face::spiric_plane::spiric_plane_method`
(`crates/geom-brep/src/curve_face/spiric_plane.rs:59`) rereads the
oval's count on `L = P ∩ Q` shifted by `±band.escalate()` along
`across`. `across` is L's normal **inside the spiric's plane P**. A
shift `d` across L in P moves the line only `d·s` off the cutting plane
Q, where `s` is the sine of the tilt between P and Q. So the reread
tests a miss of `escalate·s` off Q, not a miss of `escalate`.

The review ran one case at the default ε. The plane leaned to `s ≈ 0.2`
with the oval 3e-9 off Q, between the band's zero and its escalation.
It answers `Ok` with 0 meetings on one side and 2 on the other, so the
sub-band miss decides rather than refusing. A plane square to P
(`s = 1`) refuses at the same shift. By the same arithmetic, a miss
below ε decides once `s` falls below about 0.1.

The row `a_plane_missing_the_oval_by_less_than_the_band_refuses` (same
file) leaves this open: it sets its tangents square to P.

## Settles when

The reread shift is a height off Q: `±escalate/s` along `across`, or an
equivalent height. A row over `s ∈ {1, 0.2, 0.05, 0.01}` must refuse a
miss of the oval off Q between zero and escalate at every `s`.

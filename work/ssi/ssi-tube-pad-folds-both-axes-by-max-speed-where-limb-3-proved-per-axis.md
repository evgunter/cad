---
id: ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis
kind: issue
title: plane_nurbs_ssi pads its uniqueness tubes by max(su, sv) on both chart axes where limb 3 proved a per-axis pad, and the comment says they are the same region
status: closed
opened: 2026-09-12
priority: P1
cost: M
closed: 2026-10-01
pr: 3683
branch: ssi/chart-tube
---


## Finding

`crates/geom-brep/src/ssi.rs`, `plane_nurbs_ssi`: the chart-space tube
pad is `speed.to_param(branch.certificate.tube_radius)` with `speed`
the max-fold of the u and v derivative-box magnitudes, applied on both
axes. `crates/geom-brep/src/ssi/certify.rs`'s limb 3 proved the tube
with a PER-AXIS pad (`su.to_param(radius)`, `sv.to_param(radius)`,
each from its own box). Both sites were bare divisions until SCALAR's
`exhaustiveness-receipt-carries-its-lane` typed them through
`SupSpeed`, which moved no bits and settles nothing here. Dividing by
the larger speed on both axes gives a chart region
that is a SUBSET of the proved one — sound (the accounting is harder,
never easier) — but the comment beside the pad says it is "the same
region limb 3 proved", which it is not. Either the pad becomes
per-axis (the certificate would then carry the two speeds it was proved
with, which `SsiCertificate` currently does not store — every consumer
re-derives the speed) or the sentence says "a subset of". Found by the
SCALAR rate census, 2026-09-12; `ssi*` is TRIM's ground per PROPS'
`keep_out`.

## Design (2026-10-01)

Decided with three sibling rows; the spec is the "Design" section of
`ssi-chart-speed-usability-boundary`.

## Closed (2026-10-01, PR 3683)

The chart speed is minted once per axis (`NurbsBoxes::chart_speeds`, which refuses zero or non-finite by axis) at every door, and limb 3's NaN closure is gone. The certificate records `SsiTube::Chart { rung, pad_u, pad_v }`, and one `chart_tube_windows` builds the probe's and the accounting's windows, so the region banked is the region proved. Review was a single FULL review with one fix pass.

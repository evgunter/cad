---
id: ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis
kind: issue
title: plane_nurbs_ssi pads its uniqueness tubes by max(su, sv) on both chart axes where limb 3 proved a per-axis pad, and the comment says they are the same region
status: open
opened: 2026-09-12
---


## Finding

`crates/geom-brep/src/ssi.rs`, `plane_nurbs_ssi`: the chart-space tube
pad is `branch.certificate.tube_radius / speed` with `speed` the
max-fold of the u and v derivative-box magnitudes, applied on both
axes. `crates/geom-brep/src/ssi/certify.rs`'s limb 3 proved the tube
with a PER-AXIS pad (`radius / su`, `radius / sv`, each from its own
box). Dividing by the larger speed on both axes gives a chart region
that is a SUBSET of the proved one — sound (the accounting is harder,
never easier) — but the comment beside the pad says it is "the same
region limb 3 proved", which it is not. Either the pad becomes
per-axis (the certificate would then carry the two speeds it was proved
with, which `SsiCertificate` currently does not store — every consumer
re-derives the speed) or the sentence says "a subset of". Found by the
SCALAR rate census, 2026-09-12; `ssi*` is TRIM's ground per PROPS'
`keep_out`.

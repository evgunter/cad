---
id: interval-jet-hulls-kappa-sign-at-a-right-angle-crossing
kind: issue
title: at Interval a right-angle crossing described as a tangency refuses as an in-band tangent_second_order escalation, not at TangentParallel
status: open
opened: 2026-09-25
---


## Finding

`crates/geom-brep/src/tangent.rs`'s `tangent_jet` signs s2's normal
curvature by `σ₂ = 1.copysign(∇F₂·n̂)`. Where the normals are
perpendicular, `∇F₂·n̂` is an enclosure straddling zero at `Interval`,
so `σ₂` hulls to `[−1, 1]` and `κ_rel` hulls to `[−1/r, 1/r]`.

Measured on the ruled band's cut-off arc with the
`ContactCarrier::TransverseArc` mutant applied in
`crates/sweep/src/blend/surgery.rs`'s `attach_contact`
(`fillet_h7_transverse_cap_interval::the_rod_carves_at_the_certified_scalar_and_brackets_the_prism_closed_form`):
the carve refuses `Escalated { predicate: "tangent_second_order",
margin: Enclosure { lo: 0.0, hi: 0.0500… } }`. That is before
`crates/geom-brep/src/certify.rs`'s `TangentParallel` check (which in
f64 refuses definitely at margin `sin θ · r = 0.1`). The refusal is in
the safe direction, but its payload names an in-band second-order
reading where the cause is a definite first-order defect
(`sin θ = 1`). A reader following the refusal text would go looking for
near-osculating geometry.

## Fix shape

Either decide the parallelism defect before the second-order margin
whenever `sin θ` is definitely non-small at the folded lever arm (a
new reading, so the K-REPORT runbook applies), or let the tangent
arm report both readings when the second-order one is in band. The
C7 schedule's order (second-order first, as the lever's validity gate)
is the constraint either has to respect.

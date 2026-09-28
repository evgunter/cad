---
id: interval-jet-hulls-kappa-sign-at-a-right-angle-crossing
kind: issue
title: a right-angle crossing described as a tangency refuses at the second-order check (the osculating cause), not at TangentParallel, at Interval and in f64 with the band as s1
status: open
opened: 2026-09-25
priority: P3
cost: D
---


## Finding

The tangent arm of `crates/geom-brep/src/certify.rs` decides the
second-order margin (`tangent_second_order`) BEFORE normal
parallelism (`TangentParallel`), as the C7 schedule orders it. On a
false tangency whose defect is first-order, the second-order reading
can refuse first and name the osculating cause. The refusal is in the
safe direction, but its payload sends a reader looking for
near-osculating geometry when the cause is `sin θ = 1`. There are two
measured cases.

**At `Interval`, either order.** `tangent_jet` in
`crates/geom-brep/src/tangent.rs` signs s2's curvature by
`σ₂ = 1.copysign(∇F₂·n̂)`. Where the normals are perpendicular,
`∇F₂·n̂` is an enclosure straddling zero, so `σ₂` hulls to `[−1, 1]`
and `κ_rel` hulls to `[−1/r, 1/r]`. Measured on the ruled band's
cut-off arc with the `ContactCarrier::TransverseArc` mutant applied in
`crates/sweep/src/blend/surgery.rs`'s `attach_contact`
(`fillet_h7_transverse_cap_interval::the_rod_carves_at_the_certified_scalar_and_brackets_the_prism_closed_form`),
the carve refuses `Escalated { predicate: "tangent_second_order",
margin: Enclosure { lo: 0.0, hi: 0.0500… } }`. In f64 with the cap as
s1, the same arc refuses definitely at `TangentParallel`, margin 0.1.

**In f64, with the band (cylinder) as s1.** The jet's transverse
direction `n̂ × τ̂` is then the cylinder's ruling, along which both
Hessian forms vanish, so `κ_rel = 0` and the certificate refuses
`NotSecondOrderSeparated`. That is the osculating cause, for a
crossing at 90°. Pinned as a refusal only (current cause, not spec) by
`crates/geom-brep/tests/m5_pr12_circle_certificate.rs`'s
`a_right_angle_crossing_described_as_a_tangency_is_refused`, its
reversed-order half.

## Fix shape

Either decide the parallelism defect before the second-order margin
whenever `sin θ` is definitely non-small at the folded lever arm (a
new reading, so the K-REPORT runbook applies), or let the tangent arm
report both readings when the second-order one refuses. The C7
schedule's order (second-order first, as the lever's validity gate)
is the constraint either has to respect. The fix should tighten the
reversed-order row's assertion to `ResidualExceeded { TangentParallel }`.

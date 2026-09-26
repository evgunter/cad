---
id: tangent-certificate-refusal-cause-depends-on-surface-order
kind: issue
title: The TangentIntersection certificate names a transverse edge's refusal by argument order: NotSecondOrderSeparated one way, TangentParallel the other
status: open
opened: 2026-09-26
---


## Finding

Found by the sweep of
`work/encl/must-carry-over-edge-reads-a-transverse-edge-as-under-determined.md`
(order-dependent two-surface contact readings). `crates/geom-brep/src/certify.rs`,
the `TangentIntersection` arm of the per-sample certification walk, ACTS on
`crate::tangent_second_order(surf1, surf2, p, tau, extent, band)` —
`Ok(Zero | Negative)` returns `CertifyError::NotSecondOrderSeparated` — before
it meters `"tangent_normal_parallel"` (`CertCheck::TangentParallel`). It has
no first-order reading ahead of the jet. At a definite corner the jet's
transverse direction `n̂₁ × τ̂` lies in `surf1`'s tangent plane only, so its
`κ_rel` depends on which surface the description names first.

Measured on the ruled band's cut-off arc (a plane crossing a cylinder of
radius 0.1 at a right angle along a circle; `sin θ = 1.0`, arm 0.1 at every
station): `κ_rel = −10` with the plane as `s1`, `0.0` with the cylinder as
`s1`. With the plane first the certificate refuses at `TangentParallel`
(measured on PR 3270's head). With the cylinder first the same numbers put
the sagitta at zero, so the walk refuses `NotSecondOrderSeparated` before the
parallelism check runs — inferred from the measured `κ_rel` and the walk's
order, not run through the door.

Both orders refuse, so nothing false is stored. What differs is the CAUSE:
one geometric fact (the surfaces are not tangent) is reported as "not
tangent" in one order and as "tangent but osculating" in the other
(`memories/refusal-text-is-not-cause.md`). `topo::boolean::contact_verify`
answers the same question order-robustly: it decides first-order
(`contact_tangent_opposed`) and parallelism before it acts on the
second-order margin, falling back to the extent lever when `κ_rel` is not
definite — the shape a fix here could take.

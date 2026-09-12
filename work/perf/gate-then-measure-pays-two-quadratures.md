---
id: gate-then-measure-pays-two-quadratures
kind: issue
title: callers that validate_geometric then mass_properties pay the certified quadrature twice although the certificate door exists
status: open
opened: 2026-09-10
rides_with: PERF-6
---

## The finding

Measured by the PERF kernel lane (`perf/explore-kernel`; release, 4
vCPU). On a NURBS-walled body (`loft_prism`) `validate_geometric` costs
157 ms and `mass_properties` 160 ms: tier 3's check 7 IS a full
certified quadrature (`crates/topo/src/validate.rs` around the
`mass_properties_certified` call in `validate_geometric_certified`),
and the kernel already ships `validate_geometric_certificate` to hand
the gate's `MassProperties` back so a caller that gates and then
measures pays once. Callers that do not use it and pay twice:
`demos/tour/src/main.rs:402,423`, and the Python surface's natural
spelling `body.validate_geometric()` then `body.mass_properties()`.
The tour spends 2.2–2.7 s over 193 mass-props calls; half of that is
this.

## What a fix is

"Stop doing this": the tour takes the certificate from the gate it
already runs; `pncad-py` exposes the certificate door (or memoizes the
certificate on the validated body) so the natural Python spelling pays
once. Demo and binding code, no kernel change; `memories/demo-purpose.md`
says the awkwardness a demo hits is a library finding — the finding
here is that the one-quadrature spelling is not the natural one.

## Ruled (Ev, 2026-09-12, in chat): the demo reports the bracket

After PERF-6 the gate hands back a `SignCertificate`, and its
continuation (`refine_to_target`) can REFUSE `QuadratureBudget` on a
tier-3-valid body: the reporting target `1024·ε` is a length that
scales with ε while the fixed 12-round schedule's floor is a
part-size property (the teapot spout: floor 2.53e-8 m, target
1.024e-9 m at ε = 1e-12, refused 25× under; certifies with 40× room
at 1e-9). Ev chose, over letting the round cap grow with ε: **the
tour takes the sign certificate from the gate it already runs and
continues it; when the continuation refuses budget on a tier-3-valid
body, the volume ribbon prints the sign-level enclosure (`lo`, `hi`,
area) and checks the mesh's signed volume against that bracket (with
the chordal slack) instead of panicking.** The reporting contract and
its schedule do not change. The Python pair takes the same shape
(`SignCertificate<'b, T>` borrows the body, so the binding exposes the
continuation door rather than memoizing a certificate on the body).
This is the consumer change PR 2306's ε sweep waits on at 1e-12.

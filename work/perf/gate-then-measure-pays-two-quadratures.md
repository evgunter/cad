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

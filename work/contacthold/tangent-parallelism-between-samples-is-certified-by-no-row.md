---
id: tangent-parallelism-between-samples-is-certified-by-no-row
kind: issue
title: C7 claims tangency along the whole edge, but normal parallelism between schedule samples is bounded by no row
status: open
opened: 2026-10-11
priority: P4
cost: M
---


Found by the review of PR 4640 (N4); disclosed in its body.

C7 (`crates/geom-brep/README.md`) certifies a `TangentIntersection`
with "C2's hull bounds and tube between samples", which reads as a
whole-edge claim. Between schedule samples the certificate bounds:

- each residual (`tangent_hull_sup`, `geom_brep::certify`'s tangent arm);
- the pair's separation (`tangent_gap_hull_sup`, PR 4640);
- `|κ_rel|` (`tube_separation`, `TangentSpanBounds::kappa_drift`).

It does not bound the normals' angle. `TangentParallel` is read per
interior sample only (`check_residual("tangent_normal_parallel", …)` in
the tangent arm). The declared-contact verifier is the same
(`topo::boolean::contact_verify::tangent_locus_reading`, its first-order
arm). Where the normals turn through perpendicular between two samples,
there is no shared normal there, and the gap span line bounds
`|r₁ − σ·r₂|` under each end's σ, which is not a separation.

No in-lane constructor is known to mint such a configuration: coaxial
circles hold the relative orientation by symmetry, and rulings hold it
constant. So the gap is latent. Closing it needs a certified bound on
the relative normal turn per span. The Gauss-map Lipschitz bound
`|dn̂/dt| ≤ |dir|/arm` that `kappa_drift` already uses
(`crates/geom-brep/src/tangent.rs`, `tangent_span_bounds`) is the
natural ingredient: `sin θ` at the samples plus twice that turn over
the span, levered as the per-sample reading is.

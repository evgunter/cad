---
id: plane-nurbs-limb-refusal-carries-no-reporting-margin-for-the-import-door
kind: issue
title: geom-brep: the plane x NURBS certificate limb's definite refusal carries its miss as a bare f64, so the import door can only say the miss "may lie" within ε_in
status: open
opened: 2026-10-08
priority: P3
cost: M
---


(Filed by the ENCL fix-pass implementer on PR 4331, `encl/adoption-at-rest-eps-in`, under the fence the orchestrator set on that pass. Its F4 was to carry each definite residual's reporting margin to the import door, and to stop if that fanned out past `certify.rs`, `edge_nurbs.rs` and their direct readers.)

## What

D4 ¶1: a certification refusal whose miss lies within ε_in but beyond ε names setting ε to ε_in as a stopgap. At the import door that sentence is `geom_core::FileCoincidence::miss_recourse_in_file`.

PR 4331 hands `CertifyError::ResidualExceeded` its `MarginDiag`, through `dihedral::decide_reported` in `certify.rs`'s `check_residual`. Its definite arm therefore reads its own value: "lies within" ε_in, or the at-rest ending past it.

The plane x NURBS lane's limb refusal does not get the same treatment. `PlaneNurbsRefusal::Limb { limb, value }` (`crates/geom-brep/src/edge_nurbs.rs`:164) carries the measured bound as a bare `f64`. A bare `f64` in a payload is not a reporting margin, and the door may not compare it (`crates/geom-core/src/real.rs`, the `Bounds` scope rule, clause 2). The door therefore reads the limb's definite arm as `MissReading::DefiniteUnvalued` (`crates/geom-brep/src/recourse.rs`:120, `Unsized::residual_in_file`). It hedges "may lie within" wherever ε_in reaches past K·ε, even for a miss it could size.

## Where the margin would have to come from

The limb is decided in `crates/geom-brep/src/ssi/certify.rs`, which is upstream of the fence:
- `decide("ssi_on_locus", …)` at ssi/certify.rs:471;
- `decide("ssi_hull_sup", …)` at :497;
- the foot-point distance at :603;
- `ssi_foot_orthogonality` at :627;
- `ssi_hull_sup_chart` at :656.

Each mints `SsiError::CertificateLimb { limb, value }` (`crates/geom-brep/src/ssi.rs`:448), which `edge_nurbs.rs`'s `refusal` (edge_nurbs.rs:865) converts to `PlaneNurbsRefusal::Limb`.

Other readers of `CertificateLimb`'s payload:
- `ssi/refine.rs`:114 (`RoundMargin::Over`);
- `pcurve_cache.rs`:2706;
- `ssi.rs`'s ending tests;
- topo's `validate.rs` and `test_support_samples.rs`;
- step-import's `recognize_pins.rs`, `placed_m7_8_instance.rs` and `review_probes_m7_3.rs`.

## Repair shape

1. Switch those five sites to `decide_reported` and carry `margin: MarginDiag` on `SsiError::CertificateLimb` and `PlaneNurbsRefusal::Limb`. Keep `value` if `refine.rs`'s round margin still needs it.
2. Have `PlaneNurbsRefusal::ending_in_file` end the limb's definite arm through `Unsized::definite_residual_in_file`, as `CertifyError::ending_in_file` ends `ResidualExceeded`.
3. Once no definite residual refusal at the door is unvalued, `MissReading::DefiniteUnvalued` and its hedge sentence go.

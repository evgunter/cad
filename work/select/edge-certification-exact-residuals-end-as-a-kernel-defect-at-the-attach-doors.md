---
id: edge-certification-exact-residuals-end-as-a-kernel-defect-at-the-attach-doors
kind: issue
title: geom-brep: edge certification's exact-construction rows end as a kernel defect at a build, which a caller's carrier at the Euler attach doors reaches
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/geom-brep/src/certify.rs` `CertCheck` ending, `Ending::Residual(Unsized::Defect)` for `EndpointStart | EndpointEnd | WitnessSurfaceResidual | WitnessMidpoint | TangentParallel | TangentGap | MappedSource | ChartResidual` (~1347; one site, eight checks). **Class 1, sure** at the Euler attach doors, whose `EulerOpError` `Display` reads `Reading::Build` (`crates/topo/src/euler.rs` ~1808). `topo/tests/geometric_cube.rs` `wrong_cache_is_rejected_at_attachment` hands `Body::mev` a carrier 100ε off its endpoint and gets `ResidualExceeded { EndpointStart }`, whose text ends "this is a kernel defect". At the kernel's own builds the same rows are **class 2, likely**.
- `certify.rs`, `ChartImage` → `Unsized::Defect` (~1348). **Class 1, likely** at the attach doors, since a caller's `EdgeCurveSpec` carries its own description. **Class 2** at a kernel build.
- `certify.rs`, `PlaneNurbsReportedTransversality` → `Unsized::Defect` (~1351). **Class 2, likely**: a fold over samples that each decided transverse.
- `crates/geom-brep/src/edge_nurbs.rs` `PlaneNurbsRefusal::ending`, `PcurveFit` (~376). **Class 3, likely**, with a class-2 arm: a collocation the interpolation cannot solve is its limit, and a non-finite foot is a bug.

The attach doors' reading is TOPO's code (`euler.rs` `EulerOpError::render`); the table is this program's.

## Repair shape

- **Class 1**: a carrier the caller supplied to an attach door ends in the decision's lever (the carrier misses its endpoint or its surfaces, so move it). The door needs a reading of its own rather than `Reading::Build`.
- **Class 2**: at the kernel's own builds, a refused exact row panics per `[failure]` (`unreachable!`, answering row 0 first), and its tests move to `#[should_panic]`.
- **Class 3** (`PcurveFit`'s solve arm): `KERNEL_LIMIT_RECOURSE`, the last resort. Split the non-finite-foot arm off as class 2.

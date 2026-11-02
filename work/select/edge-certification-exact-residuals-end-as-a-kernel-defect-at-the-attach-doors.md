---
id: edge-certification-exact-residuals-end-as-a-kernel-defect-at-the-attach-doors
kind: issue
title: geom-brep: edge certification's exact-construction rows end as a kernel defect at a build, which a caller's carrier at the attach doors and user geometry at the kernel's own builds both reach
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/geom-brep/src/certify.rs` `CertCheck` ending, `Ending::Residual(Unsized::Defect)` for `EndpointStart | EndpointEnd | WitnessSurfaceResidual | WitnessMidpoint | TangentParallel | TangentGap | MappedSource | ChartResidual` (~1347; one site, eight checks). **Class 1, sure** at the Euler attach doors, whose `EulerOpError` `Display` reads `Reading::Build` (`crates/topo/src/euler.rs` ~1808). `topo/tests/geometric_cube.rs` `wrong_cache_is_rejected_at_attachment` hands `Body::mev` a carrier 100ε off its endpoint and gets `ResidualExceeded { EndpointStart }`, whose text ends "this is a kernel defect".
  **At the kernel's own builds these rows are not class 2 either.** User geometry reaches them there, as four open rows measure: `work/germ/conic-section-just-past-its-band-fails-certification-downstream.md`, `work/lathe/the-guided-derived-frame-extrude-escalates-as-a-kernel-defect.md`, `work/cleave/a-boolean-corner-on-a-curved-wall-is-read-against-the-wall-not-its-section.md` and `work/cleave/rejoining-a-tangent-seam-cut-refuses-the-union.md`. The germ row records 80 runs at 1e-12 of a split refusing `Join(Euler(Certification { Escalated { EndpointEnd } }))` / `ResidualExceeded { EndpointStart }`. The others record an extrude's and the Booleans' endpoint refusals. The construction's error outgrows the band there: class 3 (the germ row's argument), or class 1 where a lever moves the geometry. `topo::boolean::edge_join` `JoinRefusal::Kernel` wraps the same certification (HONE's row).
- `certify.rs`, `ChartImage` → `Unsized::Defect` (~1348). **Class 1, likely** at the attach doors, since a caller's `EdgeCurveSpec` carries its own description. At a kernel build it is unsure, for the reason above.
- `certify.rs`, `PlaneNurbsReportedTransversality` → `Unsized::Defect` (~1351). **Class 2, likely**: a fold over samples that each decided transverse. The per-sample transversality decision, earlier in the same lane, establishes it.
- `crates/geom-brep/src/edge_nurbs.rs` `PlaneNurbsRefusal::ending`, `PcurveFit` (~376). **Class 3, likely**, with a class-2 arm: a collocation the interpolation cannot solve is its limit, and a non-finite foot is a bug.

The attach doors' reading is TOPO's code (`euler.rs` `EulerOpError::render`). The table is this program's.

## Repair shape

- **Split by origin** with the existing `topo::policy_lane::Origin { Callers, Kernels }`, not a new reading.
- **`Callers`** (class 1): a carrier the caller supplied to an attach door ends in the decision's lever. The carrier misses its endpoint or its surfaces, so move it.
- **`Kernels`** (class 3): a refused exact row at the kernel's own build ends in `KERNEL_LIMIT_RECOURSE`, the last resort. It does not panic, since the rows above show input reaching it.
- **Ordering**: this repair does not wait on those rows. They remove triggers by tightening the constructions; they do not change which ending is honest today. Once they land, re-measure: if no input reaches a row at a build, it can become class 2 then.
- **Class 2** (`PlaneNurbsReportedTransversality`): panic per `[failure]`, after answering row 0, with its tests moved to `#[should_panic]`.
- **Class 3** (`PcurveFit`'s solve arm): `KERNEL_LIMIT_RECOURSE`. Split the non-finite-foot arm off as class 2.

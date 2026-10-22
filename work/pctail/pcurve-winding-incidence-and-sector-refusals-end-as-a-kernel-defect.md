---
id: pcurve-winding-incidence-and-sector-refusals-end-as-a-kernel-defect
kind: issue
title: geom-brep: pcurve certification's winding, incidence and sector refusals end as a kernel defect at a build, over carriers a caller may supply
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/geom-brep/src/pcurve_cache.rs` `PcurveCheck::routing`, `ChartWinding | ChartIncidence | Sector` → `Unsized::Defect` (~2439). `PcurveCertifyError`'s `Display` renders it at a build (~2310), for `PlacementTurnsExceeded` and for the escalations of all three checks. Doors: every pcurve mint at a build, including `EulerOpError::PcurveMint` over a caller's carrier at `Body::mev`/`mef`. **Class 1, unsure**: these checks read a carrier the caller may have supplied, and nothing excludes an in-band incidence or sector read for a carrier that edge certification passed.

## Repair shape

Decide the class first. Look for an attach-door input that reaches each of the three checks.

- **Class 1**: the check's lever (move the edge clear of the chart's singular set, or keep it within one period).
- **Class 2**, if the argument holds for a check: panic per `[failure]`, with its tests moved to `#[should_panic]`.

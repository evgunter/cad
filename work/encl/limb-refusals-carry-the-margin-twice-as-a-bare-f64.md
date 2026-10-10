---
id: limb-refusals-carry-the-margin-twice-as-a-bare-f64
kind: issue
title: geom-brep: SsiError::CertificateLimb, PlaneNurbsRefusal::Limb and AnalyticRung3Refusal::Limb carry value: f64 beside the margin, which refine.rs routes on
status: review
branch: encl/limb-margin-once
opened: 2026-10-10
priority: P3
cost: M
pr: 4529
---



(Found by a designer on design-fork row 104.)

## What

`SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` carry `value: f64` beside `margin: MarginDiag`, so the number is held twice. `ssi/refine.rs`'s `RoundMargin::Over(value)` routes on the bare `f64`, and the `Display`s print it. That conflicts with Bounds clause 2, which says the margin is error text only.

## Repair shape

Decide what `refine.rs` actually needs to route on: a decided verdict, or a refinement quantity that is not the reporting margin. Give it that, with one source of truth, and have the `Display`s read the margin. The foot-orthogonality limb is retired (PR 4517, under Ev's ruling in PR 4498), so `RoundMargin::Over` no longer sees a non-residual quantity.

---
id: near-tangent-pierce-poses-reach-three-classification-invariants
kind: issue
title: Near-tangent pierce poses reach ClassificationInvariant at three sites (an ON sector pair on distinct carriers, a pierce germ not within one sector, a coplanar sector on a distinct plane): 345 runs
status: open
opened: 2026-10-08
priority: P1
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by JOIN's near-tangent census measurement
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its
`## Measured`), on main `047d10d5`, release. These are outside that row's
question, and filed here because the sites are this program's.

Over the probe's 28 800 near-tangent runs at ε = 1e-9, 345 refuse
`ClassificationInvariant`, a kernel-defect refusal, at three sites:
- `boolean/recl.rs`, "geometrically-ON sector pair with
  definitely-distinct carriers": 162 runs (convex 45, w345 48, w60 69),
  at d = ±1e-5, ±1e-6 and ±1e-7. Witness: `convex nt e0 a4 d1e-5 pc U`.
- `boolean/vtxfac.rs`, "pierce germ direction not uniquely within its
  sector": 138 runs (Ltop 36, Lbot 30, Lmirror 24, asym 24, notch307 12,
  convex 6, w345 6), almost all at d = ±1e-9. Witness:
  `Ltop nt e0 a12 d1e-9 pc U`. `pierce-germ-direction-within-is-levered-at-the-sector-arm`
  (CONTACT) may be its cause.
- `boolean/vtxfac.rs`, "geometrically coplanar sector with
  definitely-distinct plane": 45 runs (convex 18, w345 9, w60 18), at
  ±1e-5 to ±1e-7. Witness: `convex nt e0 a3 d1e-5 pc U`.

At ε = 1e-12 and 1e-6 the same sites fire, at other tilts.

Repro: `NT_ONLY=convex NT_POSE="nt e0 a4" NT_D=1e-5 cargo run -p sweep
--release --example near_tangent_census_probe`.

## Owed

For each site, find which earlier decision disagrees with the later one,
and settle it: a typed refusal in band, or the two readings made one.

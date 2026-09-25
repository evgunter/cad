---
id: step-import-restates-the-cone-half-angle-convention
kind: issue
title: step-import's CONICAL_SURFACE arm restates the cone half-angle convention that geom now writes once
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [ATREST-6]
---

## What

`geom::Surface::representability_margins` (`crates/geom/src/surfaces.rs`)
computes each analytic kind's datum convention bound for the at-rest
check — for the cone, `half_angle` and `π/2 − half_angle`, both
required positive — and tier 3's check 1 reads it
(`ValidationError::UnrepresentableSurfaceDatum`,
`crates/topo/src/validate.rs`).

`step-import`'s `CONICAL_SURFACE` arm (`crates/step-import/src/entities.rs`,
`Resolver::surface`) restates the same bound as an `f64` literal,
`half_angle > 0.0 && half_angle < std::f64::consts::FRAC_PI_2`, and
refuses `MalformedRecord` on it. The two agree today; nothing keeps
them agreeing.

It is a mint-time check with a reason of its own — the apex derivation
below it divides by `tan(half_angle)` — so the refusal belongs there;
only its bound should be read rather than written. The arm checks the
angle before it has a `Surface` to ask, so the likely shape is to build
the cone at the placement location first, read its margins, and derive
the apex after; or a per-kind door on `geom` that `representability_margins`
and this arm both call.

Found by ATREST-6's sweep for restated convention bounds.

## Fence

EXCH (`crates/step-import`); the `geom` door is `props` ground and read,
not edited.

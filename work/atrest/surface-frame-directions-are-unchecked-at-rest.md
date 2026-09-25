---
id: surface-frame-directions-are-unchecked-at-rest
kind: issue
title: No tier-3 check reads an analytic surface's frame directions - a zero axis or u_ref is named nowhere, and geom's crate docs say tier 3 certifies them
status: dispatched
opened: 2026-09-24
priority: P3
cost: D
refs: [ATREST-6]
parent: ATREST-13
---

## What

ATREST-6 gave check 1 a poison read of every analytic surface's
stored datums (`poisoned_datums`, `crates/topo/src/validate.rs`): each
must be a finite number, and a plane's `normal` must not be the zero
vector. The zero-vector question is asked of the plane's normal ONLY,
because that is what the unit's spec scoped. Every other stored
direction — the `axis` of a cylinder, cone, sphere or torus, and the
`u_ref` of every analytic kind — is read for finiteness and nothing
else, so a zero axis or a zero `u_ref` passes check 1:

- a zero `axis` collapses a cylinder's `∂v` and a cone's generators,
  and puts a sphere's or torus's `v_ref = axis × u_ref` at zero;
- a zero `u_ref` collapses `radial(u)` for every axisymmetric kind and
  `v_ref` for the plane.

Neither describes the surface its variant names, which is the
plane-normal argument word for word.

The wider half: `geom`'s crate docs state that the frame fields are
"conventional data, unchecked here … **Tier-3 geometric validation
certifies the invariants at rest**" (`crates/geom/src/lib.rs`, the
unit-vector bullet). No tier-3 check reads a surface frame's
unit-ness or `u_ref ⊥ axis`. `validate_geometric`'s own not-yet list
(`crates/topo/src/validate.rs`, "Curve conventional-invariant
certification") names the CURVE half as not independently certified
and says nothing of the surface half, so the two documents disagree
and the crate docs are the one that overclaims.

## What must be decided

Whether a zero direction is the poison half (a datum that describes
no locus, named at check 1 beside the plane normal) — the likely
answer, and a two-line extension of `poisoned_datums` — and,
separately, whether unit-ness and orthogonality are tier 3's to
certify at all, or `geom`'s sentence is what should change.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1); the `geom` sentence
is `props` ground and a seam to announce.

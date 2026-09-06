---
id: transform-recertifies-through-the-narrow-lane
kind: issue
title: transform_rigid re-certifies through the plain certify door, which admits a strictly narrower class than tier 3
status: open
opened: 2026-09-04
---


Found while landing `transform-rigid-refuses-described-nurbs`, which is
what makes it reachable: until that unit, `transform_rigid` refused every
NURBS-carrying body at the surface arm, so the re-certification pass below
had never run against a described NURBS surface at all.

## The asymmetry

Edge-carrier certification has two doors, and they admit different classes:

- `geom_brep::EdgeCurve::certify` — no lane injected. An `Intersection`
  description resolves both operands through `resolve`
  (`crates/geom-brep/src/certify.rs:1262`), which refuses `Surface::Nurbs`
  outright with `CertifyError::Unimplemented`.
- `geom_brep::EdgeCurve::certify_nurbs_lane` /
  `recertify_nurbs_lane` — the M7-8 plane × described-NURBS lane wired in.
  The `Intersection` arm tries the lane FIRST
  (`crates/geom-brep/src/certify.rs:1350`) and only falls through to
  `resolve` when the operand pair is not plane × NURBS.

The kernel's own doors are split across the two, and the split is not
stated at any of them:

| site | door |
|---|---|
| `crates/topo/src/validate.rs:2920` (tier 3 at rest) | `recertify_nurbs_lane` |
| `crates/topo/src/euler.rs:2282` (`set_edge_curve_nurbs_lane`) | `certify_nurbs_lane` |
| `crates/topo/src/transform.rs:517` (`transform_rigid`) | plain `certify` |
| `crates/topo/src/boolean/combine.rs:433` | plain `certify` |
| `crates/topo/src/euler.rs:1990` (`set_edge_curve`) | plain `certify` |
| `crates/topo/src/seqgen.rs:612` | plain `recertify`, DELIBERATELY — and it is the one site that says so (`seqgen.rs:588`: the wider class "would let candidates past this gate that the operator then refuses") |

## The consequence

A body carrying an `Intersection` edge between a plane and a **described**
NURBS wall — the M7-8 class, minted through `Body::set_edge_curve_nurbs_lane`
and exercised by `crates/sweep/tests/m8_4_intersection_iso.rs` — validates
at rest through tier 3's lane door, and then **refuses at
`transform_rigid`** with `TransformError::Certify { source:
CertifyError::Unimplemented }`. A body the kernel says is valid is a body
the kernel cannot move.

Loft, sweep and skin bodies **at rest are not affected**: their wall edges
are `Chart` descriptions with a stored image, which resolve through
`resolve_iso` (`certify.rs:1272`) — the resolver that admits a described
`Surface::Nurbs` and refuses only the placeholder. That is the path
`crates/sweep/tests/transform_nurbs_walls.rs` exercises end to end, and it
passes tier 3 after the map.

## What this is not

It is not a case for loosening anything. The fix is to inject the lane the
rest of the kernel already injects — `transform_rigid` would take a
`T: Decide + geom_core::CertifiedBounds` bound and call
`certify_nurbs_lane` — which
adds no certification capability the at-rest validator does not already
have. That is a public generic signature change on a kernel door and was
out of scope for the unit that found it, so it is filed rather than taken.

`boolean/combine.rs:433` is the same shape and wants the same look; whether
its class is reachable was not established here.

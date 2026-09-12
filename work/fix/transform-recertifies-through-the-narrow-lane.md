---
id: transform-recertifies-through-the-narrow-lane
kind: issue
title: transform_rigid re-certifies through the plain certify door, which admits a strictly narrower class than tier 3
status: review
opened: 2026-09-04
pr: 2418
branch: fix/transform-nurbs-lane
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

## Taken (PR 2418) — with three corrections to the above

**The site table above is stale and was re-derived at the merge base.**
Two rows are wrong, not merely off by a line number:

| site | door, re-derived |
|---|---|
| `crates/topo/src/validate.rs` check 2 (tier 3 at rest) | **`recertify_via`** with an `Option<NurbsLane>` threaded from the caller — NOT `recertify_nurbs_lane`, which appears nowhere in `crates/topo/`. The lane-free door SKIPS an M7-8 edge (`needs_nurbs_lane` guards the call) rather than reporting it. |
| `crates/topo/src/euler.rs` (`set_edge_curve_nurbs_lane`) | `certify_nurbs_lane` — unchanged |
| `crates/topo/src/transform.rs` (`transform_rigid`) | plain `certify` — unchanged, and what this unit fixes |
| `crates/topo/src/boolean/combine.rs` (`graft_solids_with`) | plain `certify` — unchanged; filed as `graft-recertifies-through-the-narrow-lane` |
| `crates/topo/src/euler.rs` (`set_edge_curve`) | plain `certify` — unchanged |
| `crates/topo/src/seqgen.rs` (`split_site`) | plain `recertify`, DELIBERATELY. The comment says what the row claims it says, and is confirmed verbatim. Not changed. |

**The fix this item prescribed does not compile.** The item says
`transform_rigid` "would take a `T: Decide + geom_core::CertifiedBounds`
bound". It cannot: `transform_rigid` has a GENERIC caller —
`boolean::ops::apply_recuts`, under `boolean_op_recut` and
`boolean_op_with` — and `boolean_op_with` is run by `verbs::Verb`'s
`impl<T: Decide + Bounds + geom_brep::PcurveFittedLane>` block, which
the dual corpus instantiates at `Dual64`. No `Dual` implements
`geom_core::CertifiedEnclosure`, so the bound raise propagates into a
block that must keep admitting one. `crates/geom-core/src/real.rs`
already records exactly this hazard for that same `verbs` block, and
states the ratified discriminator it comes from: tighten a door's bound
only when NOTHING GENERIC CALLS IT.

**What landed instead** is the shape `validate.rs` already uses for the
same split — the lane as an ARGUMENT, per `NurbsLane`'s own doctrine
that "a caller that can derive the certificate hands one in":

- `geom_brep::EdgeCurve::certify_via` — the mint-side twin of the
  existing `recertify_via`. `certify` and `certify_nurbs_lane` are now
  that one function with the argument filled in, as `recertify` and
  `recertify_nurbs_lane` already were.
- `topo::transform_rigid_via` — `transform_rigid` with the lane taken
  as an argument. `transform_rigid`'s own signature and behaviour are
  UNCHANGED, so no caller moves and no bound propagates.

The item's load-bearing sentence — that this "adds no certification
capability the at-rest validator does not already have" — holds and was
checked rather than repeated: the injected lane is
`geom_brep::plane_nurbs_limbs`, the same function `validate.rs` injects,
and the checks and their order are `run_checks`' own either way.

## Residue

- `graft-recertifies-through-the-narrow-lane` — `combine.rs`, the same
  shape at the graft door, reachability NOT established.
- **No `transform_rigid_certified` convenience door was added.** One
  would carry `Decide + PcurveFittedLane + CertifiedBounds`, a compound
  bracket bound in a file `scripts/gates/bounds-allowlist.sh` does not
  allowlist, so it needs a ratification in `crates/geom-core/src/real.rs`'s
  `bounds_allowlist` module — text that binds future work, and Ev's
  call rather than a lane's. Deliberately left for that conversation;
  `transform_rigid_via` reaches the same behaviour today.

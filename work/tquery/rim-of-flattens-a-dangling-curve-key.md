---
id: rim-of-flattens-a-dangling-curve-key
kind: issue
title: rim_of answers NotAnArc{kind:None} for a dangling curve key, the arm whose doc says null scaffold
status: open
opened: 2026-09-14
refs: [edge-carrier-kind-has-no-readback-door, 2587]
priority: P3
cost: E
---



## What

`topo::query::rim_of` refuses `RimError::NotAnArc { edge, kind: None }`
in two different situations, and the arm's own doc describes only one
of them:

> `kind` is the carrier's kind, or `None` where the edge carries no
> certified carrier at all (a null scaffold) — the two are different
> facts and the payload says which.

The second situation is a LIVE edge whose curve key the arena does not
hold — a dangling geometry reference inside the body, which
`readback::edge_carrier_ref` names `CarrierAbsence::Dangling(Geometry(Curve))`
and both edge read-back doors refuse as
`ReadbackError::Dangling { what: Geometry(Curve(..)) }`. `rim_of` has
one word for it and for the null scaffold, so a caller reading
`kind: None` cannot tell "this edge is scaffolding" from "this body's
geometry reference is dangling".

## Why PR 2587 left it

`edge-carrier-kind-has-no-readback-door` routed `rim_of` through the
crate's one walk and kept the mapping bit-identical, which is what its
rows pin. Telling the two apart needs a `RimError` arm that can NAME a
curve key, and the intactness arm cannot: `RimError::NotIntact` carries
an `EntityId`, and a curve key is not one. So the fix is one of

- widen `NotIntact` to carry a `DanglingRef` rather than an
  `EntityId` — the reference vocabulary `readback` already uses, and
  the one `EulerOpError` maps across to — and route the dangling
  curve key there; its consumers (`sweep::blend`'s `not_intact` shape)
  move with it; or
- give `NotAnArc`'s `kind` payload a third state, which is the
  narrower change and the worse name.

Either changes what a `RimError` arm means, which is a decision about
this door's closed enum (D4 ¶3) and not a re-spelling — so it is a row,
not a line in that PR.

## Reaching it

The state is plantable from inside the crate with `get_edge_mut`, as
`readback.rs`'s
`a_live_edge_with_a_torn_curve_key_refuses_dangling_geometry_on_both_doors`
does; no public door can produce it. A unit closing this row owes a
`rim_of` row beside that one.

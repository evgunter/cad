---
id: tangent-perp-takes-an-unchecked-unit-direction
kind: issue
title: geom-brep tangent.rs perp takes a unit direction it does not check — carrier-derived, so no caller holds the witness yet
status: parked
opened: 2026-09-15
priority: P1
cost: E
blocked_on: [carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes]
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is SCALAR's `unit-vector-invariants-carried-as-prose` (ruled on PR 2457; `docs/doc-ledger/scalar-leaves-the-tracker.md`)
§RATIFIED). The class: a function whose doc or parameter name asserts
a unit-vector precondition it does not check. `geom_core::UnitVec3<T>`
now exists to carry that fact across a function boundary — minted by
the normalizing constructor (`UnitVec3::new(v, site, band)`: decide the
length under the band, divide), by exact negation, by `sin_cos`, and by
`geom-core`'s `frame.rs` ladders. A function in the class takes the
witness the day its caller holds one; until then the precondition
stays prose, and this row is where that is recorded rather than in a
merged PR body.

The geometry CARRIER fields (`Line.dir`, `Plane.normal`, the conic
axes) stay bare under `geom/src/lib.rs`'s at-rest rule by the ruling;
a parameter that is read straight out of such a field is in the class
but its caller holds no witness, so the take waits on either a
decision at the read (a `UnitVec3::new` under a name the reader owns)
or the carrier rule changing, which is not this row's call.

## The site

`crates/geom-brep/src/tangent.rs`, `perp` (`:256`): "the component of
`x` orthogonal to the unit direction `a`", `x − a·(x·a)`, which is a
projection only for unit `a`. Callers pass a surface's stored axis.

## Home

`work/issues/` because `crates/geom-brep/src/tangent.rs` is in no
open program's `paths` (`scripts/work.py territory` names no owner);
it goes to whichever program takes the geom-brep seam.

Moved from `work/issues/` to `work/scalar/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Re-measured and parked (2026-11-19, SCALAR orchestrator)

Read against main `00a5903654`. `perp` is unchanged
(`x - a * x.dot(a)`), and its only callers are inside
`circle_span_bounds`, each passing an axis destructured from a stored
`Surface::Cylinder`, `Torus` or `Cone`. No caller holds a witness, and
a cone's axis is one tier-3 check 1 does not certify.

The row waits "on either a decision at the read … or the carrier rule
changing", and the carrier rule changed: Fork 274 (PR 7006, decided
2026-11-16), built by ONEHOME's
`carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`
(P1 H). When `Surface::*.axis` is a `UnitVec3`, `perp` takes it and
this row is one signature. Minting inside `circle_span_bounds` now
would hand that function a band and a site name it has no other use
for. Parked on that row.

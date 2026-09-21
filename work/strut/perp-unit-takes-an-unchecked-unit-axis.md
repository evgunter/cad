---
id: perp-unit-takes-an-unchecked-unit-axis
kind: issue
title: blend/arms.rs perp_unit takes a unit axis it does not check — the Meridian's stored axis, so no caller holds the witness yet
status: open
opened: 2026-09-15
priority: P3
cost: E
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
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

`crates/sweep/src/blend/arms.rs`, `perp_unit` (`:293`): "the unit
component of `x` orthogonal to the unit direction `a`". Its callers
pass `Meridian::axis` — a stored support axis — so no caller holds the
witness. `Meridian::radial` and `sheet_normal` (`:718`, `:724`) are
postconditions ("the sheet's radial unit", "the unit normal of the
sheet") and not in the class, though the same grep matched them.

## A second site in the same crate (FRAME-WITNESS review, 2026-09-15)

`crates/sweep/src/blend/build.rs`, the corner candidate closure (`:322`):

```rust
let (u_ref, axis) = if convex {
    (n_a, n_a.cross(n_b).normalize())
} else {
    (-n_b, n_b.cross(n_a).normalize())
};
```

The PAIR is orthonormal by construction — `axis` is a cross product
with `n_a`, so `u_ref ⊥ axis` holds without anyone asserting it — and
what is undecided is the cross product's LENGTH: two parallel supports
give a zero-length `axis`, and `Vec3::normalize` divides by that zero
silently. So this is the same class as `perp_unit` one step on: a
frame's two legs assembled with no length decided, where the pair's
perpendicularity is free and the magnitude is not.

It is reachable: the two supports of a corner link are distinct faces
but nothing here refuses them being coplanar-parallel. Taking it wants
`UnitVec3::new` under a name BLEND owns, or
`OrthoFrame::from_axis_and_reference` if the triple is what the caller
actually wants — the second is now one call
(`crates/geom-core/src/linalg/ortho_frame.rs`).

Found by FRAME-WITNESS's review sweep, whose pattern (a doc-word match
plus a `Vec3` parameter) could not see it: the premise is in the
arithmetic, not in prose.

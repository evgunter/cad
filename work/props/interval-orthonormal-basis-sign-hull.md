---
id: interval-orthonormal-basis-sign-hull
kind: issue
title: Vec3::orthonormal_basis returns a sign-hulled frame at Interval when n.z encloses zero
status: open
opened: 2026-09-03
refs: [1191]
---

## What was measured

`crates/geom-core/src/linalg/vec.rs:405` (`Vec3::orthonormal_basis`)
opens with

```rust
let s = T::one().copysign(self.z);
```

At `T = Interval`, `Real::copysign`'s zero-containing arm applies
whenever `self.z` encloses zero: it returns the two-sided hull
`[-|x|, |x|]` with the decoration capped at `Def`
(`crates/geom-core/src/interval.rs`, `copysign`'s own docs, which state
this convention deliberately). So `s = [-1, 1]`, and both returned
basis vectors carry `s` as a factor.

Every planar face whose normal has `n.z == 0` therefore stores a
sign-hulled `u_ref`. That is EVERY vertical wall of an extruded prism:
`newell_plane` (`crates/geom-brep/src/newell.rs:157`) takes the frame
straight from `orthonormal_basis`, and an extrude's side planes are
built through it.

Observed on a literal 12-gon prism replayed at `Interval` over an
ε-scaled parameter box (M10-5's fixtures): one wall's stored frame came
back as

```
u_ref: Vec3 { x: [-6.25e-11, 6.25e-11], y: [-0.0, -0.0],
              z: [-1.0000000000312517, 1.0000000000312517] (Def) }
```

for a face whose true chart direction is exactly `+z`.

## Why it matters

The value channel is unaffected — the plane's LOCUS is still enclosed —
so nothing that only evaluates points is wrong. What breaks is any
consumer that REFINES the chart: `Surface::eval` over a `(u, v)`
sub-rectangle of such a plane returns the whole face's box however far
the rectangle is narrowed, because the frame vector itself spans both
signs. A subdivision over the chart cannot converge at all; it burns
its budget and refuses.

M10-5's clearance engine met this directly. It works around it by
re-charting planar carriers at its own door
(`editor_core::clearance::in_plane_axis` / `chart_frame`: the normal
crossed with a world axis chosen by widest cross product, normalized —
no `copysign` anywhere on that path), and by refusing typed when a
chart provably does not refine. The workaround is local to that engine;
every other chart consumer still reads the stored frame.

## What a fix would look like

The basis is Duff's branchless construction, and the branch it avoids
is exactly the one interval arithmetic cannot take. Two shapes are
plausible, and neither is this unit's to choose:

- pick the crossing axis from the normal's own components (largest
  |component| under a total order) and normalize the cross product,
  which needs no sign transfer at all;
- keep Duff's form but supply `s` from a decided sign rather than
  `copysign`, refusing typed when the normal's z-component is not
  sign-definite.

The first changes stored frames on existing documents and therefore
moves content keys; the second turns a total function into a partial
one. Both are geom-core decisions with a wide blast radius.

## Home

`crates/geom-core/src/linalg/vec.rs` (`orthonormal_basis`), consumed by
`crates/geom-brep/src/newell.rs`. Related to issue 1191 only by
symptom: that one is about enclosure WIDTH growing with the parameter
box, this one is about a frame that is degenerate at any width.

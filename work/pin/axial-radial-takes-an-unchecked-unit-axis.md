---
id: axial-radial-takes-an-unchecked-unit-axis
kind: issue
title: implicit.rs axial_radial takes a unit axis it does not check — carrier-derived, so no caller holds the witness yet
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

`crates/geom-brep/src/implicit.rs`, `axial_radial` (`:101`): "the
axial/radial decomposition `(h, w)` of `p` relative to an anchor point
and unit axis". Every caller passes a `Surface` variant's stored
`axis`, so the precondition is the carrier's at-rest promise read
through a function parameter. `implicit_gradient` (`:174`) in the
same file is NOT in the class — its "unit-magnitude" is a
postcondition — and is listed here only because the sweep's pattern
matched it.


## Added at the fix pass (re-sweep at the merged base)

Two more members in the same crate, filed here so the row names the
whole family:

- `crates/geom-brep/src/ssi/jet.rs`, `axial_radial` (`:318`): "the
  axial/radial decomposition of a path against an anchor and unit axis
  — `implicit`'s `axial_radial`, lifted to series". A self-declared
  re-derivation of the site above, with the same premise on the same
  carrier `axis`; whatever this row does to `implicit::axial_radial`'s
  parameter, the series lift does the same day.
- `crates/geom-brep/src/implicit.rs`, `circle_residual_extremes`
  (`:559`) and `circle_residual_curvature_bound` (`:593`): "the frame
  precondition of `circle_arc_residual_range` binds here too: `axis`
  and `u_ref` unit and mutually orthogonal, unchecked". A FRAME
  premise — two unit vectors, orthogonal — which is the second unit's
  shape (the frame witness the ruling's refinement names) rather than
  the single-direction witness; recorded here because the parameters
  are the same carrier `axis` and `u_ref`.

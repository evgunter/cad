---
id: authored-and-derived-directions-decide-a-unitless-norm-against-the-length-band
kind: issue
title: Authored and derived directions decide a unit-less norm against the length band through the metre door
status: open
opened: 2026-10-02
---


## What

`Margin::norm3`'s dimensional argument is "the norm of a metre-component
vector". `UnitVec3::new` (`crates/geom-core/src/linalg/unit_vec.rs`)
decides every caller's direction length through it. For a carrier's
unit-at-rest normal or axis that norm is a pure number, 1 at every model
scale. Read against the length band, it decides differently at a
millimetre and at a metre. `topo`'s `rim_dim_boolean_twins` caught
one such site (`bool_germ_plane_normal`). The REACH branch
`reach/dev-probe-red` adds `UnitVec3::levered(v, site, band, arm)` and
moves the three carrier-field reads onto it: `bool_germ_plane_normal`
(`topo/src/boolean/join.rs`), `bool_box_cylinder_axis`
(`topo/src/boolean/boxes.rs`, `face_box_rule`) and `props_torus_axis`
(`geom-brep/src/props/curved.rs`, `torus_meridian_orient`).

The rest of the sweep is a different case: a vector whose magnitude
is the AUTHOR's or a derived witness's, not a carrier field. Nobody
has argued what its norm is measured in. `topo/src/query.rs`'s
`DATUM_UNIT_NORM` docs claim "a genuine length", but a datum normal
authored as `(0, 0, 1e6)` is accepted, and nothing says that is a
megametre.

## The hit list (pattern `UnitVec3::new(` / `decide_unit_direction(`, then `Margin::norm3(` / `Margin::norm2(` on a direction)

- `topo/src/query.rs` `DATUM_UNIT_NORM`, decided at
  `editor-core/src/eval/wire.rs` (the datum boundary) and
  `editor-core/src/verbs/split.rs`: an authored datum normal or axis.
- `editor-core/src/eval/wire.rs` `EVAL_DIRECTION_NORM`: an authored
  transform axis or pattern direction.
- `editor-core/src/mate/solve.rs` `derived_direction`: a rotated unit
  witness, re-minted under its own name. The doc says its length is 1
  within rounding, so this is the carrier case: a pure number.
- `geom-core/src/linalg/frame.rs` `frame_point_at_aim`,
  `frame_path_start_tangent`, `frame_mirror_normal`, and
  `ortho_frame.rs`'s mints: these inherit whatever dimension the
  caller's vector carries.
- `sweep/src/revolve/axis.rs` `revolve_axis_direction`
  (`Margin::norm2(axis.dir)`): an authored revolve axis.
- `demos/tour/src/scalar.rs` `TOUR_FRAME_AXIS` and
  `topo/src/test_support_fixtures.rs` `FIXTURE_SPLIT_NORMAL`: literal
  unit normals in a demo and in fixtures.
- Not in the class: `editor-core/src/mate/coset.rs`
  `mate_axes_parallel`, which already levers `u × v` by an `Arm`; and
  `topo/src/boolean/boxes.rs`'s two test mints of fixture axes.

**Blind spot.** A direction decided by a bare `decide(.., Margin::of(..))`
of its norm matches neither pattern. Only the `norm3`/`norm2` shape was
swept.

## Owed

Decide, per site, whether the vector's norm has a unit. Where it has
none, lever it by the reach it is consumed over (`UnitVec3::levered`),
as the carrier reads now do. Where it is a length, state the argument
in place of the claim. A linearity twin that drives each site at two
scales is the row that can fail.

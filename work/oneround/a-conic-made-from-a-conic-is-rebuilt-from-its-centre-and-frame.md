---
id: a-conic-made-from-a-conic-is-rebuilt-from-its-centre-and-frame
kind: issue
title: A conic made from a conic is rebuilt from its centre and frame, and loses its placement
status: open
opened: 2026-11-23
priority: P1
cost: M
design: true
parent: one-stored-form-waits-on-the-readers-of-a-conics-centre
refs: [7510, 7258]
---


Cut from `one-stored-form-waits-on-the-readers-of-a-conics-centre` on 2026-11-23, when its census came in at 323 production reads against the row's 90 (PR 7510, the first part: directions, cover, extent and the gate). The functions below are pinned by that PR's gate (`clippy.toml` `disallowed-methods`, an `#[expect]` on each function, `scripts/gates/conic-centre-reads.counts`) as of `bd07a867e7`. **The sorting is the implementer's, from the census line of each read and not from reading each function through**, and a `°` marks a function that reads only `Circle::frame`. **What these readers should ask the conic is a design question in front of three designers** (`analysis/design-fork/conic-reader-interface`); two of the three already say this family is not one family, so the list is where to look, not the unit's shape.

## The functions

- topo `replace_face.rs`: `transport_curve`°, `homothety`°, `cone_v_range`
- topo `offset_axial.rs`: `mint_carrier`, `param_on`°, `reauthor`°
- topo `ring_path.rs`: `torus_circle_arc`, `distance_floor` (`ConicArc`, from an ellipse's derived frame)
- topo `rim_wedge.rs`: `rides`°, `face_boundary_circles`°, `curve_sense`°, `crossings`°; `boolean/ops.rs`: `sphere_pair_cut`°, `pair_seam`° (the `Rim` round trip)
- topo `validate.rs`: `meet_segment`° (`MeetSegment::Arc`)
- topo `injective.rs`: `conic`; `stands.rs`: `line_meetings`; `solid_contain.rs`: `sphere_chart_trim`°; `torus_region.rs`: `on_edge_plane`°; `surface_group.rs`: `wrap_rims_within`° (each restates `(origin, axis, u_ref)` into a record of its own)
- geom-brep `curve_pair/on_surface.rs`: `starting_at`°, `plane`
- geom `pole_form.rs`: `half_turn_curve`°
- sweep `swept.rs`: `register_placed_carrier_end`° (`circle_at`; also the symbolic scalar's one site)
- sweep `blend/junction.rs`: `rim_trim_circles`°, `rim_carrier`°; `blend/open/end_face.rs`: `of`; `blend/surgery.rs`: `piece_along`
- step-import `build.rs`: `column_circle`°, `circle_column_match`°; `on_built.rs`: `of` (`Conic::of`), `arc_on_circle`°, `printed_on`°

## What a placement-keeping map has to do, as the implementer understands it

- **What exists:** `Circle::placed(map)` maps the one stored frame; `Ellipse::mapped(map)` maps the stored frame and keeps the placement and the phase, right only for an isometry that keeps the parameter; `Ellipse::reversed()` flips the axis, keeps the seam and negates the phase's sine so the anchor stays where it is; `Ellipse::translated`, `centred_at`; `quarter_turned_where` and `seamed_toward` re-seam under the anchor.
- **A transported frame** (`transport_curve`, `mint_carrier`, `reauthor`, `torus_circle_arc`, the `Rim` round trip): the motion is applied to the stored placement, anchor included, never by reading `frame()` and calling a constructor on the pieces. The gap is the doors that hold the conic only as a `(centre, axis, u_ref, radius)` record of their own (`Rim`, `MeetSegment::Arc`, `ConicArc`): the record drops the placement, so either it carries the conic itself or the round trip says its result is another conic.
- **A reversed axis** (`column_circle`, `half_turn_curve`, `Curve3::reversed`'s callers): for an ellipse it is `Ellipse::reversed()`, not `mapped(Frame::reversed)`, which is a half turn about the line through the stored point along the seam and so another ellipse when that point is the anchor. For a circle `placed(Frame::reversed)` is right only because a circle's stored point is its centre. `column_circle` and `half_turn_curve` also re-square the frame through `Curve3::circle`/`Frame::new`, a second change of bits beside the placement.
- **A homothety** (`replace_face::homothety`) **and an offset** (`offset_axial.rs`) change the sizes; no existing door does that keeping the placement. Whether an offset of an ellipse is a conic at all is `mint_carrier`'s own question.
- **Not known:** whether any of these doors is handed an arc-held ellipse today. `mint_carrier`, `torus_circle_arc`, `distance_floor`, `piece_along`, `end_face::of`, `Conic::of` and `cone_v_range` take a conic or an ellipse and are the ones to check first.

---
id: point-to-carrier-distance-is-spelled-seven-times
kind: issue
title: A point's distance to a carrier is spelled seven times over different kind subsets, and the on-carrier decision reads it exact in one door and linearized in another
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

A point's distance to a carrier (and the on-carrier decision built on it) is spelled in:

- `geom_brep::implicit_residual` (`crates/geom-brep/src/implicit.rs:165`): exhaustive, but Nurbs and Approx return poison rather than refusing typed;
- `carrier_eq::distance_to` (`crates/topo/src/boolean/carrier_eq.rs:1027`): plane, sphere, cylinder and torus;
- `carrier_touch`'s own distance;
- `pcurve_cache::chart_distance` (`crates/geom-brep/src/pcurve_cache.rs:8207`): `unreachable!` for Nurbs and Approx;
- `blend/reach::surface_distance` (`crates/sweep/src/blend/reach.rs:490`);
- `offset_axial::surface_residual` (`crates/topo/src/offset_axial.rs:2556`): `_ => 0`, filed separately on SHELL;
- `solid_contain::torus_elevation` (`crates/topo/src/boolean/solid_contain.rs:2673`).

The on-carrier decision is not one reading either:
- `contain` decides on the exact ρ−r;
- `solid_contain` decides on the linearized (ρ²−r²)/2r;
- `vtxfac` and `reduce` decide on `implicit_residual`, and the placement then decides again on the exact form, under the same predicate name `bool_curved_contain_carrier`.

## Owed

Re-aimed to D3 "One reading per question". A point's residual, distance and on-carrier decision are a reading of one carrier against a point. One such reading, with one arm per surface kind, which every spelling above reads.

- `implicit_residual`'s Nurbs and Approx poison is a total evaluator's poison, which D3 says is not a decision: no reading reads it without first refusing on kind. The reading refuses those kinds typed, naming what it lacks.
- One decision form per predicate name: the exact ρ−r and the linearized (ρ²−r²)/2r are two readings of one fact, which D3 forbids. `bool_curved_contain_carrier` decides once.
- D3 places one-carrier-against-a-point readings in `geom`, the lowest crate that can state them.

The test oracles keep their own copies by design (D3 exempts them).

## Curves (2026-10-15, the curve-pair reader's split on PR 5154)

A point against one CURVE carrier now has its readings in `geom`: the point's foot (`Curve3::foot`, `crates/geom/src/curves/on_carrier.rs`) and whether it lies on the carrier (`Curve3::on_carrier`, on by the gap at the foot and off only by a lower bound on the distance from the whole carrier). `geom_brep::curve_pair::Span::contact` composes them with the span's reading of its ends. Moved onto them in that PR:
- `pose::foot`, `line_foot` and `conic_foot` (`crates/geom-brep/src/curve_pair/pose.rs`) now delegate to `Curve3::foot`;
- `walk::polish_foot` and `polish::foot` are gone;
- `ConicArc::hit`'s ellipse Newton foot and Hessian bound (`crates/topo/src/splitting/containment.rs`) are now `Curve3::on_carrier` at the foot;
- `curve_pair_span_gap` is now `curve_carrier_gap`;
- `carrier_cross`'s boundary-vertex `param_near` is now the foot.

The sweep's residue (§5, "a spelling of the foot or of the carrier gap outside those readings"):
- **`circle_miss`** (`crates/topo/src/splitting/containment.rs:1051`), a point's exact distance from a circle, read by:
  - `reduce.rs:3042` (`bool_arc_chain_on_circle`);
  - `reduce.rs:3112`;
  - `contain.rs:1341` (`bool_contact_arc`, `point_on_circle`).
  
  These are on-carrier decisions that do not read `Curve3::on_carrier`. The exact distance is its clearance on a circle.
- **The census line lane's gap** (`crates/topo/src/census.rs` `on_edge_interior_reading`, `pm_census_ve_line_gap`) and its foot `(q − p0)·dir`: a line's on-carrier reading and foot spelled in place. The FUSE stage-(c3) PR moves it onto `Span::contact`.
- **The census curved lane's foot search** (`crates/topo/src/census/curved.rs` `feet`/`polish_foot`/`near_pieces`): a second foot search beside `Curve3::foot`. The (c2)/(c3) merge moves it.
- **`SpiricArc::contact`** (`crates/topo/src/splitting/spiric_arc.rs:226`): the containment walk's spiric boundary reading, with its own gap rows (`SpiricRows`), not `Curve3::on_carrier`.
- **`param_near` as a parameter of a point already on the carrier**:
  - `reduce.rs:4264` (the split parameter);
  - `ring_path.rs:712`;
  - `chord_join.rs:1252`;
  - `offset_axial.rs:2416` and `2445`;
  - `shell.rs:3169`, `3206` and `3262`;
  - `replace_face.rs:3566`, `4255` and `4333`;
  - `blend/surgery.rs:3644` and `5339`;
  - `blend/open/end_face.rs:884`;
  - `offset_derive.rs:456` (`running_with`: the candidate's tangent at the old middle's parameter);
  - `edge_join.rs:967`;
  - `shell.rs:3219`;
  - editor-core `names/emit_topo.rs:3246`, `3381`, `3880` and `6835`;
  - `curve_pair/walk.rs:194`, which is a seed: each parameter it yields is read at `Curve3::foot`.
- **Fixed in the PR:** `offset_derive.rs` `nearest` read `param_near` as a foot to choose the section branch nearest the old middle. On an ellipse that is the defect class, so it now reads the gap at `Curve3::foot`.

  None of these decides on-carrier at that parameter here. Each is a foot only where the point is on the carrier, which is not where the unit-coordinate angle fails. On an ellipse they read the unit-coordinate angle, which is not the foot of a point off the curve: each site owes a check that its point is on the carrier.

## Merged from `analytic-exact-elevations-have-inline-homes` (2026-10-26, tracker sweep)

That row, closed into this one, asked for the EXACT signed distance (not `implicit_residual`'s linearized `(q² − r²)/2r`) of the cylinder, sphere and torus to get one home each beside `geom_brep::cone_elevation` (`crates/geom-brep/src/implicit.rs:136`, PR 3322, which `implicit_residual`'s cone arm, `offset_axial::surface_residual`, the solid door and the face door all call). Sites it listed that this row does not:
- cylinder `ρ − r`: `contain.rs` `curved_face_placement`'s carrier test, `validate.rs`'s `radial` read, `geom-brep/src/props/curved.rs`;
- sphere `|p − c| − r`: `contain.rs` `sphere_face_containment`.

When the reading lands, check bit-identity per site: the torus copies (`solid_contain::torus_elevation` against `offset_axial::surface_residual`, spelled through `Vec3::norm`) differ in operation order.

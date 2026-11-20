---
id: lever-doors-pinned-sites-await-a-geometry-read
kind: issue
title: 59 production references still hand a named lever door a number or a point; lever-doors.sh pins them, and wrappers and hand-built geometry pass it
status: open
opened: 2026-11-19
priority: P1
cost: H
---

`scripts/gates/lever-doors.sh` is a ratchet on the named `Lever`/`ExtentBall`/`Reach` doors that take a number or a point the caller computed. Each is referenced only in its home, and elsewhere at a pin per door and file (`scripts/gates/lever-doors.counts`) that only falls. It does not make `crates/geom-core/README.md`'s "never from a length a caller vouches for" true: the 59 pinned references below still do it, and the wrappers and hand-built geometry at the end of this row do it with no pin.

`LINES_FOR=ALL bash scripts/gates/lever-doors.sh` lists the references with line numbers. Sites are named by function here.

## The 59 pinned references, by what the site has in hand

Totals: in the function 22, one frame up 10, none near 23, not a site 4. "zero" marks a reference whose value is always 0 (`ExtentBall::point(p).lever()`: a fold's seed or a point's own reach); there are 8.

Each conversion moves a reading and wants its own differential.

### In the function (22): the consumed carrier, edge, face or cover is in hand

- `sweep::blend::battery::extent_of` — `points_diameter`. Holds `carrier: &Curve3` and `(t0, t1)`, exactly `edge_extent_inf`'s arguments.
- `step_import::reach::Reach::lever` — `points_diameter`. `Reach` is `Reach(Cover<f64>)`; it builds two box corners from `self.along(..)`.
- `step_import::build::faces_reach` (2: the empty-box return and the corner pair) — `points_diameter`. Holds `solid: &SolidSpec` and the two face indices; `step_import::reach::Reach::of(solid, faces)` builds a `Cover` from the same pair.
- `topo::boolean::fate::leaving` — `ExtentBall::point`. Holds `body`, the half-edge, its edge and the certified `curve`; the `Line` arm reads the two end vertices.
- `topo::boolean::fate::piece_beside` — `ExtentBall::point`. Holds `body`, the face and the half-edge of a line edge; reads an end vertex from `p`.
- `topo::boolean::overlap::half_plane` — `ExtentBall::point`. Holds `body`, the half-edge and `carrier`; the `Line` arm reads `carrier.eval(tb)` from `p`.
- `topo::offset_axial::corner_arms` — `ExtentBall::point`. Holds `curve.carrier()` and `(t0, t1)`; the fallback arm reads `other.eval(t1)` from `other.eval(t0)`. Its `Line` and `Circle` arms are `Lever::unsourced`.
- `topo::offset_axial::axial_frame` (2, one zero) — `ExtentBall::point`. Holds `body` and the `scope` of faces; folds the scope's vertices from `origin`.
- `topo::offset_together::corner_arms` — `ExtentBall::point`. Holds `body`, the vertex and each outgoing half-edge; reads the far vertex.
- `topo::pcurves::decision::edge_reach` — `points_lever_from`. Holds `carrier` and `(t0, t1)`, plus points its caller adds.
- `topo::wedge_reach::WedgeReach::reach` — `points_lever_from`. Holds `carrier`, `(t0, t1)` and the two face keys; reads the span ends and the faces' stored points.
- `topo::ring_path::spline_wall::IsoPiece::travel` — `ExtentBall::point`. Holds `self.curve` (a `NurbsCurve3`) and `self.span`; reads the chord of its ends.
- `topo::ring_path::spline_crossings` — `ExtentBall::point`. Holds the `Piece::Segment`/`Piece::Ruling` ends and builds `Curve3::line(from, to - from)` on the next line.
- `topo::solid_contain::selection_reach` (zero) — `ExtentBall::point`. The fold's seed; every term is `cover::face_reach_from(body, face, q, band)`.
- `topo::splitting::containment::extent_from` (2, one zero) — `ExtentBall::point`. Holds `covers: &[Cover<T>]` and reads them in the same fold, beside `verts`.
- `topo::cover::cone_face_reach` — `Reach::of_face`. Holds `body`, `face`, the `cone` surface and the face's `cover`; hands `of_face` the cover's own window and far distance.
- `topo::cover::meet_within` — `of_cover_within`. Holds both faces' covers; the number is `tolerance: f64`.
- `geom_brep::dihedral::seam_face_reading` — `curvature_radius`. Holds the spline `chart` and `uv`; the arm is the `chart_arm` closure's number, an `Option<f64>` its caller computes.
- `geom_brep::pair_section::CylinderSpherePose::sweep_ball` — `of_cover_within`. Holds `self.sphere` and its cover; the number is `pad`, the larger of `self.big_r` and a band multiple.
- `geom_brep::intersect::ruled_frame` — `ruled_lever`. Holds the `Reach` and the `PlaneCylinder` pose; the numbers are the pose's gap, sine and cosine.

### One frame up (10): points or scalars here, the caller holds the geometry

- `topo::splitting::containment::polygon_walk` (2, one zero) — `ExtentBall::point`. Holds the loop's vertex points; its caller holds the loop record with its `verts` and `covers`.
- `topo::splitting::join::crossing_reach` (2, one zero) — `ExtentBall::point`. Holds `crossings`; its two callers hold the face being split.
- `topo::splitting::order::sort_indices_by_point` (2, one zero) — `ExtentBall::point`. Holds points and the `SplitPlane`; its caller holds the reduction and its face.
- `topo::boolean::join::frame_reading` — `ExtentBall::point`. Holds a surface and two point lists; its caller passes `witnesses(body_a, face_a)` and holds both bodies and faces.
- `topo::separating::planar_axis` — `ExtentBall::box_diagonal`. Holds `body`, `face` and a `reach` closure that returns `(lo, hi)`; two callers pass `census::face_reach(body, face, band)`, two pass a box of their own.
- `topo::bound_side` `descend` — `curvature_radius`. Holds `FaceDatum.lever: Option<T>` and `FaceBend::Implicit`'s radius; `FaceBend::at(surface, sense, p)` reads the surface one frame up.
- `geom_brep::props::curved::cone_boundary` — `curvature_radius`. The rim's `Circle` is destructured in the loop that fills `rims`, but `Rim` does not keep it, so `cone_arm` re-derives the radius as `|v|·sin α` (or `1` with no rim). Its caller holds the cone.

### None near (23): no carrier, edge, face or cover within a frame

- `geom_brep::carrier_eq::at_consumed_extent` and `reading` (3) — `lever_from_within`. Hold two `CarrierDesc` and the `ExtentBall`; the number is the apexes' distance or an axis offset.
- `geom_brep::intersect::pair_bound` — `generator_lever`. Holds the `Reach` and the cone's apex, axis and half-angle cosine as scalars. Its callers were not read.
- `geom_brep::pair_section::cylinders::decide_stance` — `of_crossing_cylinders`. Holds two axis `Line`s, two radii and the pose's sine and gap.
- `geom_brep::pair_section::cylinders::region_span`, `bisector_nodes`, `cylinder_cylinder` (3) — `ExtentBall::point`. Each places a computed point (an axis foot, a node, a touch point) against the region; none levers it.
- `geom_brep::ssi::point_arm` — `within_curvature`. Holds `kappa: T` and an extent `Lever`.
- `sweep::blend::battery::corner_independence` — `curvature_radius`. Holds three normals and `radius: T`, the blend radius.
- `topo::boolean::join::bool_connect` (2) — `ExtentBall::point`. One is a site point mirrored through a loop's axis origin; one maps a germ site's vertex point.
- `topo::boolean::join::surface_pair_reach` (2) — `ExtentBall::point` over `face_witnesses` to find a centre, then `Reach::of_face` of the maxima of several faces' `Reach::Face` fields. Each face's reach is read off its cover; the fold across faces has no cover of its own.
- `topo::boolean::section_cert::cone_plane` — `lever_from_point_and_plane`. Holds a `Cone` pose, the plane's `p0` and `n`, and the `Reach`.
- `topo::bound_side::BoundSupport::lever` — `ExtentBall::point`. `BoundSupport::Chord { base, far }` stores two points.
- `topo::chord_join::wall_face_section` — `Reach::of_face`. Holds the wall `Surface` and a `WallReach` of scalars.
- `topo::link::Link::of` (2, one zero) — `ExtentBall::point`. Holds `p` and the sectors' wedge points.
- `topo::link::Link::of_tangent` (zero) — `ExtentBall::point`. A point's own reach.
- `topo::link::second_order::reach` — `ExtentBall::point`. `point(apex + d).lever_from(apex)` is `|d|`.
- `topo::ring_path` `ruled_paths` — `ExtentBall::point`. On a cone, the apex `base` read from a point `on` of the ruling; holds the wall as the local `Quadric`, not the surface.

### Not a site (4)

- `geom_brep::lib` — the re-export of `points_diameter` and `points_lever_from` (2).
- `topo::boolean::join::surface_pair_reach` — `Reach::Face { .. }` as a destructuring pattern.
- `topo::test_support_fixtures::describe_as_intersections` — test support, mounted without a `cfg(test)`.

## Doors the pins do not see

**Wrappers.** Each takes a number or a point from its caller and hands it to a pinned site in its own file, so a caller in an unpinned file reaches the door. Caller counts are production call sites.

- `sweep::blend::battery::corner_independence(vertex, normals, radius, band)` — `pub`. Takes `radius: T`. 4 callers, all in `battery.rs`.
- `geom_brep::dihedral::classify_seam(.., chart_arm, ..)` — `pub`. Takes a closure `Fn(&NurbsSurface, Point3, T) -> Option<f64>` whose number becomes a curvature arm in `seam_face_reading`. 1 caller (`topo::pcurves::mint`).
- `geom_brep::ssi::point_arm(kappa, extent)` — `pub(crate)`. Takes `kappa: T`. 5 callers (`ssi::march` 1, `dihedral_side` 2, `edge_nurbs` 2); the `march` one also passes `Lever::unsourced(extent)`.
- `topo::bound_side::FaceDatum::of_face(normal, bend, lever)` — `pub(crate)`. Takes `lever: T`. 4 callers (`boolean::sectors` 3, `boolean::vtxfac` 1).
- `topo::bound_side::BoundSupport::Chord { base, far }` — a `pub(crate)` variant with public fields. 23 constructions or matches: `boolean::offer_rows` 8, `bound_side` 4, `boolean::sectors` 2, and one each in `boolean::fate`, `boolean::recl`, `splitting::neighborhood`, `star::walk`. The widest.
- `topo::cover::meet_within(a, b, tolerance)` — `pub(crate)`. Takes `tolerance: f64`. 2 callers (`boolean::ops`, `boolean::glue`).
- `topo::separating::planar_axis(body, face, reach, band)` — `pub(crate)`. Takes a closure returning `(lo, hi)`. 4 callers.
- `topo::chord_join::wall_face_section(plane, wall, reach, band)` — `pub(crate)`. Takes a `WallReach` of `at`, `below`, `above`, `across`. 2 callers.
- `topo::link::Link::of(p, sectors)` and `Link::of_tangent(p, pieces)` — `pub(crate)`. 4 and 2 callers, in `link.rs`.
- `topo::link::second_order::reach(apex, d)` — `pub(super)`. Takes a vector; 2 calls.
- `topo::pcurves::decision::edge_reach(carrier, span, points)` — `pub(super)`. Takes extra points beside the carrier. About 33 references across `pcurves` (a pattern count).
- `topo::wedge_reach::WedgeReach::reach(faces, carrier, span)` — `pub`. About 32 references with the free `wedge_reach` (a pattern count).
- `topo::boolean::join::frame_reading(sa, on_a, on_b)` — `pub(super)`. Takes two point lists. 1 caller.
- `topo::solid_contain::selection_reach(body, faces, q, band)` — `pub(crate)`. Takes the pivot `q`. 4 callers.

**Hand-built geometry.** `Surface`, `Circle`, `Frame` and `Curve3` have public fields, and `geom::cover::Cover` has public constructors (`points`, `segment`, `sphere`, …, referenced in about 24 files outside `geom`'s cover home). A length from any crate reaches every reading constructor through a carrier, a span or a cover the caller made: `circle_curvature_arm(&Circle { radius: len, .. })`, `ExtentBall::of_cover(&Cover::segment(p, q)?)`.

**A caller's point.** `lever_from(p)` reads from any pivot. The curvature home's `curvature_lever_arm(cone, p)` and `parallel_curvature_arm(s, p)` read `p`'s distance from the surface's axis, so a far point is a long arm.

**Other spellings.** A door called through a macro's expansion, and `Self::door(…)` in a trait impl for `geom_brep::Reach` written in another crate.

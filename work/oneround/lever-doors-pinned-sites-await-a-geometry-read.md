---
id: lever-doors-pinned-sites-await-a-geometry-read
kind: issue
title: lever-doors.sh pins 47 production references to the named doors that take a number or a point; wrappers and hand-built geometry pass it
status: open
opened: 2026-11-19
priority: P1
cost: M
---

`scripts/gates/lever-doors.sh` is a ratchet on the named `Lever`/`ExtentBall`/`Reach` doors that take a number or a point the caller computed. Each is referenced only in its home, and elsewhere at a pin per door and file (`scripts/gates/lever-doors.counts`) that only falls. It does not make `crates/geom-core/README.md`'s "never from a length a caller vouches for" true.

The gate counts 47 references (59 when it landed; `oneround/lever-sites-brep` cleared 12). 4 are not sites, and 8 of the rest are always zero (`ExtentBall::point(p).lever()`: a fold's seed or a point's own reach). The other 35 hand a door a number or a point of the site's own.

`LINES_FOR=ALL bash scripts/gates/lever-doors.sh` lists the references with line numbers. Sites are named by function here, and each line says what that function's own signature and body hold. Callers are named only where stated.

Each conversion moves a reading and wants its own differential.

## Geometry in the function (31)

The function holds a carrier, a surface, a cover, a `Reach` or extent ball, a pose of carriers, or a body with the face, edge or vertex.

geom-brep:
- `intersect::pair_bound` — `generator_lever(axis, half_angle)`. The `Reach`, and the cone's apex, squared axis and half-angle as locals of its lane. The door reads the window and takes the cosine itself; the axis line and the angle are still the caller's. The pin comes off when the lane holds a cone pose (a struct it builds once, as `PlaneCylinder`) and the door takes that.
- `pair_section::CylinderSpherePose::sweep_ball` — `of_cover_within`. `self.sphere` and its cover; the number is `pad`, the larger of `self.big_r` and a band multiple.
- `pair_section::cylinders::decide_stance` — `of_crossing_cylinders(lines, radii, &reach)`. The two axis `Line`s, the two radii and the `Reach`. The door reads the common perpendicular itself and hands it back; the lines and radii are still the caller's. What it builds is a region, the two walls' meet, so the pin comes off when that ball is read off a cover (each wall's within the reach, met), not when a pose is passed.

sweep:
- `sweep::blend::battery::extent_of` — `points_diameter`. `carrier: &Curve3` and `(t0, t1)`, which are `edge_extent_inf`'s arguments.

topo:
- `boolean::fate::leaving` — `ExtentBall::point`. `body` and the half-edge; the `Line` arm reads the two end points.
- `boolean::fate::piece_beside` — `ExtentBall::point`. `body`, the `(face, edge)` sides and `p`.
- `boolean::join::bool_connect` (2) — `ExtentBall::point`. The reduction and both bodies. One site mirrors `p` through a loop record's axis origin where the record holds no ball; one maps a half-edge's start point.
- `boolean::join::frame_reading` — `ExtentBall::point`. A `Surface` and two point lists (its caller's `witnesses` of two faces).
- `boolean::join::surface_pair_reach` (2) — `ExtentBall::point` over the face pairs' witnesses, then `Reach::of_face` of the maxima of the faces' `Reach::Face` fields. Both bodies, the face pairs and the cone.
- `boolean::overlap::half_plane` — `ExtentBall::point`. `body`, the half-edge and its carrier; the `Line` arm reads `carrier.eval(tb)` from `p`.
- `boolean::section_cert::cone_plane` — `lever_from_point_and_plane`. The `Cone` pose, the plane's `p0` and `n`, and the `Reach`.
- `chord_join::wall_face_section` — `Reach::of_face`. The wall `Surface` and a `WallReach` of scalars.
- `cover::meet_within` — `of_cover_within`. Both faces' covers; the number is `tolerance: f64`.
- `cover::cone_face_reach` — `Reach::of_face`. `body`, `face`, the cone and the face's cover; the point `at` is its caller's.
- `link::Link::of` (2, one zero) and `Link::of_tangent` (zero) — `ExtentBall::point`. `p` and the sectors or pieces, whose `FaceSector`s hold a `LinkFace` with its `Surface`; `of` folds the sectors' wedge points, `of_tangent` reads a point's own reach.
- `offset_axial::axial_frame` (2, one zero) — `ExtentBall::point`. `body` and the scope; folds the body's vertices from `origin`.
- `offset_axial::corner_arms` — `ExtentBall::point`. `body`, the vertex and each curve's carrier and span; the fallback arm reads `other.eval(t1)` from `other.eval(t0)`. Its `Line` and `Circle` arms are `Lever::unsourced`.
- `offset_together::corner_arms` — `ExtentBall::point`. `body`, the vertex and each outgoing half-edge's far vertex.
- `pcurves::decision::edge_reach` — `points_lever_from`. The carrier and span, plus points its caller hands.
- `ring_path` `Quadric::ruled_paths` — `ExtentBall::point`. The wall as the local `Quadric` and the ruling `Line`; on a cone, the apex read from a point of the ruling.
- `ring_path::spline_crossings` — `ExtentBall::point`. The `SplineArc`'s carrier and span and the piece's two ends.
- `ring_path::spline_wall::IsoPiece::travel` — `ExtentBall::point`. `self`'s curve and span; reads the chord of its ends.
- `separating::planar_axis` — `ExtentBall::box_diagonal`. `body` and `face`; the box is a closure's `(lo, hi)`, its caller's.
- `solid_contain::selection_reach` (zero) — `ExtentBall::point`. `body` and the faces; the fold's seed.
- `splitting::containment::extent_from` (2, one zero) — `ExtentBall::point`. `covers: &[Cover<T>]`, read in the same fold beside `verts`.

## Points or scalars only (12)

- `geom_brep::ssi::point_arm` — `within_curvature`. `kappa: T` and an extent `Lever`.
- `sweep::blend::battery::corner_independence` — `curvature_radius`. Three normals and `radius: T`.
- `topo::bound_side::BoundSupport::lever` — `ExtentBall::point`. `BoundSupport::Chord { base, far }`, two stored points.
- `topo::bound_side` `read_down` — `curvature_radius`. `FaceDatum.lever: Option<T>` and `FaceBend`'s radius, two stored scalars; `FaceBend::at(surface, sense, p)` reads the surface where the bend is built.
- `topo::wedge_reach::reach_from_vertex` — `points_lever_from`. A pivot and points, both the caller's: a pass-through. `WedgeReach::reach` held this site with the carrier, the span and the two face keys until EMBED's `embed/remeter` (PR 7329) moved it here, so that the embedding question reads the same recipe at its lifted scalar (`reach_over`) and two faces' reach from a vertex.
- `topo::link::second_order::reach` — `ExtentBall::point`. `point(apex + d).lever_from(apex)` is `|d|`.
- `topo::splitting::containment::polygon_walk` (2, one zero) — `ExtentBall::point`. The loop's points and `q`.
- `topo::splitting::join::crossing_reach` (2, one zero) — `ExtentBall::point`. The crossings and `from`.
- `topo::splitting::order::sort_indices_by_point` (2, one zero) — `ExtentBall::point`. The points and the `SplitPlane`.

## Not a site (4)

- `geom_brep::lib` — the re-export of `points_diameter` and `points_lever_from` (2).
- `topo::boolean::join::surface_pair_reach` — `Reach::Face { .. }` as a destructuring pattern.
- `topo::test_support_fixtures::describe_as_intersections` — test support, mounted without a `cfg(test)`.

## Doors the pins do not see

**Wrappers.** Each takes a number or a point from its caller and hands it to a pinned site in its own file, so a caller in an unpinned file reaches the door.

Each count is the number of matches of the pattern beside it in the gate's production view, less the definition's own `fn` line. To re-run:

```
. scripts/gates/lib.sh; gate_require_crate_sources; gate_production_sources
gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" > /tmp/prod.txt
grep -vE 'fn NAME\b' /tmp/prod.txt | grep -oE 'PATTERN' | wc -l
```

- `sweep::blend::battery::corner_independence(vertex, normals, radius, band)` — `pub`, takes `radius: T`. 4 (`corner_independence\(`).
- `geom_brep::ssi::point_arm(kappa, extent)` — `pub(crate)`, takes `kappa: T`. 5 (`point_arm\(`).
- `topo::wedge_reach::reach_over(carrier, span, faces)` and `reach_from_vertex(vertex, faces)` — `pub(crate)`, each takes the faces' points, and the second the pivot; `WedgeReach::reach`'s pinned site moved into the second. 2 and 2 (`reach_over\(`, `reach_from_vertex\(`). Added by EMBED's `embed/remeter` (2026-11-22): the embedding question reads an edge's reach by tier 3's recipe at its own scalar, off the lifted carrier and the two faces' lifted vertices, and two faces' reach from a vertex they share alone, which is a new read at a new pivot. The pin did not rise because both go through one call; that is this list's pattern, not a geometry read.
- `topo::bound_side::FaceDatum::of_face(normal, bend, lever)` — `pub(crate)`, takes `lever: T`. 3 (`FaceDatum::of_face\(`). `boolean::sectors::pierce_side(.., lever: T, ..)`, `pub(super)`, hands it on: 2 (`pierce_side\(`).
- `topo::bound_side::BoundSupport::Chord { base, far }` — a `pub(crate)` variant with public fields. 11 (`BoundSupport::Chord`); a `Self::Chord` or a `Chord` reached by import is not in that count. The widest.
- `topo::cover::meet_within(a, b, tolerance)` — `pub(crate)`, takes `tolerance: f64`. 2 (`meet_within\(`).
- `topo::cover::cone_face_reach(body, face, at, ..)` — `pub(crate)`, takes the point `at`. 2 (`cone_face_reach\(`).
- `topo::separating::planar_axis(body, face, reach, band)` — `pub(crate)`, takes a closure returning `(lo, hi)`. 4 (`planar_axis\(`).
- `topo::chord_join::wall_face_section(plane, wall, reach, band)` — `pub(crate)`, takes a `WallReach` of `at`, `below`, `above`, `across`. 2 (`wall_face_section\(`).
- `topo::link::Link::of(p, sectors)` — `pub(crate)`. 1 `Link::of\(` (in `census.rs`) and 5 `Self::of\(` in `link.rs`, counted over that file's records alone; those inside `impl Link` are its. `Link::of_tangent(p, pieces)` — `pub(crate)`. 3 (`of_tangent\(`).
- `topo::link::second_order::reach(apex, d)` — `pub(super)`, takes a vector. 10 `\breach\(` in `second_order.rs`, and 1 `second_order::reach\(` outside it, in `link.rs`. `second_order::cusp(apex, m, ..)`, `pub(super)`, hands on its caller's vectors: 1 (`second_order::cusp\(`).
- `topo::splitting::order::sort_indices_by_point(points, plane, ..)` — `pub(super)`, takes the points it levers. 1 (`sort_indices_by_point\(`).
- `topo::pcurves::decision::edge_reach(carrier, span, points)` — `pub(super)`, takes extra points beside the carrier. 2 (`\bedge_reach\b`): one import and one call, both in `pcurves::site`.
- `topo::boolean::join::frame_reading(sa, on_a, on_b)` — `pub(super)`, takes two point lists. 1 (`frame_reading\(`).

**Hand-built geometry.** `Surface`, `Frame` and `Curve3` have public fields, a conic has a public constructor (`Circle::new(frame, radius)`), and `geom::cover::Cover` has public constructors (`points`, `segment`, `sphere`, …, referenced in about 24 files outside `geom`'s cover home). A length from any crate reaches every reading constructor through a carrier, a span or a cover the caller made: `circle_curvature_arm(&Circle::new(frame, len))`, `ExtentBall::of_cover(&Cover::segment(p, q)?)`. Inside geom-brep, `intersect::PlaneCylinder` has crate-public fields and is built by literal, and `Reach::ruled_pivot_lever` reads it.

**A caller's point.** `lever_from(p)` and `lever_from_either(p, q)` read from any pivot. The curvature home's `curvature_lever_arm(cone, p)` and `parallel_curvature_arm(s, p)` read `p`'s distance from the surface's axis, so a far point is a long arm; its `chart_curvature_arm(lane, chart, p, reach)` is `pub` and reads a chart at any point.

**Other spellings.** A door called through a macro's expansion; `Self::door(…)` in a trait impl for `geom_brep::Reach` written in another crate; generics nested three deep in a turbofish or a qualified path; and, inside geom-brep, a variant reached by `use crate::Reach::*`.

## Added 2026-11-20 (`oneround/lever-sites-brep`)

The geom-brep, step-import and sweep sites under "Geometry in the function" (16 references), each checked against the tree. 12 references are gone and their pins with them (59 to 47); 4 stay, and the lists above are brought up to that.

Converted, by what the site reads now:

- `carrier_eq::at_consumed_extent` and `reading` (3): `ExtentBall::lever_from_either(p, q)`, the ball's reach from the farther of the two pivots the datum is read between (two cones' apexes; a point of the second axis and its foot on the first). Shorter than the widened reach wherever the pivots stand apart.
- `dihedral::seam_face_reading`: `chart_curvature_arm(lane, chart, p, reach)` in the curvature home. `classify_seam` takes the scalar's certification lane (`Option<impl NurbsLaneOf<T>>`) in place of a closure, so the wrapper `classify_seam(.., chart_arm, ..)` is gone from the list above; topo's scalar policy answers the lane (`nurbs_lane()`, in place of `spline_chart_arm`). Same bits.
- `intersect::ruled_frame`: `Reach::ruled_pivot_lever(&PlaneCylinder)`, which returns the row's frame, the pivot, the gap read at it and the lever. Same bits.
- `pair_section::cylinders::region_span`, `bisector_nodes`, `cylinder_cylinder` (3): no ball is built; `placement` takes what it places, a ball or a point of no radius. Same bits.
- `props::curved::cone_boundary`: `uniform_curvature_arm(cone)`. **A correction to the row as filed**: the lever built there was never read. A cone's rim levels are lengths (`RimLevel::Length`), and `RimArms::level` is read for `Unit` levels only; the rim radius is read as the azimuth's rate, a `T`, which stays. The level arm is now the cone's own answer with no point to read at, poison. The field that holds an arm nothing reads is `work/nearpoint/rim-arms-holds-a-level-arm-a-cone-never-reads.md`.
- `step_import::reach::Reach::lever`: `geom_brep::cover_diameter(&cover)`. Same bits.
- `step_import::build::faces_reach` (2): gone. `choose` reads how far the edge's two faces run from both sides (`step_import::reach::faces_run`, a `LeverBounds`): from below, their boundary edges' extent (`geom_brep::edges_extent_inf`), for the section readings, which build on a definite sign; from above, their region's diameter (`Reach::lever`), for the tangent arm, which builds on its Zero. A pair of faces with no region refuses, naming the edge or face whose region does not read. A cone in the pair is levered from its apex, so each bound takes in the edge's reach from the apex on its own side (`geom_brep::edge_reach_bounds`).

**A second both-bounds read under it.** `topo::section_closed` takes one `extent`, which its doc calls the old edge's extent from below, and builds its own `LeverBounds::Between { below: extent, above: edge_extent_sup(seed) }` for the cone rows. The import hands it the faces' run from below, so that pair can be inverted: 4 m below and the seed edge's 2 cm above on the folded discs. A reading taken at both and acted on only where they agree is conservative whichever is the larger, so nothing unsound follows; it is two regions' bounds under one name, and the door taking a `LeverBounds` from its caller would make them one.

Reshaped and still pinned (the same door name, one reference each):

- `intersect::pair_bound`, `generator_lever`, and `pair_section::cylinders::decide_stance`, `of_crossing_cylinders`: each door now computes what its caller used to derive (the cosine and the window; the common perpendicular's foot, direction, sine and length) and hands it back, so nothing is computed twice. Each still takes loose geometry nothing ties to a carrier, an axis line and an angle, or two lines and two radii. What takes each pin off is on its line above: a pose for the cone, a cover for the crossing.

Left as they were:

- `pair_section::CylinderSpherePose::sweep_ball` (`of_cover_within`): the widening is the ruling sweep's clearance, the sphere's own scale floored at 64 escalation widths, and no extent of anything consumed. A door that takes no number would own the sweep's constant or take the band and hide the multiple. Whether that ball is an extent at all is the question to settle first.
- `sweep::blend::battery::extent_of` (`points_diameter`): its honest door, `edge_extent_inf`, reads a circular arc past a half turn shorter than the site's own nine samples do, and a sweep row pins the difference (`edge-extent-inf-reads-a-circular-arc-past-a-half-turn-short-of-its-diameter`).

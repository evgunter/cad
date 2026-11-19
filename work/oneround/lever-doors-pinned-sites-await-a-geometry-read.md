---
id: lever-doors-pinned-sites-await-a-geometry-read
kind: issue
title: 57 production sites still hand a lever door a number or a point of their own; lever-doors.sh pins them
status: open
opened: 2026-11-19
priority: P1
cost: H
---


`scripts/gates/lever-doors.sh` holds every `Lever`/`ExtentBall`/`Reach` constructor that takes a number or a point the caller computed. Each is referenced only in its home, and elsewhere at a per-file pin (`scripts/gates/lever-doors.counts`) that only falls. The 57 pinned references are the sites `oneround/lever-doors` stopped on: none has the consumed carrier, edge or face in hand at the call, and none was routed through `unsourced`.

Run `LINES_FOR=ALL bash scripts/gates/lever-doors.sh` for the current list with line numbers. By what each site has in hand:

**A bare curvature arm (5).**
- `geom_brep::ssi::point_arm`: a `κ` its five callers' lanes computed (`max_curvature` and kin), and an extent. No carrier.
- `geom_brep::dihedral::seam_face_reading`, spline arm: `chart_arm`'s certified number for a spline chart.
- `geom_brep::props::curved::cone_boundary`: `cone_arm`, the first rim's `|v|·sin α`, or a literal `1` when there is no rim. `Rim` does not hold its circle.
- `topo::bound_side` `descend`: `FaceDatum.lever: Option<T>` and `FaceBend::Implicit.radius`, two stored scalars. The first enters through `FaceDatum::of_face(.., lever: T)` from `PierceLever::within`, `NO_CURVATURE()` or `f64::MAX`; the second is `1 / implicit_max_normal_curvature`.
- `sweep::blend::battery::corner_independence`: a public `radius: T`, the blend radius.

**A classifier's pose numbers (10, all in geom-brep).**
- `carrier_eq` (3): `lever_from_within(pivot, apart)`, `apart` the distance between two apexes or an axis offset. It has both carriers' descriptions.
- `intersect` `ruled_frame` and `pair_bound`: `ruled_lever` and `generator_lever`, fed the pose's sine, cosine and gap. It has the `Reach` and both surfaces.
- `pair_section::cylinders` `decide_stance`: `of_crossing_cylinders`, fed the radii, sine and gap of the two cylinders it holds.
- `pair_section::cylinders` (3): `ExtentBall::point` of a point computed on an axis or at a node, placed against the reach.
- `pair_section` `sweep_ball`: `of_cover_within(sphere cover, pad)`, the pad a band multiple or the torus's major radius.

**A chord between two points (11, topo).** `boolean::fate` (2), `boolean::overlap`, `bound_side` (`BoundArm::Chord`), `offset_axial` `corner_arms`, `offset_together` `corner_arms`, `ring_path` (2), `ring_path::spline_wall` `travel`, `link::second_order::reach` and `boolean::join` `bool_connect`. Most hold a carrier and a span the points were evaluated from, so an edge extent or `Reach::of_span` is in reach. Two are a vector's norm in point clothing: `second_order::reach` is `|d|` as `point(apex + d).lever_from(apex)`, and `bool_connect` mirrors a point through an axis origin.

**A fold over a point set (20, topo).** `link` (3), `offset_axial` `axial_frame` (2), `pcurves::decision::edge_reach`, `solid_contain::selection_reach` (the fold's zero seed), `splitting::containment` (4), `splitting::join::crossing_reach` (2), `splitting::order` (2), `wedge_reach`, `separating::planar_axis` (a box's diagonal) and `boolean::join` (3, `map(ExtentBall::point)` into an enclosing ball). They hold vertices, crossings or samples of an entity whose cover topo can read (`crate::cover`). Eight of these references are `ExtentBall::point(p).lever()`, a fold's seed or a point's own reach, which is zero.

**An import's points (3).** `step_import::build::faces_reach` and `step_import::reach`: a spec's vertex points or box corners, before any body exists.

**Nine samples of a carrier (1).** `sweep::blend::battery::extent_of`: it holds the carrier and the span, and overlaps `edge_extent_inf` (`extent-reads-made-outside-the-extent-home`).

**A face's measured window (3) and a tolerance pad (1).** `Reach::of_face` at `topo::cover::cone_face_reach`, `boolean::join::surface_pair_reach` and `chord_join::wall_face_section` (BOXES `reach-readme-says-no-vouched-length-but-two-variants-are-one`), and `topo::cover::meet_within`'s `of_cover_within(cover, tolerance)`.

**Not sites (3).** `geom_brep`'s re-export of `points_diameter` and `points_lever_from` (2), and `topo::test_support_fixtures`, which is test support mounted without a `cfg(test)`.

**What the gate cannot see** (its header says the same): a `Surface`, `Curve3`, span or `Cover` built by hand and passed to a reading constructor; and the pivot, since `lever_from(p)` reads from any point the caller names. `geom::cover::Cover`'s public constructors (`points`, `segment`, `sphere`, …) are referenced in about 24 files outside `geom`'s cover home, so `ExtentBall::of_cover(&Cover::segment(p, q)?)` builds the same lever `ExtentBall::point(q).lever_from(p)` does, with no pin.

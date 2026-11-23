# Circle-reason `Circle::frame` readers, classified

Tree: `bd07a867e790d35fc4bc7d67e317ca5068e0e28f` (detached worktree `oneround-cri-classify`), read only; nothing built.

## Reason strings found

Every `#[expect(clippy::disallowed_methods, reason = ...)]` under `crates/*/src` (142 in all, equal to the sum of the pins in `scripts/gates/conic-centre-reads.counts`, and equal per file in all 57 pinned files) carries one of two reason strings:

- **79** x `"a circle's centre frame, which a circle stores: owed a reading through the conic (work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md)"`
- **63** x `"a conic's centre frame, derived where an ellipse is held on its arc: owed a reading through the conic (work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md)"`

The first is the circle reason: **79** functions, the count the brief gave. No third string and no malformed site.

## Labelling rule used

The label says what the function does **with the circle's centre/frame**, read in the body (and in the callee or caller where the body only passes it on). Where two labels apply the dominant use of the centre/frame is first. Line numbers are the `fn` line at this commit.

## Table

| # | crate | file | function (line) | label(s) | what it does with the centre/frame | confidence |
|---|---|---|---|---|---|---|
| 1 | geom-brep | `certify.rs` | `arc_of_circle` (1962) | CONSTRUCTION | builds the edge's `RevolvedPoint` scaffold description: an axis line from the circle's centre and axis | sure |
| 2 | geom-brep | `curve_face/circle_torus.rs` | `axes` (317) | DECISION | circle's pose against a torus axis: tilt, centre's axial height and radial offset (the `Axes` every circle-on-torus rung reads) | sure |
| 3 | geom-brep | `curve_face/circle_torus.rs` | `meridian` (367) | OTHER, DECISION | centre read only as a coordinate magnitude for the rounding charge added to the meridian deviation of the circle against a torus; the deviation itself reads `axis()`/`radius()` and `Axes` | likely |
| 4 | geom-brep | `curve_pair/on_surface.rs` | `starting_at` (422) | CONSTRUCTION | a new circle on the same centre, axis and radius with `u_ref` re-phased to start at a held point `p` | sure |
| 5 | geom-brep | `edge_nurbs.rs` | `arc_rim_measured` (1303) | EVALUATION | the circle's parameter (azimuth) at a chart row's corner control point, then placed on the edge's branch | sure |
| 6 | geom-brep | `implicit.rs` | `circle_sphere_harmonic` (1970) | DECISION | circle's distance range against a sphere: centre offset from the sphere centre resolved on the circle's frame (harmonic lo/hi, phase) | sure |
| 7 | geom-brep | `pair_section/gap.rs` | `read` (264) | EVALUATION, DECISION | centre gives the core point's second derivative (`centre - p`) and a rounding magnitude; the reading is the core point's distance to a spine line/circle and its derivatives | likely |
| 8 | geom-brep | `pair_section/gap.rs` | `critical_gap` (571) | CONSTRUCTION, DECISION | builds a `Cover::circle` record (axis line through the centre, radius) read only for its extent lever, inside the critical-gap sweep of the core circle against a spine | likely |
| 9 | geom-brep | `pair_section/winds.rs` | `azimuth` (222) | DECISION | tube angle of a foreign section point about the torus's core circle (height over its plane, distance from its axis less radius): the winding channel | likely |
| 10 | geom-brep | `pair_section/winds.rs` | `clear` (244) | DECISION | whether a piece's box certainly clears the core circle (certified sign of height / radial offset in the circle's frame) | sure |
| 11 | geom-brep | `pcurve_cache.rs` | `run_iso_arc_checks` (9261) | EVALUATION, DECISION | evaluates the arc's rational-quadratic control points (on-circle and tangent-corner points from centre, u_ref, v_ref) to certify the chart's boundary column is this circle | likely |
| 12 | geom-brep | `pcurve_cache.rs` | `torus_oblique_circle` (13044) | DECISION, CONSTRUCTION | Villarceau gates: circle centre against the torus centre, axis and equator, plane through the torus centre, tilt, radius; then phases the `FocalSection` pcurve from the centre's radial direction | sure |
| 13 | geom-brep | `props/curved.rs` | `require_one_chart_branch` (846) | DECISION | reads the frame's axis only: circle's axis against the sphere's polar axis (meridian or rim class), then pole-in-span margins | sure |
| 14 | geom-brep | `props/curved.rs` | `cylinder_windings` (1750) | DECISION | a rim's stored turn carried from its own centre to the cylinder's axis: the loop's winding about the axis (`EdgeTurn::carried`) | sure |
| 15 | geom-brep | `props/curved.rs` | `cylinder_boundary` (2167) | DECISION | circle is a rim of the cylinder: axis class, radius fit, tilt, centre off the axis; its axial level from the centre | sure |
| 16 | geom-brep | `props/curved.rs` | `require_on_cone` (2641) | DECISION | circle lies on the cone: coaxial, at the cone's radius for its centre's height, not tilted, not off axis (`require_cone_circle`) | sure |
| 17 | geom-brep | `props/curved.rs` | `cone_boundary` (3476) | DECISION | circle is a rim of the cone (`require_cone_circle`, rim incidence) and its slant level from the centre's height over the apex | sure |
| 18 | geom-brep | `props/curved.rs` | `sphere_loop` (4079) | DECISION, CONSTRUCTION | circle lies on the sphere (centre offset along its own axis, radius fit); stores `SphereArc {centre, axis, radius}` and end tangents `axis x (p - centre)` for Gauss-Bonnet | sure |
| 19 | geom-brep | `props/curved.rs` | `sphere_boundary` (5290) | DECISION | circle is a latitude rim or a meridian great circle of the sphere (centre on the axis / at the sphere centre, radius fit) and its latitude level | sure |
| 20 | geom-brep | `props/curved.rs` | `torus_chart_loop` (5893) | DECISION | a torus piece's turn carried from the circle's own centre to the torus axis or core circle: the loop's winding on the chart | sure |
| 21 | geom-brep | `props/curved.rs` | `torus_classify` (6310) | DECISION, CONSTRUCTION | circle is a parallel or a meridian of the torus (centre height, radius fit, plane through the axis); keeps `TorusArc {n_c, c_c}` | sure |
| 22 | geom-brep | `tangent.rs` | `tangent_span_bounds` (225) | DECISION | bounds along the span on the circle's residual and curvature drift against two surfaces, from its centre's offset to each surface's centre/axis (coaxiality defect) (`circle_span_bounds`) | likely |
| 23 | geom | `curves.rs` | `same_content_by` (262) | DECISION | whether another carrier is the same circle by stored content: frame origin, axis, u_ref and radius compared value by value | sure |
| 24 | geom | `curves.rs` | `data` (1183) | OTHER | the stored-data walk: hands centre, axis, radius and u_ref out as named datum fields for field-by-field readers (poison read, hash key) | sure |
| 25 | geom | `curves/on_carrier.rs` | `distance_within` (729) | DECISION | a point's distance from the circle's arc: whether its foot lies in the arc, from its offset from the centre against the arc's middle radius | sure |
| 26 | geom | `pole_form.rs` | `half_turn_curve` (51) | CONSTRUCTION | a new circle: the carrier turned a half turn about a line (centre placed, axis and u_ref turned) | sure |
| 27 | mesh | `sizing.rs` | `sphere_face_widest_cos` (590) | DECISION | the arc's height range along a sphere's polar axis (centre's height is the harmonic's mean), for a sphere face's latitude window in mesh sizing | likely |
| 28 | step-import | `adopt.rs` | `mapped_self_description` (538) | CONSTRUCTION | builds the imported edge's `RevolvedPoint` self-description: an axis line from the circle's centre and axis | sure |
| 29 | step-import | `build.rs` | `circle_column_match` (1927) | DECISION | a spline column (and a printed spline) lies on the circle's arc and turns about its axis in the edge's sense (`arc_residual` against centre, axis, u_ref, radius) | sure |
| 30 | step-import | `build.rs` | `column_circle` (1995) | CONSTRUCTION | a new circle: the recognised witness circle with its axis reversed, same centre, radius and u_ref | sure |
| 31 | step-import | `normalize.rs` | `mint_band` (226) | DECISION | a band rim circle is coaxial with the surface's axis (centre's distance off the axis, tilt) and which way it winds about it | sure |
| 32 | step-import | `normalize.rs` | `seam_vertex_on_loop` (510) | EVALUATION | the circle's point in the surface's `u_ref` direction (centre + radius*u_ref) and that point's parameter, to split the rim at the seam | likely |
| 33 | step-import | `normalize.rs` | `wall_inversion` (627) | DECISION | reads the frame's axis only: circle's axis against the cylinder/cone axis picks the wall's rims for the orientation cross-check | sure |
| 34 | step-import | `on_built.rs` | `printed_on` (97) | DECISION | certified sup of a printed spline's distance from the built circle's locus (`circle_locus_within` on centre, axis, radius) | sure |
| 35 | step-import | `on_built.rs` | `arc_on_circle` (268) | DECISION | certified sup of a printed circle's arc's distance from a built circle: height over its plane and distance from its centre | sure |
| 36 | sweep | `blend/arms.rs` | `chamfer_setback_point` (189) | EVALUATION | the circle's point at `p`'s azimuth (foot of `p` on the trim circle): centre + radial unit * radius | sure |
| 37 | sweep | `blend/arms.rs` | `frustum_rim_side` (1587) | EVALUATION, DECISION | the two trim circles' points in the rim's meridian (centre + radial*radius), then which side of the generator between them the rim lies on | likely |
| 38 | sweep | `blend/battery.rs` | `frustum_arm` (1510) | CONSTRUCTION, DECISION | the meridian sheet about the circle's axis line, in which the supports' coaxiality is decided and the chamfer frustum is minted | sure |
| 39 | sweep | `blend/battery.rs` | `curved_arm` (1633) | CONSTRUCTION, DECISION | the meridian sheet about the circle's axis line, in which the supports' coaxiality is decided and the fillet torus/cylinder is minted | sure |
| 40 | sweep | `blend/junction.rs` | `rim_trim_circles` (607) | CONSTRUCTION | hands out both trim circles as `(centre, radius)` pairs; the callers mint the band's scaled trim arcs about them (`scaled`) and the strip level's centre | likely |
| 41 | sweep | `blend/junction.rs` | `rim_carrier` (652) | CONSTRUCTION | `RimCarrier` record holding the rim circle's frame, which the band's new trim circles inherit (`frame.with_origin`) | sure |
| 42 | sweep | `blend/reach.rs` | `circular_reach` (1432) | CONSTRUCTION | an arc's window as wedge half-spaces through the centre with the end tangents as normals, bounds of the link's reach | likely |
| 43 | sweep | `blend/surgery.rs` | `arc_past` (2683) | EVALUATION | a point's azimuth on the circle, read past a start parameter | sure |
| 44 | sweep | `blend/surgery.rs` | `arc_distance` (2705) | DECISION | how near and how far the arc comes to a point `c`: height of `c` along the axis and distance from it, against the radius | sure |
| 45 | sweep | `blend/surgery.rs` | `arc_along` (2748) | DECISION | how low and how high the arc reaches along a direction from a point: centre's height plus/minus radius times the in-plane part | sure |
| 46 | sweep | `blend/surgery.rs` | `of` (3533) | CONSTRUCTION, DECISION | builds a `StripLevel` record: the rim's axial height over the trim circle's centre (rim above/below its trim decided) and window wedges about the centre | likely |
| 47 | sweep | `blend/surgery.rs` | `arc_within` (3921) | DECISION, EVALUATION | clips the circle's windows to a half-plane: centre's height against radius reach decides in/out/touch; the crossing angles from an azimuth | sure |
| 48 | sweep | `swept.rs` | `register_placed_carrier_end` (791) | EVALUATION | evaluates the carrier's end point (`circle_at(frame, radius, t)`) to register it equal to the placed sketch end | sure |
| 49 | sweep | `test_support.rs` | `arcs_at` (717) | TEST-SUPPORT | fixture scan: circle edges whose radius and centre's `y` match a station | sure |
| 50 | sweep | `test_support.rs` | `circle_arcs_where` (2042) | TEST-SUPPORT | fixture scan: circle edges a predicate on (centre, radius, axis) accepts | sure |
| 51 | topo | `boolean/join.rs` | `pt_germ_frame` (4317) | CONSTRUCTION | a `SectionLoop` record: the circle's axis line (centre and axis) as the frame a plane-torus section loop turns about | sure |
| 52 | topo | `boolean/ops.rs` | `sphere_pair_cut` (6764) | CONSTRUCTION | a `SphereCutIn` record storing the section circle's centre (foot), radius, normal and u_ref for the sphere re-chart/cut | sure |
| 53 | topo | `boolean/ops.rs` | `pair_seam` (6817) | CONSTRUCTION | reads `u_ref` only: the section circle's reference direction becomes the seam the sphere pair's re-charts share | likely |
| 54 | topo | `chart_iso.rs` | `traversal_kind` (70) | DECISION | a rim's chart row `v` from its centre's height on the chart's axis and its radius (`Chart::rim_v`); reads a coordinate, decides no margin | unsure |
| 55 | topo | `contain.rs` | `loop_circle` (538) | DECISION | every edge of a loop is an arc of one circle: centre offset, radius difference and axis tilt against the first circle | sure |
| 56 | topo | `offset_axial.rs` | `param_on` (2798) | EVALUATION | the moved endpoint's parameter: the angle between two points' rays about the circle's axis line, from an anchor | sure |
| 57 | topo | `offset_axial.rs` | `reauthor` (3083) | CONSTRUCTION | re-authors a sketch arc (`Arc2 {centre, radius, sweep}`) about the moved circle's centre flattened into the sketch plane | sure |
| 58 | topo | `region.rs` | `swept` (1049) | EVALUATION | the angle round the circle from its start to a point of it (`y - centre` against `u_ref`) | sure |
| 59 | topo | `region.rs` | `arc` (1061) | EVALUATION | the arc length round the circle from its start to a point of it | sure |
| 60 | topo | `region.rs` | `heading` (1093) | EVALUATION | the circle's tangent direction at a point of it (`axis x (y - centre)`) | sure |
| 61 | topo | `replace_face.rs` | `transport_curve` (1249) | CONSTRUCTION, DECISION | the offset circle: same frame at `radius + d` on a cylinder, re-minted about the slid apex on a cone, re-minted parallel/meridian on a torus (class decided on axis and centre height) | sure |
| 62 | topo | `replace_face.rs` | `homothety` (1456) | CONSTRUCTION | a new circle under a homothety about a point: centre mapped, radius scaled | sure |
| 63 | topo | `rim_wedge.rs` | `curve_sense` (755) | DECISION | reads the frame's axis only: the sense the circle's parameter runs about the rim's axis | sure |
| 64 | topo | `rim_wedge.rs` | `crossings` (799) | DECISION, EVALUATION | where the circle edge's interior meets a locus line or rim circle (closed-form circle-line / circle-plane / circle-circle roots from the centre), kept where inside the arc | likely |
| 65 | topo | `rim_wedge.rs` | `rides` (972) | DECISION | whether the circle is the rim's own circle (`same_circle` on centre, axis, radius) | sure |
| 66 | topo | `rim_wedge.rs` | `face_boundary_circles` (1046) | CONSTRUCTION, DECISION | `Rim {center, axis, radius, u_ref}` records of a face's boundary circles; the one caller pairs two faces' records through `same_circle` | likely |
| 67 | topo | `ring_path.rs` | `plane_torus` (1088) | CONSTRUCTION, DECISION | a `LoopSplit::Side` record whose direction runs from the torus centre to the first meridian circle's centre, telling the two section loops apart | likely |
| 68 | topo | `ring_path.rs` | `path_parity` (1398) | DECISION | crossing parity of a path's circle piece against loop arcs: its centre's offset from the torus's core axis, the plane through it, the ruling through its plane | sure |
| 69 | topo | `ring_path.rs` | `path_resolution` (2016) | OTHER | collects the pieces' and crest arcs' centres as stored terms whose spread sets the rounding resolution charged to the path's margins | sure |
| 70 | topo | `ring_path.rs` | `plane_roots` (2198) | DECISION | whether and where a circle piece meets a plane: centre's offset from the plane against the radius, then the two root parameters | sure |
| 71 | topo | `solid_contain.rs` | `sphere_chart_trim` (1351) | DECISION | a sphere face's boundary circle is a latitude rim or a meridian great circle (centre at the sphere centre / on the axis, radius, axis class) | sure |
| 72 | topo | `solid_contain.rs` | `wall_crosses_by_turn` (3050) | DECISION | how far a boundary circle is from a parallel of the cylinder wall (tilt plus centre's distance off the axis) against the root's clearance | sure |
| 73 | topo | `surface_group.rs` | `coaxial_margins` (72) | DECISION | the two margins that are zero when the circle is coaxial with an axis line: axis sine and centre's distance off the axis | sure |
| 74 | topo | `surface_group.rs` | `wrap_rims_within` (231) | CONSTRUCTION | returns the wrap's rims as `Rim {center, axis, radius, u_ref}` records (the coaxial decision is `WrapRims::holds`, not here) | sure |
| 75 | topo | `test_support_fixtures.rs` | `arc_cap` (2240) | TEST-SUPPORT, CONSTRUCTION | fixture: a plane and a cylinder surface on the circle's frame for an arc cap body | sure |
| 76 | topo | `torus_region.rs` | `on_edge_plane` (468) | DECISION, EVALUATION | whether a point of the edge's plane is on the edge's own circle or its mirror through the torus centre, then the point's place on the span from its frame coordinates | sure |
| 77 | topo | `validate.rs` | `ring_outer_meeting` (9816) | DECISION | two loop circles apart, nested, tangent or crossing, from their centres and radii (`circle_pair`) | sure |
| 78 | topo | `validate.rs` | `meet_segment` (9976) | CONSTRUCTION, DECISION | a `MeetSegment::Arc {center, axis, radius, u_ref, t0, t1}` record, which `segments_meet` reads to decide whether two edges meet | sure |
| 79 | topo | `validate.rs` | `locus_gap` (10536) | DECISION | a point's distance from the circle's whole locus: height along the axis and radial defect in quadrature | sure |

## Totals

| label | as the first (dominant) label | as a second label |
|---|---|---|
| DECISION | 39 | 12 |
| CONSTRUCTION | 22 | 4 |
| WRITER | 0 | 0 |
| EVALUATION | 12 | 3 |
| TEST-SUPPORT | 3 | 0 |
| OTHER | 3 | 0 |
| **total** | **79** | 19 |

Confidence: 62 sure, 16 likely, 1 unsure.

Total against 79: **79**, no difference. Per file, circle-reason + conic-reason expectations equal the `.counts` pin in every file; the circle-reason share per file is:

| file | circle reason | conic reason | pin |
|---|---|---|---|
| `crates/geom-brep/src/certify.rs` | 1 | 0 | 1 |
| `crates/geom-brep/src/curve_face/circle_torus.rs` | 2 | 0 | 2 |
| `crates/geom-brep/src/curve_pair/on_surface.rs` | 1 | 1 | 2 |
| `crates/geom-brep/src/edge_nurbs.rs` | 1 | 0 | 1 |
| `crates/geom-brep/src/implicit.rs` | 1 | 7 | 8 |
| `crates/geom-brep/src/pair_section/gap.rs` | 2 | 0 | 2 |
| `crates/geom-brep/src/pair_section/winds.rs` | 2 | 0 | 2 |
| `crates/geom-brep/src/pcurve_cache.rs` | 2 | 3 | 5 |
| `crates/geom-brep/src/props/curved.rs` | 9 | 2 | 11 |
| `crates/geom-brep/src/tangent.rs` | 1 | 0 | 1 |
| `crates/geom/src/curves.rs` | 2 | 2 | 4 |
| `crates/geom/src/curves/on_carrier.rs` | 1 | 1 | 2 |
| `crates/geom/src/pole_form.rs` | 1 | 0 | 1 |
| `crates/mesh/src/sizing.rs` | 1 | 0 | 1 |
| `crates/step-import/src/adopt.rs` | 1 | 0 | 1 |
| `crates/step-import/src/build.rs` | 2 | 0 | 2 |
| `crates/step-import/src/normalize.rs` | 3 | 0 | 3 |
| `crates/step-import/src/on_built.rs` | 2 | 1 | 3 |
| `crates/sweep/src/blend/arms.rs` | 2 | 0 | 2 |
| `crates/sweep/src/blend/battery.rs` | 2 | 0 | 2 |
| `crates/sweep/src/blend/junction.rs` | 2 | 0 | 2 |
| `crates/sweep/src/blend/reach.rs` | 1 | 0 | 1 |
| `crates/sweep/src/blend/surgery.rs` | 5 | 2 | 7 |
| `crates/sweep/src/swept.rs` | 1 | 0 | 1 |
| `crates/sweep/src/test_support.rs` | 2 | 0 | 2 |
| `crates/topo/src/boolean/join.rs` | 1 | 3 | 4 |
| `crates/topo/src/boolean/ops.rs` | 2 | 0 | 2 |
| `crates/topo/src/chart_iso.rs` | 1 | 0 | 1 |
| `crates/topo/src/contain.rs` | 1 | 0 | 1 |
| `crates/topo/src/offset_axial.rs` | 2 | 1 | 3 |
| `crates/topo/src/region.rs` | 3 | 0 | 3 |
| `crates/topo/src/replace_face.rs` | 2 | 1 | 3 |
| `crates/topo/src/rim_wedge.rs` | 4 | 0 | 4 |
| `crates/topo/src/ring_path.rs` | 4 | 5 | 9 |
| `crates/topo/src/solid_contain.rs` | 2 | 0 | 2 |
| `crates/topo/src/surface_group.rs` | 2 | 0 | 2 |
| `crates/topo/src/test_support_fixtures.rs` | 1 | 0 | 1 |
| `crates/topo/src/torus_region.rs` | 1 | 0 | 1 |
| `crates/topo/src/validate.rs` | 3 | 0 | 3 |

No WRITER carries the circle reason: the STEP writer's one expectation (`crates/step-export/src/writer.rs`, `edge_curve`) carries the conic reason.

## Observations that bear on the count (facts read, not proposals)

- **Frame read without the centre.** Rows 13, 33 and 63 read `frame().axis()` only; row 53 reads `frame().u_ref()` only. Rows 38, 39 and 51 take `frame().axis_line()` (centre and axis together, as an axis). Rows 41, 75 and the cylinder arm of row 61 take the whole `Frame` value.
- **Centre read only for a magnitude.** Row 3 (rounding charge), row 69 (rounding resolution) and one of the two reads in row 7 (`size`) use the centre's coordinates as a scale, not as a place. Row 8 puts it in a cover whose only consumed output is the extent ball's radius (`ExtentBall::lever` returns the radius, not the ball's centre); a ball's radius is translation-invariant, but `Cover::ball`'s own arithmetic was not traced, and a non-finite centre poisons the lever.
- **Records that carry the centre on.** Rows 18, 21, 40, 41, 46, 51, 52, 66, 67, 74, 78 store or return centre/axis/radius (or the frame) in a struct or tuple that another function reads; rows 66 and 78 exist to feed a decision (`same_circle`, `segments_meet`).
- **Evaluation that could not be `point_at(t)` as written.** Rows 32, 36 and 37 form `centre + direction * radius` for a direction that comes from other geometry (a surface's `u_ref`, a foreign point's radial), i.e. the circle's point at another thing's azimuth; row 11 also forms the tangent-corner points at `radius / cos(h/2)`, which are off the circle.

## Not classified by reading, or classified with a reservation

None was left unclassified. The ones where the label is a judgement:

- **Row 54 `traversal_kind` (unsure, DECISION vs OTHER).** It hands centre and radius to `Chart::rim_v` (`crates/topo/src/chart.rs:180`), which reads the centre's height on the chart axis and returns the chart `v` of the coaxial circle at that height and radius. Its own doc says it "decides nothing; it reads the coordinate". It is a reading of the circle against a surface, with no margin; DECISION is the nearest label.
- **Row 3 `meridian` (OTHER first).** The function's product is a deviation of the circle against a torus (a DECISION input), but the gated read, `frame().origin()`, enters only the rounding charge.
- **Row 8 `critical_gap` (CONSTRUCTION first).** The gated read builds a `Cover::circle` used for a lever; the function as a whole decides a tangency gap, which reads the circle through `core.point`/`core.velocity` and row 7.
- **Row 7 `read` (EVALUATION first).** `centre - p` is the circle's second derivative at `phi`; the gap it feeds is a distance of the circle's point to a spine, so a reader who counts by the function's purpose would call it DECISION.
- **Row 9 `Channel::azimuth` (DECISION).** An angle of a foreign point about the core circle's tube, summed by the caller into a winding; a reader could call it EVALUATION of an angle, but the point is not on the circle.
- **Row 27 `sphere_face_widest_cos` (DECISION).** The centre's height over the sphere's equator is the mean of a height harmonic whose other two terms come from `eval(0)` and `eval(pi/2)`; it is an extent of the arc against the sphere's axis, with no margin decided.
- **Rows 11, 37 (EVALUATION first).** Both evaluate points from the frame and then decide; the decision does not read the centre except through those points.
- **Rows 40, 66, 67 (CONSTRUCTION first, on the caller's use).** The bodies only repackage the centre; the label follows what the callers do with the package (`scaled` trim arcs and `StripLevel` for row 40; `same_circle` for row 66; loop side tests for row 67).
- **Rows 42, 46 (CONSTRUCTION).** The wedge half-spaces are planes through the centre with end tangents as normals; each such plane also contains the arc's own end point, so the centre is one of several points that would state the same plane. Recorded as read, not as a proposal.
- **Row 24 `Curve3::data` (OTHER).** Neither a decision nor a construction: the generic field walk that names the centre as the `Center` datum for whoever folds it.

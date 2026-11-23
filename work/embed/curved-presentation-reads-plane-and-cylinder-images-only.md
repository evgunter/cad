---
id: curved-presentation-reads-plane-and-cylinder-images-only
kind: issue
title: The curved presentation reads a harmonic image on a plane or a cylinder and a section ellipse's focal image on a cone, so a Projected image (a hyperbola or parabola section of a cone), a Villarceau circle, a non-coordinate harmonic image on a torus, and a spiric, fitted or general image still refuse at depth 0
status: dispatched
opened: 2026-10-29
priority: P0
cost: M
branch: embed/cone-image
---

## Finding

From CURVEDEP (`departure-presents-no-curved-shared-edge`).

`present_curved` (`crates/topo/src/injective.rs`) reads a curved edge's chart curve where it has a form `Form::of` gives:
- a plane's image of a circle or ellipse carrier (`plane_conic`), a `Trig`;
- a cylinder's harmonic row winding the azimuth once with no linear height term, a `Trig`;
- a cone's `FocalSection` image, a plane's section ELLIPSE (`Focal`), read as a chart curve alone (2026-11-23, below).

Everything else stays `unread` (`Structural`), refusing at depth 0:
- **a hyperbola or parabola section of a cone**: it arrives as a `Nurbs` carrier with a `Projected` image, never as `FocalSection` (2026-11-23, below; the two cone cavities);
- a `FocalSection` image on a torus (a Villarceau circle), pinned unread by `injective::focal_rows::a_villarceau_row_on_a_torus_is_still_unread`;
- a harmonic image on a sphere or torus chart that is no coordinate line (a cone has none: its derivations are `ConeRim`, `ConeRuling` and `FocalSection`);
- `Projected`, `Spiric`, `Fitted`/`General` images;
- a plane face bounded by a spiric or spline carrier.

## What a fix needs

The column argument does not depend on the carrier. What is carrier-specific:
- an enclosure of the chart curve and its derivative over a parameter piece (`Harmonic::on`/`rate`);
- a separator's value along the face's image of the curve, tight at any parameter width (`Trig::psi`). A centred form over the piece is too loose.

For a cone's section ellipse the value form was not needed and is not built (2026-11-23): no separator that signs a cell beside one reads a value along the cone's image of it. The conic's own plane is zero along the arc by membership. At a ruling's corner the bisector plane through the shared ruling is zero along the RULING; the arc does not lie on it and is only decided to leave the ruling onto the face's side. And the cone's own carrier is read along the PLANE face's `Trig`. Were a pair to need a plane's value along the cone's image: the image `apex + v·(cos α·axis + sin α·radial(u))` over a focal chart curve is a trigonometric polynomial of degree 2 in the eccentric anomaly over `1 − e·cos E`.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

## 2026-11-18 — the two cone cavities read here once they have windows (EMBED, lane rational-window)

Main `c5bc653ae` with `a-rational-edges-whole-span-enclosure-is-unbounded-so-its-face-has-no-window`'s fix, f64, the cavity asked through the uncommitted `CavityGate` switch (`origin/embed/cavity-trace-measure`). Both rows that refused at the window now refuse here:
- `sf2b_r2_probes::r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`: `Uncertified { a: 1v1, b: 4v1, shared: Edge(8v1), arm: Departure }`, receipt 14 candidates, 100 discharged, 2 refused, 88 splits, 33 certified;
- `shell7_seam_corner::a_three_quarter_turn_cone_frustum_hollows`: the same pair and edge, 14 / 161 / 2 / 149 splits / 25 outside / 34 certified.

`1v1` is the cavity's plane cap, `4v1` its cone wall, and `8v1` the hyperbola between them, a rational quadratic. The presentation trace (`CV_TRACE`) reads `Err(Structural) | Err(Structural)` for that pair at depth 0: the plane bounded by a spline carrier, and the cone's image of a plane section, both on this row's list.

On the at-rest gate's branch (`origin/embed/gate`, `9d271e43e`) with the fix's first form (the edge covers read through `Cover::aabb(0.0)`), `mesh` `lift_probes::the_spline_section_bodies_read_lifts_that_close` (the census's named row for its 37-test window group, a cylinder face) moves from the window refusal to `Uncertified { a: 4v1, b: 5v5, shared: Edge(17v1), arm: Departure }`. Not traced to its cell; its shape fits this row.

**Re-banded P0** (2026-11-18, the EMBED orchestrator's ruling on PR 7120): valid bodies now reach this row and refuse here, the conical wedge, the three-quarter cone frustum and the spline-section group's named row above, and a blocker takes the band of what waits on it. The `embed/lune` lane (PR 7113) reads a sphere face beside an off-axis circle by re-posing the chart; whether that is the general form for the images listed here is not settled, and this row's unit is cut after that PR's review.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate and prints at each
`Structural` exit (`embed/trace-cone-torus-measure`): **24 tests,
departure at an edge, cone/cone**, all f64. Two cone faces of one cone,
split by a plane cut (an ellipse or hyperbola, a `FocalSection` image on
the cone's chart) or by a Boolean through the apex, refuse at depth 0
by `departure`'s "both stuck" exit: both faces `Structural` in their
whole windows, both raised at `beside_curve`'s "a corner where a curved
edge meets another at a vertex of the shared cell, which no corner
reading reads". **Corrected 2026-11-23** (`embed/cone-image`, printed): the edge the
two faces SHARE is a ruling in every one of the 20 that are this cause;
the curved edge at the corner is each face's own, unshared section arc,
`pcurve=FocalSection carrier=Ellipse`; none holds a hyperbola; and 4 of
the 24 hold no focal row at all (below). Traced:
- `reach_cone_split::a_tilted_cone_cut_is_never_misread_by_containment`
  (faces `5v1`, `7v1`, `u` a whole turn, sharing two arcs `[0, 1]` and
  their vertices);
- `near_circular_sections::a_near_square_split_of_a_frustum_validates_its_halves`
  (the same shape);
- `a_cone_apex_meets_a_vertex::two_cones_through_one_apex_build_across_the_pose_families`
  (a Boolean result: two faces each holding the apex, the same exit).

The corner is unread because its curved edge is no image `present_curved`
reads on a cone (`reads_curve`), which is this row's ground; that the
shared arcs' images are `FocalSection` is inferred from the fixtures
(plane sections of a cone), not printed. All three refuse the same way
on PR 7115's topo diff. The other 21 rows are assigned by bucket,
unchecked. Valid bodies, so P0 at the gate's landing. Cheapest:
`sweep::all reach_cone_split::a_tilted_cone_cut_is_never_misread_by_containment` (0.21 s).

## Seven more census instances, plane against torus (EMBED collinear-cover lane, 2026-11-22)

Seven rows of the census's plane/torus and torus/plane Boolean buckets
were assigned by shape to
`vacuous-reads-a-cell-at-a-corner-local-at-every-depth` and are not
that row's. Run on `embed/collinear-cover-measure` with the gate and
the trace, each is refused with both faces `Structural` in the cell
(`departure`'s "both stuck" exit), raised at `beside_curve`'s unread
corner, the exit of the cone/cone instances above. The pairs:
- `sweep::all germ_plane_torus_frame::a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name`
  (`3v1`/`5v5` across `14v1`);
- `sweep::all germ_plane_torus_spiric::the_donuts_spiric_poses_build_their_true_bodies`
  (`3v1`/`5v5` across `16v1`);
- `sweep::all germ_plane_torus_spiric::the_fat_corner_bars_lens_is_the_true_body`
  (`3v1`/`6v3` across `7v9`);
- `sweep::all germ_torus_doors::a_chord_across_the_hole_is_pierced_not_passed`
  (`2v9`/`3v1` across `16v1`);
- `mesh::all lift_probes::a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube`
  and `mesh::all reach_probes::a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row`
  (`2v5`/`3v1` across `6v7`);
- `mesh::all reach_probes::a_ring_stored_a_period_away_meshes_where_the_door_moves_it`
  (`3v1`/`14v1` across `10v5`).

The exit and the pair are printed; which edge's image is unread, and
that it is a plane section of the torus, is read off the fixtures.

## Three plane/cone instances (lane `embed/apex-separator`, 2026-11-21)

Three census tests the apex row (`a-cone-cell-presented-by-its-apex-side-is-signed-by-no-separator`)
held unchecked, traced on main `0c1eb97ad` with the gate (`CV_TRACE`):
their last cell pair is a cone part `Structural` at a shared conic edge
with no shared vertex near, the same before and after the apex lane's
change. Plane/cone, f64:
- `mesh::all reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`:
  cone `1v1` `Structural` at `u ∈ (1.66897, 1.71806)`,
  `v ∈ (−0.03516, −0.03281)`, against plane `10v1` read by its curve
  (`Zero::Curve(XLo)`), shared edge `9v1`;
- `sweep::all a_ring_on_a_cone_face::a_box_edge_through_a_cone_wall_crosses_it_at_the_rings_corners`:
  cone `1v1` `Structural`, the plane `Local`, edge `8v1`;
- `mesh::all split_rulings::a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes`:
  the plane by its side, cone `2v1` `Structural`, edge `19v3`.

## 2026-11-23 — the first unit cut from this row: a cone chart reads a plane section's image (EMBED orchestrator)

**Unit `embed/cone-image`. Letter H. Tier: dual** (a new certified enclosure, a trigonometric polynomial of degree two on a cone chart and degree four under a quadric separator, whose soundness is not to be believed by reading it). Stacked on the second landing branch (`embed/wave3-land`), which holds PR 7113's sphere reading and PR 7315's apex arm.

Why now: `embed/wedge-measure`'s final count (`work/embed/log.md`, 23 Nov) puts 269,522 of the 297,886 crease cell pairs the remetered reading refuses for good at no plane separator at all, presentations `Structural` or `Local` on one face or both. The margin is not what refuses there. PR 7113's review is done, which is what this row waited on.

**The unit takes:** the images on a CONE's chart that this row lists, measured on the 24 cone/cone census tests above and the two cone cavities (2026-11-18 section): a plane section of a cone (ellipse, hyperbola, parabola: `FocalSection`), any other harmonic image on a cone that is no coordinate line, and the corner those edges make at a shared vertex, as far as the corner is unread only because its curved edge is (`reads_curve`).

**Re-scoped by the lane's measurement, the same day** (next section): on a cone "a plane section's image" is the section ELLIPSE's `FocalSection` and nothing else. A hyperbola or parabola arrives as a `Nurbs` carrier with a `Projected` image, a cone has no other non-coordinate harmonic image, and the 20 census tests of this cause refused at the corner a shared RULING makes with a face's own section arc, not along a shared section. So the unit is: a cone's chart reads a section ellipse's focal image, at the corner and along the edge; no trigonometric polynomial is built.

**It leaves on this row:** a non-coordinate harmonic image on a torus chart, a Villarceau circle, `Projected`, `Spiric` and `Fitted`/`General` images, and a plane face bounded by a spiric or spline carrier (the two cone cavities' plane cap is bounded by a rational quadratic: the lane says whether they close without it). At the unit's close the lane rewrites this row's title and Finding to what is left.

## 2026-11-23 — a cone's chart reads a section ellipse's focal image (`embed/cone-image`)

**Re-scoped by measurement.** The unit was cut as "a cone's chart reads a plane section's image: ellipse, hyperbola, parabola". Printed on the landing branch `3e433e678` with the gate asking (`embed/cone-image-measure`, `measure/cone-image/`):
- 20 of the 24 cone/cone census tests refused at the root pair of two halves of one cone, sharing two RULINGS, both `Structural` at the corner where a ruling meets a half's own section arc (`FocalSection`, `Ellipse`). `reading` had no line arm on a cone, and `reads_curve` was false for the arc.
- 4 of the 24 hold no focal row: `a-cone-cell-near-two-shared-rulings-at-a-tiny-end-reads-local-at-zero-ofs-last-arm`.
- The two cone cavities' hyperbolas print `pcurve=Projected carrier=Nurbs` on the cone and bound their plane caps as spline carriers. Nothing mints a hyperbola or parabola as `FocalSection`. They refuse as before and stay here.

**What landed.**
- `Use::curve: Option<ChartCurve>` replaces `harmonic` and `wall`; `ChartCurve::Focal(Focal)` is the focal image, a Kepler azimuth and a harmonic slant. It holds the stored row's own image and is enclosed over a span by that row's chart box (`geom_brep::Pcurve::closed_form_span_box`, read at the interval scalar: the azimuth's hull of its two end evaluations, which is its range since it is monotone for `|β| < 1`, and the slant's chord-dip box, a superset). `Focal::of` requires `|β| < 1` of `β`'s whole enclosure.
- `Form::Focal`: `present_curved` reads the arc as a graph, alone in a cell and through a corner whose other edge is straight in the chart. Its image is read by no value (`Curved::psi` is `None`).
- `reading`'s line arm admits a cone: a ruling or a rim's arc shared, a section arc leaving it (`present_line_corner`).
- `cone_regular`: every one of those readings refuses a rectangle that holds `v = 0` or is a turn wide.
- A focal row gives the face's extent its enclosure (`row_of`), as a harmonic row does: without it a cone face with a section arc has no sheet and no coordinate line of it is placed.

**Measured** (f64, the 24 through the gate): 0 pass before, **12 after**, 11 of them with every ask `Embedded`. The other 12:
- the 4 with no focal row, unchanged;
- 5 at a curved corner at a vertex the pair shares alone, ruling or rim against a section arc (`a_needle_cone_cut_off_its_apex…`, `a_plane_moved_to_the_apex_band…`, `an_upright_cone_splits…`, the two `two_cones_through_one_apex…`): `departure-reads-no-curved-corner-off-its-shared-edge`, `a-torus-or-cone-corner-at-a-vertex-alone-reads-no-quadrant`;
- 2 at a corner of two section arcs (`every_op_on_a_cone_wall…`, `a_cone_face_met_on_its_axis…`): the same row's second bullet;
- `a_needle_funnel_cut_at_its_apex…` at `zero_of`'s last arm near the apex, no curved exit left;
- `lift_probes::every_loop_of_a_door_built_curved_face…` on a torus body.

**Found beside it:** `two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic` (a cap cut under a generic tilt refuses on its cut cap at f64) and `an-interval-cone-cut-refuses-at-its-caps-two-arcs-and-beside-its-rulings` (the cut cone is embedded at f64 and refuses at `Interval`).

**Two sightings beside the 24** (same run):
- `mesh::all lift_probes::every_loop_of_a_door_built_curved_face_has_a_lift_that_closes` (4.7 s) passes its cone body and now refuses on a TORUS body, plane against torus across a spiric: `Uncertified { a: 2v5, b: 3v1, shared: Edge(6v7) }`, `{ 3v1, 5v5, 6v7 }`, `{ 4v1, 8v5, 21v3 }`, the torus's row on each edge `pcurve=Spiric carrier=Spiric`; exits `beside_curve`'s unread corner (1,107) and unread edge (231), `zero_of` (1,959). A spiric image and a plane bounded by a spiric carrier are on this row's list; which of the two refuses first was not separated.
- `mesh::all reach_probes::the_unit_funnel_cut_near_its_apex_meshes_at_every_delta` passes as a test with 72 asks `Embedded` and one `Budget { stop: Pairs { max_cell_pairs: 65536 }, a: 4v1, b: 7v3 }` (the gate log prints no kinds for it): a budget stop, the walk's (EMBEDWALK), not traced.

**On the landing branch once it took main (`8d34076a9e`), same day:** the bodies above refuse again at `f64`, not on the cone: main's anchored ellipse states a cap's two section arcs a half turn apart and `one_stated_conic` (now `same_content`) reads two conics, so the plane cap has no corner reading (`two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic`, re-banded P0 there). The two halves of the wall hold when asked alone, in every placement. The 12-of-24 above was measured before that merge and was not re-run after it.

**Re-run on the merged tree** (`embed/cone-image-measure`): **1 of 24 pass** (`near_circular_sections::a_near_square_frustum_minus_a_box_builds`); the 11 lost each refuse on a Cone/Plane pair across a section arc. The cause is not this row's: the cap's two section arcs are stated a half turn apart and `one_stated_conic` did not read them one, which `embed/conic-symmetry` widens (`two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic`). On the landing tree this unit delivers the cone's side of those pairs and the wall's own corners; the 11 return when that unit lands.

**After the dual review (the fix pass, same day).** No code defect; pins and one fold:
- the focal enclosure had re-derived `geom-brep`'s closed-form chart box; it now reads it, and `Focal` holds the image it evaluates;
- the extent's own reader is pinned (`clearance::extent_rows::a_focal_rows_extent_holds_its_arc_and_its_slants_extreme`); no body the kernel builds observes an extent drawn too small, since that takes a face sharing a cell with the wall and crossing it in the part left out;
- the side each reading presents, and the rectangle, are asserted (`injective::focal_rows`); the far-slant speed is pinned at each of its two call sites by a tall rectangle whose near speed would grant the vertex concession;
- a kept-apex cut is a body row (`a_cut_cone_that_keeps_its_apex_holds_between_its_halves`); 150 placements of the halves and 18 flaps at the ruling's corner are rows.

Left unpinned at body level on this tree: the cone's side of a cone/cap pair (the pair refuses on the cap), and `Reading::Conic` on a cone, which goes live with `embed/conic-symmetry`.

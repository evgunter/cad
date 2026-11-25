---
id: curved-presentation-reads-plane-and-cylinder-images-only
kind: issue
title: The curved presentation reads a harmonic image on a plane or a cylinder only, so a curved shared edge on a cone, sphere or torus chart, or a focal-section, projected or spiric image, still refuses at depth 0
status: dispatched
opened: 2026-10-29
priority: P0
cost: M
branch: embed/cone-image
---

## Finding

From CURVEDEP (`departure-presents-no-curved-shared-edge`).

`present_curved` (`crates/topo/src/injective.rs`) reads a curved edge's chart curve only where its face image is a `Trig` (`Trig::of`):
- a plane's image of a circle or ellipse carrier (`chart_curve`);
- a cylinder's harmonic row winding the azimuth once with no linear height term.

Everything else stays `unread` (`Structural`), refusing at depth 0 as before CURVEDEP:
- a harmonic image on a cone, sphere or torus chart that is no coordinate line;
- a `FocalSection` image (a plane section of a cone, a Villarceau circle);
- `Projected`, `Spiric`, `Fitted`/`General` images;
- a plane face bounded by a spiric or spline carrier.

## What a fix needs

The column argument does not depend on the carrier. What is carrier-specific:
- an enclosure of the chart curve and its derivative over a parameter piece (`Harmonic::on`/`rate`);
- a separator's value along the face's image of the curve, tight at any parameter width (`Trig::psi`). A centred form over the piece is too loose.

For a cone, the image `apex + v·(cos α·axis + sin α·radial(u))` is a product of two harmonic channels, a trigonometric polynomial of degree 2. The same coefficient reading extends to it, at degree 4 under a quadric separator.

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
reading reads". Traced:
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

## 2026-11-23 — the first unit cut from this row: a cone chart reads a plane section's image (EMBED orchestrator)

**Unit `embed/cone-image`. Letter H. Tier: dual** (a new certified enclosure, a trigonometric polynomial of degree two on a cone chart and degree four under a quadric separator, whose soundness is not to be believed by reading it). Stacked on the second landing branch (`embed/wave3-land`), which holds PR 7113's sphere reading and PR 7315's apex arm.

Why now: `embed/wedge-measure`'s final count (`work/embed/log.md`, 23 Nov) puts 269,522 of the 297,886 crease cell pairs the remetered reading refuses for good at no plane separator at all, presentations `Structural` or `Local` on one face or both. The margin is not what refuses there. PR 7113's review is done, which is what this row waited on.

**The unit takes:** the images on a CONE's chart that this row lists, measured on the 24 cone/cone census tests above and the two cone cavities (2026-11-18 section): a plane section of a cone (ellipse, hyperbola, parabola: `FocalSection`), any other harmonic image on a cone that is no coordinate line, and the corner those edges make at a shared vertex, as far as the corner is unread only because its curved edge is (`reads_curve`).

**It leaves on this row:** a non-coordinate harmonic image on a torus chart, a Villarceau circle, `Projected`, `Spiric` and `Fitted`/`General` images, and a plane face bounded by a spiric or spline carrier (the two cone cavities' plane cap is bounded by a rational quadratic: the lane says whether they close without it). At the unit's close the lane rewrites this row's title and Finding to what is left.

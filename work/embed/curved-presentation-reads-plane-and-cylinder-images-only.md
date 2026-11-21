---
id: curved-presentation-reads-plane-and-cylinder-images-only
kind: issue
title: The curved presentation reads a harmonic image on a plane or a cylinder only, so a curved shared edge on a cone, sphere or torus chart, or a focal-section, projected or spiric image, still refuses at depth 0
status: open
opened: 2026-10-29
priority: P2
cost: M
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

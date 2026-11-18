---
id: curved-presentation-reads-plane-and-cylinder-images-only
kind: issue
title: The curved presentation reads a harmonic image on a plane or a cylinder only, so a curved shared edge on a cone, sphere or torus chart, or a focal-section, projected or spiric image, still refuses at depth 0
status: open
opened: 2026-10-29
priority: P0
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

## 2026-11-18 — the two cone cavities read here once they have windows (EMBED, lane rational-window)

Main `ff2319654` with `a-rational-edges-whole-span-enclosure-is-unbounded-so-its-face-has-no-window`'s fix, f64, the cavity asked through the uncommitted `CavityGate` switch (`origin/embed/cavity-trace-measure`). Both rows that refused at the window now refuse here:
- `sf2b_r2_probes::r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges`: `Uncertified { a: 1v1, b: 4v1, shared: Edge(8v1), arm: Departure }`, receipt 14 candidates, 100 discharged, 2 refused, 88 splits, 33 certified;
- `shell7_seam_corner::a_three_quarter_turn_cone_frustum_hollows`: the same pair and edge, 14 / 165 / 2 / 153 splits / 26 outside / 35 certified.

`1v1` is the cavity's plane cap, `4v1` its cone wall, and `8v1` the hyperbola between them, a rational quadratic. The presentation trace (`CV_TRACE`) reads `Err(Structural) | Err(Structural)` for that pair at depth 0: the plane bounded by a spline carrier, and the cone's image of a plane section, both on this row's list.

On the at-rest gate's branch (`origin/embed/gate`, `9d271e43e`) with the same fix, `mesh` `lift_probes::the_spline_section_bodies_read_lifts_that_close` (the census's named row for its 37-test window group, a cylinder face) moves from the window refusal to `Uncertified { a: 4v1, b: 5v5, shared: Edge(17v1), arm: Departure }`. Not traced to its cell; its shape fits this row.

**Re-banded P0** (2026-11-18, the EMBED orchestrator's ruling on PR 7120): valid bodies now reach this row and refuse here, the conical wedge, the three-quarter cone frustum and the spline-section group's named row above, and a blocker takes the band of what waits on it. The `embed/lune` lane (PR 7113) reads a sphere face beside an off-axis circle by re-posing the chart; whether that is the general form for the images listed here is not settled, and this row's unit is cut after that PR's review.

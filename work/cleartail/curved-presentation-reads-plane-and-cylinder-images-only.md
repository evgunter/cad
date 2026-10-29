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

---
id: a-tangent-edge-along-a-torus-meridian-cannot-be-certified
kind: issue
title: A tangent edge along a torus meridian circle cannot be certified, and a mixed fillet chain now mints one
status: open
opened: 2026-11-04
priority: P0
cost: H
refs: [a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses]
---

Filed by the band-mixchain lane (2026-11-04).

`crates/geom-brep/src/tangent.rs` (doc of `tangent_certificate_lane`) names this class as latent: two surfaces tangent along a torus's MERIDIAN (minor) circle are jet-determinate and inside the certificate lane, but `tangent_span_bounds` measures a large deviation there. So certification refuses `ResidualExceeded { TangentHull }`, which that doc calls "in-lane but uncertifiable". The doc says no constructor mints the configuration. One does now.

A rounded outline filleted whole is a closed chain whose line links blend with a cylinder band and whose arc links blend with a torus band. Two consecutive bands meet G1 along the rolling ball's section at the junction. That section is the cylinder's transverse circle and the torus's meridian circle, with centre on both spines and radius `r`. The must-carry rule demands the intrinsic tangency there, and certification then refuses at `geom_brep::certify` (`tangent_hull_sup`).

Witness, measured on branch `band/closed-mixed-chain`: the stadium of `crates/sweep/tests/review_closed_chain_junctions_r1_probes.rs::r1_mixed_line_and_arc_closed_rims_pair_by_the_bodys_incidence` (a 0.5 half-length, 0.5 end radius slot, filleted at 0.1).
- The section edge runs from (0.5, −0.5, 0.9) to (0.5, −0.4, 1.0).
- It lies between `Cylinder { origin (0, −0.4, 0.9), axis x, r 0.1 }` and `Torus { center (0.5, 0, 0.9), axis z, R 0.4, r 0.1 }`.
- It refuses `ResidualExceeded { TangentHull, margin 0.0199 }`. The rounded rectangle at r = 0.05 refuses with margin 0.0094 at `Interval`.

The geometry is exact: both surfaces contain that circle and share its normals.

What would close it is the "meridian-aware bound" the doc names: a span bound for tangency along a minor circle of a torus against a cylinder (and against a sphere or a torus, for the other mixed junctions an outline can have). The blend does not work around this; the band unit waits on it.

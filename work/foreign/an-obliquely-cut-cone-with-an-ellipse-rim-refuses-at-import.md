---
id: an-obliquely-cut-cone-with-an-ellipse-rim-refuses-at-import
kind: issue
title: an obliquely cut Open CASCADE cone, its rim an ELLIPSE, refuses at import: the pole-form meridian reads no crossing of a conic other than a circle
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by PR 6007's review (FOREIGN, 2026-10-29); its row is `crates/step-import/tests/review_pole_vertex_probes.rs`'s `an_obliquely_cut_open_cascade_cone_refuses_typed`.

A cone stated Open CASCADE's way (a seam generator to a valence-1 apex) whose base is cut by an oblique plane has an ELLIPSE rim (and OCC states even a coaxial cut that way in the probe's generator). The chart-singularity pass completes the pole form with the seam's half-turn twin and runs it to the first boundary edge it meets; `crossings` (`crates/step-import/src/normalize/singularity.rs`) reads lines and circles only, so the import refuses typed, "a boundary carrier the chart-singularity normalization reads no crossing of" — one of the spec's listed refusals. Main refuses the same files too, earlier, at its LINE-edge gate, so nothing regressed; but an obliquely cut cone is an everyday shape.

To close: a line × ellipse crossing in `crossings` (the generator meets the ellipse's plane at one point; check it lies on the ellipse within the band), with the edge-parameter reading for an ellipse (`param_at`'s eccentric anomaly, the convention `geometry::endpoint_params` already derives) and the sampled-distance arm of `nearest_on_edge` replaced by the same. Interop on an everyday shape: P3 by the band definitions.

---
id: offset-axial-reads-an-unlisted-kind-as-a-default-value
kind: issue
title: offset_axial reads an unlisted kind as a default value (residual zero, length the chord) where a typed refusal belongs
status: open
opened: 2026-10-14
priority: P3
cost: E
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

Two single-kind wildcards in `crates/topo/src/offset_axial.rs` answer with a value instead of refusing:

- `surface_residual` (`:2556`) has `_ => T::zero()`. That residual feeds the decision `offset_axial_edge_on_surface` (`:677`), so an unlisted kind would read as ON the moved surface. The comment on the torus arm records that this already happened once, for torus.
- The edge-length list (~l.1097) has `other => chord distance`. That is a lower bound on the arc length, not the length, and the door's conditioning is levered by it.

Neither is reachable today. The constraint builder refuses Nurbs/Approx charts `TogetherAxialUnsupported` (~l.1055), and the door's edges are lines and circles. So this is latent: the next kind admitted upstream gets a silent default.

## Owed

Re-aimed to D3 "One reading per question": no arm answers a default value, and an unlisted arm refuses typed, naming what it lacks.

- Both wildcards become exhaustive matches whose unlisted kinds refuse typed, so admitting a kind upstream is a compile error here rather than a zero.
- `surface_residual` is one of the point-to-carrier spellings on `honeone/point-to-carrier-distance-is-spelled-seven-times`. Once that reading lands, this door reads it rather than keeping its own; until then the exhaustive match above is what is owed here.

---
id: offset-axial-reads-an-unlisted-kind-as-a-default-value
kind: issue
title: offset_axial reads an unlisted kind as a default value (residual zero, length the chord) where a typed refusal belongs
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

Two single-kind wildcards in `crates/topo/src/offset_axial.rs` answer with a value instead of refusing:

- `surface_residual` (`:2556`) has `_ => T::zero()`. That residual feeds the decision `offset_axial_edge_on_surface` (`:677`), so an unlisted kind would read as ON the moved surface. The comment on the torus arm records that this already happened once, for torus.
- The edge-length list (~l.1097) has `other => chord distance`. That is a lower bound on the arc length, not the length, and the door's conditioning is levered by it.

Neither is reachable today. The constraint builder refuses Nurbs/Approx charts `TogetherAxialUnsupported` (~l.1055), and the door's edges are lines and circles. So this is latent: the next kind admitted upstream gets a silent default.

## Owed

Both become exhaustive matches whose unlisted kinds refuse typed, so admitting a kind upstream is a compile error here rather than a zero.

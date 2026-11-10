---
id: an-edge-ending-on-a-degree-one-crease-mints-no-pcurve
kind: issue
title: An edge ending on a degree-1 spline chart's crease mints no pcurve: the projected lane's envelope refuses it at the crease end (ResidualExceeded, Envelope, sample 0), so a fold's ridge corner reads Unplaced and the sector reader's straddle reading has no topo body to reach it
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found while building the folded-chart row the review of PR 6749 asked
for (`onecopy/crease-self-touch`).

## What

`sector_face::tests::roof_ridge` (`crates/topo/src/sector_face.rs:826`)
builds a house-profile prism, `(0,0) (1,0) (1,1) (½,1½) (0,1)` extruded
over `z ∈ [0, 1]`, kills its vertical ridge edge so the two roof slopes
are one face, and carries that face on a degree-1 chart creased at
`u = ½` along the ridge. Every roof edge lies on the chart exactly: the
bottom and top rim edges are the iso lines `v = 0` and `v = 1` on each
slope, and the struts the iso lines `u = 0` and `u = 1`.

`crate::pcurves::mint_pcurves` (`sector_face.rs:873`) refuses it:

    Certify { half_edge: HalfEdgeKey(7v1), error: ResidualExceeded { check: Envelope, sample: 0 } }

Half-edge 7 is the bottom rim edge from the ridge `(½, 1½, 0)` to
`(0, 1, 0)`: an edge whose pcurve ends on the crease run. The envelope
refusal is the projected lane's (`geom-brep` `pcurve_cache/projected.rs`,
check 4). It reads the chart over the pcurve's box, and a box touching
the knot takes in the span on the crease's other side too. An edge that
crosses a crease transversally inside its span mints: the cube-top
fixtures `split_at_half` and `split_at` mint before they split.

A second shape fails as well, with `FittedLimb { limb: HullValueChart }`:
a cube top whose degree-1 chart has a collapsed middle span, columns
`x = 0, ½, ½, 1` on knots `0, 0, 0.4, 0.6, 1, 1`.

## Why it matters

A real body with a spline face folded at a boundary vertex gets no pcurves
on the edges at the fold. So the face's corner there reads
`SectorFaceError::Unplaced` (fail-safe, `Unreadable`), and
`geom_brep::chart_normal_beside`'s straddle reading (a corner holding
both sides of a fold refuses unless the two one-sided normals are decided
equal) is reached by geom-brep rows only, never through a topo body.

## Pinned

`sector_face::tests::a_corner_on_a_folds_ridge_with_edges_down_both_slopes_refuses`
is a disabled red citing this row. It asserts that the ridge corner
refuses `Crease` and that each slope's corner reads its own slope. It
runs once the edges at the fold mint.

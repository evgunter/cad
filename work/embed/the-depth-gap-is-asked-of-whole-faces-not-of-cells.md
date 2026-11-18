---
id: the-depth-gap-is-asked-of-whole-faces-not-of-cells
kind: issue
title: The certificate asks the depth gap of two whole faces, so a face that lies partly past the other's carrier, and a spline wall beside a near-parallel one, are still parted cell by cell
status: open
opened: 2026-11-18
priority: P2
cost: H
---


Found by lane `embed/nested-tori` (2026-11-18), measuring
`nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance`.

## Evidence

`topo::clearance` asks the census's depth gap of a face pair's root
only (`apart_in_depth`, read off `topo::face_cover` of each face). Two
shapes it does not part:

- **A face that lies partly past the other's carrier.** The two
  faces' depth ranges about every proposed site then overlap, so the
  root has no gap, and each cell pair is decided by boxes and supports
  as before. A cell of such a face that lies wholly on one side has a
  gap, but nothing reads a cell's depths: `geom` covers a carrier
  whole (`Surface::cover`) and a curve over a span
  (`Curve3::cover_over`), and a surface over a chart rectangle has no
  cover.
- **Two spline walls.** A spline face's cover is its control net,
  which proposes no site. The tour's Klein loop (two near-parallel
  spline walls) is unchanged at 51,302 splits of 65,536, and the lily
  blades and the twisted loft are unchanged too
  (`work/clear/injectivity-price-on-tour-lofts-exceeds-the-pair-budget.md`
  has their table).

Measured on the tour walk, main `ff2319654` against this lane's
branch: 22 of 85 bodies split less, and none of them is a loft.

## What would part them

A cover of a chart rectangle of a carrier (a torus's or a sphere's
patch, a cylinder's wall within a slab, each already a `Cover` piece),
read by `gaps_in_depth` for a cell as it is for a face. For the spline
walls there is no site, so that pair needs another reading: the
offset relation between the two walls, where the builder knows it.

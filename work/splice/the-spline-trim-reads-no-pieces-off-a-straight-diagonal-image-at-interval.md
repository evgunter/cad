---
id: the-spline-trim-reads-no-pieces-off-a-straight-diagonal-image-at-interval
kind: issue
title: The spline trim reads no pieces off a lamina's straight diagonal chart image at Interval, so a station of a trimmed spline face there is never placed
status: open
opened: 2026-11-25
priority: P2
cost: M
---


## Finding

Found by lane `embedwalk-trimmed` (branch
`embedwalk/trimmed-freeform-window`), probing the trimmed spline
window's cell readings. `crates/topo/src/spline_trim.rs` is in no
program's paths; it is filed beside `spline_region.rs`, its one reader
here.

The fixture is `clearance_windows_on_a_spline_chart.rs`'s triangle
lamina (topo tests): the identity chart over `[0, 1]²`, corners
`(0, 0)`, `(1, 0)`, `(1, 1)`. Its diagonal's chart image is minted
`Pcurve::General` (a degree-1 `NurbsCurve2`) at both `f64` and
`Interval`; its other two edges are `IsoLine`s.

- At `f64`, `topo::cell_region::station_region` places `(0.75, 0.25)`
  `In`.
- At `Interval`, every station of the face is `None`, inside or out.
  `Trim::of` (`spline_trim.rs`) fails in `TrimChart::chain` with
  `SplineTrimGap::NoPieces`: `image_pieces` answers `None` for the
  diagonal's `General` image (its `bezier_pieces` over the span's value
  channel), at every loop of both faces.

So a trimmed spline face bounded by a straight non-iso image has no
point door at the certified scalar. The clearance walk keeps every
cell of it (`CellRegion::Undecided`), which is sound but drops nothing,
and a minimum walk has no upper end on it.

## What is needed

Read the degree-1 `General` image's pieces at `Interval` as at `f64`,
or say in `image_pieces` which interval spans it refuses and why.

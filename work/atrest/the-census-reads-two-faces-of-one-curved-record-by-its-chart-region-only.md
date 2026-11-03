---
id: the-census-reads-two-faces-of-one-curved-record-by-its-chart-region-only
kind: issue
title: The at-rest census reads two faces of one curved surface record by the chart-region lane alone, which refuses a seam branch or a touching boundary the two-record lane certified
status: open
opened: 2026-11-03
---


A boolean now leaves each glued one-carrier pair's faces on one surface
record before its merge (`topo::Body::unify_surfaces`, DESIGN
`[coincidence.door]`), so a curved group whose merge is skipped
(`SkippedMerge`) reaches the census as faces of one record where it used
to be faces of two. Three sweep rows go red at tier 3′ on branch
`onedoor/one-surface-record`:

- `crates/sweep/tests/curved_mergedoor.rs`
  `floating_and_mid_bore_pegs_ship_honest_with_their_period_closures`:
  `CensusUnsupported { subject: FacePair(..), cause: ChartRegion(SeamBranch) }`.
- `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
  `a_boss_over_a_hole_by_pose`: `UnrecordedContact { EdgeEdgeOverlap }`,
  `CensusUnsupported { cause: ChartRegion(TouchingBoundary) }` and a
  `CensusUndecidable` on two faces of the bore's one cylinder record.
- the same file's
  `a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`:
  `UnrecordedContact { EdgeEdgeOverlap }` and `CensusUnsupported`.

The pair of faces on one record is read through `chart_region`'s overlap
lane (`census.rs`, the `ChartRegionError` arm that carries
`SeamBranch`/`TouchingBoundary` as `CensusUnsupported`), whose curved-chart
inventory is narrower than the two-record carrier-pair lane that read the
same faces before. The edge-edge overlaps are the glued rims, between two
faces of one record now, which the census does not find recorded.

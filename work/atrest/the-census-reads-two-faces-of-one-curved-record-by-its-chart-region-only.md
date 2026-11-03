---
id: the-census-reads-two-faces-of-one-curved-record-by-its-chart-region-only
kind: issue
title: The census's one-key conformal arm cannot read two opposed faces of one curved record that abut along a rim
status: open
opened: 2026-11-03
---


Root cause, read on branch `onedoor/one-surface-record`. A boolean now
leaves each glued one-carrier pair's faces on one surface record
(`topo::Body::unify_surfaces`, DESIGN `[coincidence.door]`). So two
opposed faces of a Rest pair that a skipped curved merge leaves (a
boss's wall and its hole's wall) reach `census.rs`
`sweep_conformal_patches` (curved faces of one key, opposed senses). On
two records that arm never read them: they went to the cross-solid
backstop or the same-solid residue. So no two-record lane certified
these pairs. They were unread, or read `CensusUndecidable`.

The arm now goes to `one_record_overlap`. Where the region walk refuses
`TouchingBoundary` or `SeamBranch`, it takes the faces' own reading
(`face_interiors_meet`) where that reads `false`, which is a certificate.
That cleared `curved_mergedoor`'s floating and mid-bore pegs.

What remains is two rows in `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`:
`a_boss_over_a_hole_by_pose` and
`a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`. In both, the
opposed boss and hole walls abut along the rim. The region walk refuses
`TouchingBoundary`, and the faces' own reading answers `true`. That `true`
is no certificate: the witness search reads two crossings and their cones
as shared interior. So the arm reports `CensusUnsupported`, which the rows'
pins (only `EdgeEdgeOverlap` and `CensusUndecidable`) do not admit.

The rims' `UnrecordedContact { EdgeEdgeOverlap }` is not new. It is the
rows' pinned residue (`Reads::Unrecorded`; the second row asserts only it).

Options:
1. A region reading in the shared chart that decides two faces abutting
   along a boundary arc apart, the chart-region door's `TouchingBoundary`
   arm on a curved chart.
2. A `face_interiors_meet` variant whose `true` is certified for opposed
   faces.
3. Admit the arm's `CensusUnsupported` for an abutting pair in the two
   rows' pins, which states the gap rather than closing it.


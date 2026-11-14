---
id: the-census-reads-two-faces-of-one-curved-record-by-its-chart-region-only
kind: issue
title: The census's one-key conformal arm cannot read two opposed faces of one curved record that abut along a rim
status: closed
closed: 2026-11-03
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

The arm now goes to `one_record_overlap`. Wherever the region door
refuses for want of a lane (`TouchingBoundary`, `SeamBranch`,
`NonPlanarTrim`, `ArmUnbounded`, ...; not `Escalated` or `Corrupt`), it
takes the faces' own reading (`face_interiors_meet`) where that reads
`false`, which is a certificate. That cleared:
- `curved_mergedoor`'s floating and mid-bore pegs;
- `full_turn_bore_mate`;
- `rest_mate_every_op`'s arc-split bores;
- `two_rims_review_probes`' rod on a step.

Two rows in `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
then read `CensusUnsupported`: `a_boss_over_a_hole_by_pose` and
`a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`. In both, the
opposed boss and hole walls abut along the rim and the region walk refuses
`TouchingBoundary`. The faces' own reading answered `true` there because it
read every pair as facing the same way: its cone readings (`cones_meet`)
took the second face's corner as given, so an opposed face's corner read
as its complement. Abutting opposed walls read as sharing interior, and
congruent opposed faces read apart.

The rims' `UnrecordedContact { EdgeEdgeOverlap }` is not new. It is the
rows' pinned residue (`Reads::Unrecorded`; the second row asserts only it).

## Closed

2026-11-03 on branch `onedoor/one-surface-record`. The faces' reading
takes the two faces' facing (`census.rs` `Facing`), which
`one_record_overlap` reads off their senses on the record, and reads the
second cone reversed where they oppose. The abutting walls then read
apart, and the two closed-conic rows hold their pins unchanged.

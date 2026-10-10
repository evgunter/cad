---
id: a-transform-drops-the-contact-records-a-placement-carries
kind: issue
title: Transform drops its input's contact records while PlaceInWorld carries them verbatim, though both are rigid motions that keep arena keys
status: open
opened: 2026-10-08
---

Found by the review of INTENT stage 2 C (PR #4359, NOTE-3).

`wire_place_in_world` (`crates/editor-core/src/eval/wire.rs`) carries
its input's contact records and declaration rows verbatim, on the
argument that a rigid placement keeps every arena key; the posed row
`intent_s2_c_world::a_posed_placement_moves_its_copy_and_its_records_rigidly`
pins it. `wire_transform` returns `OpOut::plain` and drops a boolean
input's records, though a rigid `Transform` keeps the same keys. One
of the two rules is wrong, or the stated reason is not the one that
decides.

`Transform` retires in INTENT stage 3 (`[ev]` #4326: nothing moves a
body), so the cheapest settlement may be to let it retire; until then
a `Transform` of a boolean silently loses the records a placement of
the same boolean would keep.

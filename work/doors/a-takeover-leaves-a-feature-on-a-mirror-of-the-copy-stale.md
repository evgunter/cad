---
id: a-takeover-leaves-a-feature-on-a-mirror-of-the-copy-stale
kind: issue
title: The takeover rebinds nothing read through a mirror of a re-pointed copy: a fillet of C ∪ mirror(C) on an image edge goes stale
status: open
opened: 2026-11-08
priority: P1
cost: M
---


Found by the re-check of PR 6581 (lane `placing-recheck-6581`, m3). Filed by `placing-copy-reads` on the orchestrator's ruling.

**What.** Mirror a world copy `C` of `B` (`C ∪ image`), then fillet the union on an edge the image carries. Then fillet `B` as itself and accept the re-point (`SessionOp::RepointPlacements`, through `Recording::take_over`). The re-point writes the copy's `Source` and one `Rebind` on `C`'s own selection (the mirror's plane face), but nothing for the fillet's edge, which is read in the union through the image (`[From { read: image, of: … }]`). The fillet then fails: "edge 0 no longer resolves … Union …'s payload differs. Recourse: rebind it".

**Where.** `take_over::chains` follows the mirror (`names::verbatim_edge`, `Reflected`), and `rebinds_of` maps a name read downstream through `names::recarried(name, &in_body)`. One of two things is going wrong:
- `in_body`'s map for the image's output does not hold the image's name for the edge;
- the union's table does not answer the successor as one entity (`one`).

Not run further: the forward door is DOORS' (`take_over.rs`). The delete's half is owned by PR 6581: `continuation::rebinds` now walks the same chains (`take_over::carrying_chains`), mirrors and splits included.

**Done when** `crates/viewer/tests/gesture_placements.rs`'s `a_feature_on_a_mirror_of_the_copy_survives_the_repoint_and_its_delete` (now `#[ignore]` naming this row) passes un-ignored.

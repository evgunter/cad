---
id: a-merged-face-of-one-construction-reads-as-that-construction
kind: issue
title: A Merged face whose members are one construction is walked as its own origin, so a split's reunion row against it proves only at the carrier verdict
status: open
opened: 2026-10-23
priority: P3
cost: M
---

Found by INTENT's `a-symmetric-parts-seam-is-not-walked-as-one-construction` (PR 5616, 2026-10-23), and raised again by its review.

**The scene.** The L plate of `crates/editor-core/tests/a_mirror_is_a_construction.rs` is split by the plane of its own `x = 1` notch wall, read plainly (`Plane { face }`). The split fuses the coplanar section face into that wall. The half holding the upper arm names the result `Merged([Lateral(wall), SectionFace])`, minted at the split.

**The row.** Unioning the two halves back together records a `SameOpposite` `PlaneLadder` row between the other half's `SectionFace` and that merged face. The walk reads the merged face as `Origin::Minted(Merged([..]))` at the split. Since PR 5616, the `SectionFace` walks to the wall's construction, because the plane is read off the wall. So rung 1 leaves the row, and only the replay's `CarrierVerdict` proves it.

**The proposal.** Every member of the `Merged` face walks to one construction: the wall itself, and the section face through `plane()`. Such a face could read as that construction (`coincide::construction`'s minted arm), and the row would prove at rung 1. Members that walk to different constructions keep the merged face's own origin.

This needs deciding against what a `Merged` face guarantees: the boolean merges faces it found coplanar by value. The member walk would make the claim structural only where every member is one construction.

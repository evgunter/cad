---
id: a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation
kind: issue
title: A recorded tangency past the tangent certifier's set is held on its citation at tier 3: the census has no lane that confirms it
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found in the review fix pass of PR 7124 (`embed/records`, 2026-11-22),
ruled by the EMBED orchestrator the same day against the unit's
ratified text ("its result names the bar it paid (tier 3, or 3′)").

**The shape.** A plate (−2..2)²×(0..1), a neighbour (2..300) flush
with it along x, and a rod of radius 0.5 leaning 0.1·ε rad toward the
plate's y = 2 face, tangent to it within the band over the pair's own
extent (`crates/editor-core/tests/union_fold_face_face_touch.rs`,
`a_merged_face_far_longer_than_the_pair_holds_the_pass_s_line`). The
union's fold decides the tangency over the pair's extent and records
the face–face touch, citing its row. The neighbour's face merges with
the plate's, so the record lies on a face 298 long.

**What the census answers.** The plate and the rod alone, either
order: confirmed. Any of the four member orders with the neighbour:
`CensusUnsupported { FacePair, ContactLane(NotCertifiable { "a
face-face record on two carriers with no closed-form tangent locus
(plane x cylinder along a ruling, parallel cylinders)" }) }`. Over the
merged face's extent the leaning carriers have no tangent locus the
certifier holds, so it reads nothing.

**What the gate does** (`census::confirm_only`'s `reading`,
`crates/topo/src/census/confirm_only.rs`): this is the one cause it
classes as no lane. The record is held on its citation and the body's
bar is `Bar::Tier3`, pinned by
`union_fold_face_face_touch::a_record_no_lane_reads_is_held_at_tier_3`.
Every other cause the confirmation can raise refuses.

**Owed.** A reading that confirms a recorded tangency where it was
decided: over the extent its op decided it at (a record "is read where
it is minted and carried by lineage", `[ev]` PR 4951), or a certifier
whose set holds a tangency within the band over a longer face. Until
then a body holding such a record is tier 3, and nothing downstream
may read it as tier 3′. Population on 2026-11-22: one test in the fast
set reaches this class; the slow set, the tour and the demos are
logged in PR 7124's body.

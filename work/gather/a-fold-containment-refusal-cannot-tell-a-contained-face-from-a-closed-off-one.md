---
id: a-fold-containment-refusal-cannot-tell-a-contained-face-from-a-closed-off-one
kind: issue
title: A union's Contained refusal cannot tell a face inside another member from one closed off by a coincident face: the accumulation's rows read the same
status: open
opened: 2026-09-24
priority: P4
cost: D
refs: [member-space-look-through-stops-at-splits-containment-and-fragmented-merges]
---


## What

`look_through_merges` (`crates/editor-core/src/eval/wire.rs`) refuses a
member-space declaration whose accumulation-side face has NO descendant
row, while its member still derives it, as
`Diagnosis::ConsumedByFold { by: FoldConsumption::Contained }`
(`crates/editor-core/src/resolve/mod.rs`). The accumulation's name table
says only that nothing of the face reached the boundary. Two
compositions leave that same table:

- the face lies inside another member's volume (R2's `r2_p7`, pinned by
  `docm8_flat_merged::a_member_face_inside_a_later_member_refuses_as_contained`);
- the face met a coincident, opposite-facing face of another member and
  the pair verb closed both off (the internal wall of two blocks resting
  face to face).

The rendering says both ("within another member, or against a
coincident face of one"), so nothing reads as wrong. But the author gets
one answer for two different causes. The second case is **unmeasured**:
no fixture feeds a declared pair against a face that a resting contact
closed off at an earlier step.

## What could tell them apart

It would not be the name table. Each step's `topo::ContactRecords` is
structural and records the contacts that step admitted, and
`wire_union` keeps only the LAST step's (`last`). A per-step record of
which member faces a declared or detected resting contact consumed would
let the refusal name a closed-off face without re-measuring anything.
Whether that is worth a second arm is the open part. An author who
declared the contact that closed the face off already knows it.

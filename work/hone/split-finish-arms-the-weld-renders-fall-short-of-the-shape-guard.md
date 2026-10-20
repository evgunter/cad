---
id: split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard
kind: issue
title: Split/Finish's ResultEscalated runs 94 words with two face keys, and its Merge arm carries the merge's stage label, a key and no recourse
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`, whose PR welds the feature-tree census's
`Split/Finish/` rows to `topo::SplitFinishError`'s arms
(`every_forwarded_arm_has_a_row` in
`crates/editor-core/tests/refusal_concision_chains.rs`). The arms the
roster turned up with no row render short of the standard:

- **`ResultEscalated`** (`crates/topo/src/splitting/finish.rs`, its
  `Display`) renders 94 words over the 75-word budget, and names the
  aimed ray's two faces by key, on a payload of side `Below`,
  `BooleanDecision::ShellRole { near, far, others: 1 }` and a margin
  of 3e-10 in a ±1e-9 zero band: "the piece on the below side of the
  plane: whether a shell of the result bounds material or a cavity is
  undecided: margin 3e-10 lies within the zero band (±1e-9). an aimed
  ray met faces FaceKey(null) and FaceKey(null) of the result there. 1
  more of the result's readings lies in band, decided by the same
  tolerance. Recourse: move the split plane or the geometry so the two
  faces the aimed ray met are clearly apart, or, if this gap is
  intended, tighten the tolerance below 3e-11 m". With `others: 0` it
  is still over. No admission lets a budget breach stand, so the census
  holds no row for it: `ROWLESS` cites this file.
- **`Merge`** (a side's `MergeCoplanarError`, rendered whole) carries
  the merge's stage label, names its edge by key and states no
  recourse, on `MergeCoplanarError::PeriodClosure`: "the piece on the
  above side of the plane: merge_coplanar_faces: killing shared edge
  EdgeKey(null) would close the curved cosurface run's full chart
  period — outside the merge's inventory: …". Admitted on `FILED`
  (the label), `ADMISSIONS` (the key) and `FILED_NO_RECOURSE`, each
  citing this file.

## Repair shape

Bring `ResultEscalated` under the budget and name the faces in words,
then give it a row in `split()`'s `finish` list and take it off
`ROWLESS`. Have `Merge` drop the merge's stage label and key and end
in the merge's recourse (or the kernel-defect or not-yet ending), and
drop its three admissions.

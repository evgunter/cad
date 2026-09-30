---
id: merged-face-role-ambiguity-ends-in-no-recourse
kind: issue
title: topo: the merge's winding decision refuses its zero or split verdict as MergedFaceRoleAmbiguous with a stage label and no recourse
status: review
opened: 2026-09-30
branch: topo/merge-helpers-announce
pr: 3532
---


(TOPO, the receipt of PR 3506: `MergeCoplanarError::Escalated` has a
second raise, the merged face's loop winding, which the row that PR
closed did not name.)

## What

`merge_faces::merged_outline_ring` (`crates/topo/src/merge_faces.rs`)
reads each loop's winding (`loop_winding`) and takes the one positively
wound loop as the outline:

- an in-band winding escalates as `MergeCoplanarError::Escalated`
  (`MergeDecision::LoopWinding`, PR 3506), ending in the winding's lever
  with the tolerance the margin gives;
- zero or several positive windings (a zero winding counts as not
  positive) refuse as `MergeCoplanarError::MergedFaceRoleAmbiguous`,
  rendered "merge_coplanar_faces: merged face {face:?} has no unique
  positively-wound outline among its loops — outer/ring roles cannot be
  assigned; refused": a stage label, an arena key, and no recourse.

A zero winding is band-decided, so D4 ¶1 (iv) wants it to tell the
in-band arm's story. The other `merge_coplanar_faces:`-labelled arms of
`MergeCoplanarError`'s `Display` (`InputNotClosed`, `GroupKindSplit`,
`DeclaredOppositeOrientation`, `DeclaredCarrierUnsupported`, `Band`, …)
carry the same label.

## Repair shape

Carry the verdict that emptied the outline (a zero winding, with its
decided margin, or several positive ones), end the zero case in the
winding's `SizedDecision` (`LOOP_WINDING` in `merge_faces.rs`), give the
several-positive case its own lever, and drop the label from the arms
that carry it. `LOOP_WINDING` passes on either nonzero sign
(`SizedPass::NonZero`, since PR 3506's fix pass), so its zero arm
already offers the tolerance a nonzero decided margin gives, and a zero
winding carried to it reads right.

## What holds now (PR 3532)

`merged_outline_ring` reads every loop's decided winding, margin
included (`Body::planar_loop_winding_decided`, through
`decide_reported`), and refuses with the verdict that left no unique
outline:

- several positive windings: `MergedFaceRoleAmbiguous { face, verdict:
  OutlineVerdict::SeveralPositive { loops } }`, which names the loops
  and ends "Recourse: reshape the merged faces so their union is one
  connected region";
- no positive winding and a zero one: `Escalated { decision:
  MergeDecision::LoopWinding, diag }` with the zero winding's own
  margin, so it tells the in-band arm's story through `LOOP_WINDING`
  and offers the tolerance that margin gives;
- no positive winding and a loop the kernel cannot wind (NURBS, spiric,
  null-edge scaffold): `OutlineVerdict::Unread { loop }`, ending in the
  carriers the winding reads;
- every loop negative: `OutlineVerdict::AllNegative`, ending in the
  orientation tier 3 requires.

The `merge_coplanar_faces:` label is gone from this arm. The twelve
other arms that carry it are filed as
`merge-coplanar-refusals-open-with-a-stage-label`.

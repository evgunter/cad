---
id: merge-coplanar-escalation-offers-a-declaration-already-made
kind: issue
title: topo: MergeCoplanarError::Escalated renders its payload whole, advising a declaration on a pair already declared
status: review
opened: 2026-09-29
priority: P2
cost: E
branch: topo/torus-and-merge-one-story
pr: 3506
---

(TOPO, the §5 second pass of PR 3493. `work.py territory` gives
`crates/topo/src/merge_faces.rs` to TOPO and ZIP.)

## What

`MergeCoplanarError::Escalated` (`crates/topo/src/merge_faces.rs`, the
`Display` arm near :587) renders "merge_coplanar_faces: plane-identity
margin escalated verifying a declared pair ({diag})": the whole
`Indeterminate`, so it ends in `COINCIDENCE_RECOURSE`, unlabelled.

- It is raised only while verifying a pair the caller already
  declared (`merge_faces.rs` near :1873, `declared: true`), so
  "declare the coincidence" advises a declaration that is already
  there.
- Under `declared: true` the plane rung bridges an in-band offset or
  parallelism; what reaches this arm is `bool_plane_orient` deciding
  Zero, with an `INVALID` margin, so the payload also claims a poisoned
  reading (the same defect PR 3493 fixed on `DeclarationContradicted`).
- It opens with the `merge_coplanar_faces:` function label, which the
  shape guard reads as a stage prefix.

## Repair shape

D4 ¶1 (i): carry the decision as a closed type from the raise, and
compute the ending from the decision and its verdict; never route by
`diag.predicate`.

- `plane_eq`'s declared rung knows which rung refused. Carry it on
  `PlaneEqError::Escalated` (for example a `PlaneRung` beside the
  diagnostics), and carry the orientation rung's decided margin
  (`decide_reported`) rather than minting `INVALID` for its Zero
  verdict.
- The orientation decision passes on either definite sign, so its Zero
  verdict is band-decided: end it in a
  `geom_brep::recourse::SizedDecision` (`SizedPass::NonZero`) whose
  lever the merge's caller can take (turn one of the declared faces
  so the two clearly face the same way or clearly opposite ways), and
  the tolerance its margin gives.
- Name the decision in plain words (whether the declared planes face
  the same way or opposite ways), offer no declaration, and drop the
  `merge_coplanar_faces:` label. PR 3493 gave the sibling
  `DeclarationContradicted` arm its closed type
  (`boolean::Contradiction`) the same way.

## A second raise (PR 3506's receipt)

The variant had a second raise the row did not name: the merged face's
loop winding (`merge_faces::loop_winding`), which rendered the same
"plane-identity margin escalated verifying a declared pair" text. PR
3506 gives it its own decision (`MergeDecision::LoopWinding`); its
definite sibling is `work/topo/merged-face-role-ambiguity-ends-in-no-recourse.md`.

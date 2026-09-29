---
id: merge-coplanar-escalation-offers-a-declaration-already-made
kind: issue
title: topo: MergeCoplanarError::Escalated renders its payload whole, advising a declaration on a pair already declared
status: open
opened: 2026-09-29
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

Name the decision (whether the declared planes face the same way or
opposite ways) and a lever the merge's caller can take, without the
payload for an `INVALID` margin; drop the label. PR 3493 did this for
the sibling `DeclarationContradicted` arm through
`boolean::refusal_routes`.

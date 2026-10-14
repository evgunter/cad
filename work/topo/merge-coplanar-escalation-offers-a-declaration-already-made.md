---
id: merge-coplanar-escalation-offers-a-declaration-already-made
kind: issue
title: topo: MergeCoplanarError::Escalated renders its payload whole, advising a declaration on a pair already declared
status: closed
opened: 2026-09-29
priority: P2
cost: E
branch: topo/torus-and-merge-one-story
pr: 3506
closed: 2026-09-30
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

## Close (PR 3506)

The declared pair's orientation is the merge's own decision
(`MergeDecision::DeclaredPlanes(PlaneRung::Orientation)`, ending in
`merge_faces::DECLARED_ORIENTATION`): only a same-facing pair glues, so
it passes on a positive margin, and `DeclaredOppositeOrientation` is
its sign-certain arm, with the same lever and no label or face keys.
Its margin is the normals' cosine at the shared edge's chord, so what
an undecided margin measures is the chord, and the lever names both
moves; that the chord is 0 on a closed shared edge is
`work/topo/merge-orientation-rung-reads-a-closed-shared-edges-chord-as-its-arm.md`.
The Boolean's cross-operand doors keep `plane_eq::PLANE_ORIENTATION`
(either sign passes); a declared door's parallelism escalation (an
unreadable norm) ends as a defect at both the merge and the Boolean.

## Closed (2026-09-30, PR 3506)

Built to D4 ¶1 (i)/(iv), ratified by Ev in PR 3352.
- **Torus.** A spindle or horn torus is a shape the kernel never
  represents, per D1 ("spindle tori have no representation"), the type's
  own R > r > 0 convention, and the fact that no constructor mints one.
  The closed `geom_brep::TorusConvention {Tube, Ring}` is shared by
  tier 3 and the Boolean's pierce door. It ends both arms in one lever,
  with the valued tolerance only on the band-decided arm.
  - Through the Boolean's front door, a degenerate torus operand still
    stops earlier, at the trim placement. That is filed on CONTACT as
    `degenerate-torus-operand-meets-the-declare-menu-and-a-false-solid-is-fine`.
- **Plane ladder.** A closed `PlaneRung` is routed by `PlaneDoor`
  (Undeclared, Declared, Neighbours).
  - The Boolean's cross-operand orientation (`PLANE_ORIENTATION`,
    `NonZero`) and the merge's declared orientation
    (`DECLARED_ORIENTATION`, Positive) are separate decisions, because
    their doors accept different outcomes.
  - `DeclaredOppositeOrientation` is the merge decision's sign-certain
    arm.
  - Both levers name the shared edge's chord, which is what the margin
    measures.
  - No declaration is offered on a declared pair or on a same-operand
    pair.

The single review found the first head shared one orientation table
across the two doors (a fresh D4 (iv) fork). The fix pass split it.

Filed from the unit:
- the closed-edge chord arm;
- the plane-offset `INVALID` encoding;
- F7's same-operand coincidence;
- revolve's axis clearance (CARVE);
- the shape-guard stage label (TINT);
- rows on GERM, OFFSET and CONTACT.

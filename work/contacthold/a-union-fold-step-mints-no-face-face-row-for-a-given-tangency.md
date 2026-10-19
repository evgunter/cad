---
id: a-union-fold-step-mints-no-face-face-row-for-a-given-tangency
kind: issue
title: A union fold step's given tangent pair never reads its line against the two faces, so a touch through both interiors ships with no face–face record
status: closed
branch: contacthold/fold-step-face-face-row
pr: 4982
opened: 2026-10-12
priority: P2
closed: 2026-10-19
---


Found building the face–face row (`contacthold/face-face-touch-row`).

## What

A pair boolean mints the reduction's face–face row where its glue door
verified a tangency whose line runs through both faces' interiors
(`boolean::verify_declared_contacts` reads `Tangency::interiors`,
`boolean::locus_through_both_faces`; the row is
`ReductionContacts::ff`). A step of an n-ary union's fold is handed its
verdicts (`Verdicts::Given`) and builds its certificates in
`boolean::given_verdicts` (`boolean/mod.rs`), which records a given
`Tangent` pair's kinds and nothing else: the line is never read against
the two faces, so no `touches` entry exists and no face–face row is
minted. Where two members touch along a tangent line through both
faces' interiors and the fold joins them, the step's result carries no
record of the touch.

The pair door used to refuse such a union (`TangentSlitArmUnbuilt`);
the fold step never did, so this was already a silent drop there.

Not measured on a fixture; read from the code.

## Owed

A witness through `editor_core`'s n-ary union (two members touching
along a ruling through both interiors, a third folded in after), and
the fold step reading the given tangency's line against the faces, as
the pair door does, with the given pair's decision as the row's
citation (the union pass's tangency row, read through member lineage).

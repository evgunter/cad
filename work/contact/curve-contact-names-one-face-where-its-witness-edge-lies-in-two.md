---
id: curve-contact-names-one-face-where-its-witness-edge-lies-in-two
kind: issue
title: CurveContact { face_a, face_b, witness } names one face of a solid whose witness is an operand edge lying in two, the face-pair ambiguity JOIN-1 retires from the join's germs
status: open
opened: 2026-10-02
priority: P1
cost: M
design: true
refs: [JOIN-1]
---


Found by both designers on JOIN's in-face-edge fork (2026-10-02), filed
by JOIN.

## What

`crates/topo/README.md` C3 records a curve touch as
`CurveContact { face_a, face_b, witness }`. When the witness is an
edge of one operand, that edge lies in TWO of that operand's faces, so
the record's single face is a choice it cannot state. This is the same
ambiguity that left the join's section germs four loose ends on the
half-lap (`work/join/an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired.md`).
JOIN-1 retires it from the join by giving a germ a per-operand cell,
`OnEdge(edge) | InFace(face)`.

## What the taker owes

Both designers judged the end state to be one cell type shared by the
reduction's germs and the census's records. C3 is ratified, though, and the
census is the at-rest door, so changing the record's shape is an `[ev]`
question of its own, citing JOIN-1 as where the concept comes from. Not
blocked on JOIN-1, but cheapest once its cell type exists.

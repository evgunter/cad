---
id: a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins
kind: issue
title: A union fold step restates a given tangency on the first verdict's margin where several pass verdicts land on one face pair
status: open
opened: 2026-10-13
priority: P3
refs: [4982]
---



Found on PR 4982's review (`contacthold/fold-step-face-face-row`), MINOR m2.

## What

`editor_core`'s `wire::given_pairs` keys a fold step's given pairs by
(accumulation face, member face). Where an accumulation face descends
from several member faces and each has a pass verdict against the same
member face, the step is handed only the first verdict
(`GlueVerdict::given_at`) and its decision: its margin and line. The step's restated
tangency row (`boolean::given_verdicts`, `tangent_row`) then states that
one decision. The node's face–face record is unaffected: `wire::restated_given`
re-reads the lineage and cites every pass row it reaches.

This is exercised by two flush plates whose y = 2 faces merge, with a rod
tangent to the merged face (`review_4982_probes::probe_merged_face_two_verdicts`
on the review branch, order P1, P2, R). The record and its citations are
right there. Only the step-internal row's margin is the first verdict's.

## Owed

Hand a step one given pair per verdict, or restate the pair on every
verdict's decision. Either way the step's rows would say what the pass
decided rather than one of the decisions.

## Measured (FLUSH, 2026-10-26)

FLUSH's `flush/nary-union-door` moved the fold into the kernel
(`topo::nary_boolean`, `crates/topo/src/boolean/nary.rs`), and with it
`given_pairs` and `restated_given` (now `given_pairs` and
`restated_rows` there). The move keeps their logic: a step is still
handed one given pair per (accumulation face, member face), under the
first verdict's decision, and the step's restated row maps to the rows
of every verdict on the pair (`topo::NaryStep::restates`). So this row
is not fixed by the move. What changed is where the fix goes: the door
holds every verdict and every step, so handing a step one pair per
verdict or restating on each decision is local to `nary.rs`, with no
channel between two crates. A probe of the brief's pose through the
door (P1, P2 flush, a rod at x = 0.25 tangent to P2's y = 2 face) gave
only one tangency verdict on the merged face, (P2, R); (P1, R) recorded
no face-pair row, so that pose does not reach the defect.


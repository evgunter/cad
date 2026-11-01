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

FLUSH's `flush/nary-union-door` (PR 5772) moved the fold into the kernel
(`topo::nary_boolean`, `crates/topo/src/boolean/nary.rs`), and with it
`given_pairs` and `restated_given` (now `given_pairs` and
`restated_rows` there). The move keeps their logic, so this row is not
fixed by it: a step is still handed one given pair per (accumulation
face, member face), under the first verdict's decision (`given_pairs`'
`or_insert`), while the step's restated row maps to the rows of every
verdict on the pair (`topo::NaryStep::restates`).

It is reachable through the door (PR 5772's review, R1's probe): two
plates overlapping in z whose y = 2 faces are coplanar and merge, the
second raised by ε/100, and a rod along x tangent to the merged face
along z = 0.75, through both faces' interiors. The pass decides the
rod's tangency against the first plate at margin 0.0 and against the
second at ε/100 (1.0e-11 at ε 1e-9); the step's restated row carries 0.0 and stands for
both. It is pinned as it stands, so a fix flips it:
`crates/sweep/tests/face_face_touch.rs`,
`contacthold_m2_a_restated_tangency_carries_the_first_of_two_verdicts_margins`.

Where the fix goes changed: the door holds every verdict and every
step, so handing a step one pair per verdict or restating on each
decision is local to `nary.rs`. Which verdict's margin a restated row
carries is this row's question.

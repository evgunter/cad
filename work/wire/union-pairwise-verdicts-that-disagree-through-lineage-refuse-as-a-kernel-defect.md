---
id: union-pairwise-verdicts-that-disagree-through-lineage-refuse-as-a-kernel-defect
kind: issue
title: given_pairs refuses UNION_VERDICTS_DISAGREE as a kernel defect where two independently decided pairwise verdicts disagree through one accumulation face's lineage, a values case
status: open
opened: 2026-10-20
priority: P3
cost: M
---



Filed by WIRE's `wire-emis-rest` lane (branch `wire/emission-rest`), on
`names-emission-raises-panic-or-take-their-true-ending`.

## What

`crates/editor-core/src/eval/wire.rs`'s `given_pairs` hands a union
fold step the pairwise verdicts between its member's faces and each
accumulation face whose member-face lineage holds the verdict's other
face. Where two verdicts from two different member pairs reach one
(accumulation face, member face) key under two relations, it refuses
`NamingError::Emission { what: UNION_VERDICTS_DISAGREE }`, which ends
with the kernel-defect ending.

The two verdicts are separate decisions from values: member pair
(i, k) and member pair (j, k), each decided on its own margins. An
accumulation face whose lineage holds a face of i and a face of j (a
merge across the two) can then carry both. That the two agree is a
geometric expectation, not something any record or door guarantees,
so a correct kernel can plausibly reach the disagreement near a band.
It was left typed for that reason rather than made `unreachable!`.

## What it needs

A witness, or the argument that the merge which joins the two lineage
faces is itself decided by the verdicts and so forces agreement. With a
witness, the true ending is row 1 (a precision limit with its lever and
`KERNEL_LIMIT_RECOURSE`), on its own variant rather than
`NamingError::Emission`.

## Moved (FLUSH, 2026-10-26)

`given_pairs` moved into the kernel's n-ary door with the fold
(`flush/nary-union-door`, `crates/topo/src/boolean/nary.rs`). The
disagreement stays typed there, as `topo::NaryRefusal::VerdictsDisagree`
naming the step's member and the two faces, and `editor-core`'s
`verb_refused` (`crates/editor-core/src/eval/wire.rs`) maps it to the
same `NamingError::Emission { what: UNION_VERDICTS_DISAGREE }` as
before. What this row owes is unchanged; its true ending would now be
given where `VerdictsDisagree` is mapped.

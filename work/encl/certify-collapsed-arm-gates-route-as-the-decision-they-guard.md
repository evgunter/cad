---
id: certify-collapsed-arm-gates-route-as-the-decision-they-guard
kind: issue
title: geom-brep: dihedral_arm and nurbs_span_meter refuse under the decision they guard, with the poisoned-margin note, because the funnel folds the gate's verdict into MarginDiag::Invalid
status: open
opened: 2026-09-28
---



(ENCL implementer, residue of
`certify-span-and-zero-arms-cannot-carry-their-decisions-full-ending`.)

## What

`dihedral_arm` (`crates/geom-brep/src/dihedral.rs`, `classify_dihedral`)
and `nurbs_span_meter` (`certify.rs`, `run_checks`'s NURBS span arm)
run `decide_positive` on a lever arm or a metered extent before the
decision proper. A definite non-positive verdict comes back as
`geom_core::k_stats::decide_positive`'s `Indeterminate` carrying
`MarginDiag::Invalid`, and certification reports it under the decision
it guards (`Transversality`, `ParamSpan`), ending in that lever plus
"an unreadable or collapsed margin may indicate a kernel bug worth
reporting".

## Why it stays

At the call site the gate's own verdict is gone: the funnel folds a
definite non-positive sign and a poisoned margin into the same
`MarginDiag::Invalid`, and D4 ¶1 (i) forbids telling them apart by
predicate name. Routing the gate as its own decision needs the gate to
hand its verdict back (a typed collapse from `decide_positive`, which
is `geom_core`'s), and for `nurbs_span_meter` the question is already
split on NURBS's slate: a reversed knot domain reads as a collapse
there (`work/nurbs/nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one.md`).
The note the gates end in is true of both halves they fold (a
collapsed arm on sound geometry and a poisoned margin are both a
kernel or data defect), so the ending is not wrong, only unsplit.

## Shape

Once `decide_positive` returns the gate's verdict (or the NURBS row
separates the reversed domain), give each gate a `CertCheck` of its own
with its own lever, and route it through `certify::recourse`.

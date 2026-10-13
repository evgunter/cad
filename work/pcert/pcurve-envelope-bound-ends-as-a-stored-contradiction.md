---
id: pcurve-envelope-bound-ends-as-a-stored-contradiction
kind: issue
title: geom-brep: PcurveCheck::Envelope is a certified between-samples bound but ends as Unsized::Fit, so a loose envelope reads as a stored contradiction at rest
status: closed
opened: 2026-10-10
priority: P3
cost: E
branch: pcert/envelope-recourse-bound
pr: 5041
closed: 2026-10-13
---



(Filed by the ENCL orchestrator from the review of ENCL's PR 4504, which implements fork-log row 104: `Unsized { Defect, Fit, Bound }`.)

## What

`crates/geom-brep/src/pcurve_cache.rs` `PcurveCheck::recourse` maps `MapResidual | Envelope | EnvelopeTerm(_)` to `Unsized::Fit`. `Envelope` is the certified closed-form between-samples bound (its comment says "bounds on a fitted image"), the same shape as `CertCheck::TangentHull`, which PR 4504 moved to `Unsized::Bound`.

So at rest, a definite `Envelope` refusal still reads "the stored boundary does not match the face" (`topo::validate::classify_pcurve`, `C::ResidualExceeded { .. } => (WRONG, DEFECT)`) and ends in the defect ending. A loose envelope contradicts nothing.

`EnvelopeTerm(_)` incidence terms (Centre and the others) may be exact readings of stored data, so they may rightly stay `Fit`. Decide per term.

## Sibling (ENCL may take it)

`crates/topo/src/merge_faces.rs` `MergeDecision::DeclaredReach`'s comment calls its quantity "a bound over a ball enclosing the faces", yet it is `Unsized::Fit`. It is read only at Build, so the text is the same either way, but the name contradicts the comment. Reword the comment or move it to `Bound`.

## Repair shape

Move `Envelope` (and any `EnvelopeTerm` that is a bound) to `Unsized::Bound`. Give `classify_pcurve`'s at-rest reason a bound arm, composed by the reader per fork-log row 9. Pin one at-rest row.

## Closed (PR 5041, 2026-10-13)

`PcurveCheck::bounds_a_miss` is true for `Envelope` and every
`EnvelopeTerm`, and `recourse` routes them through `Unsized::Bound`.
Each term bounds a part of the miss from above, and the parts share
`t`, so they can partly cancel: no term is an exact reading. A sample's
`MapResidual` stays `Fit`, because it is the miss itself. The build
`Display` and `classify_pcurve`'s at-rest reason name the bound, and a
definite envelope refusal at rest ends in the kernel-limit recourse.
Rows: `a_refused_envelope_reads_as_a_bound_at_rest` (the `envelope-fit`
mutant turns it red) and `own_close_endings_follow_their_decisions`'
definite pcurve rows.

The sweep found a sibling in this file. The fitted lane re-wraps a
decided SSI limb refusal as `FittedCertificate`, which carried no
ending, so a hull-sup limb, a `Bound` at the SSI door, read at rest as
a stored contradiction. It now ends by its limb's check, as at the SSI
door. `merge_faces::MergeDecision::DeclaredReach` routes through
`Bound` (no text moves at Build); `PairReach`, whose margin never reads,
routes through `Defect`.

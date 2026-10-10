---
id: pcurve-envelope-bound-ends-as-a-stored-contradiction
kind: issue
title: geom-brep: PcurveCheck::Envelope is a certified between-samples bound but ends as Unsized::Fit, so a loose envelope reads as a stored contradiction at rest
status: open
opened: 2026-10-10
priority: P3
cost: E
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

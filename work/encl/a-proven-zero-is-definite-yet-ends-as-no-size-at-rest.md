---
id: a-proven-zero-is-definite-yet-ends-as-no-size-at-rest
kind: issue
title: geom-core/geom-brep: a symbolic-tier proven zero classes NoSize, so at rest it never takes RefusedReading::ContradictsStoredData; a proved-zero ParamSpan ends in lever plus note while a reversed span ends in the defect ending
status: open
opened: 2026-10-14
---


(Filed by the ENCL implementer of `a-lever-decisions-definite-arm-at-rest-drops-its-note`, from the PR 5101 review.)

## What

PR 5101 made a definite refused reading end one way at rest whichever arm routes it. The ending reads the reading's class (`Shows`, `crates/geom-core/src/predicate.rs`) and the decision's `StoredDefinite` (`crates/geom-brep/src/recourse.rs`, `SizedDecision::words`). Only `Shows::Refused` reads `SizedWords::refused`, so only a reading past the band on the refused side can end as a stored contradiction (`RefusedReading::ContradictsStoredData`, `KERNEL_OR_FILE_DEFECT_ENDING`).

A zero proved by the symbolic tier is just as definite: no tolerance and no subdivision moves it (`MarginDiag::shows`: "a proved zero is no size the user may intend"). But it classes `Shows::NoSize`, so at rest it ends in the lever and the decision's note:

- `CertCheck::ParamSpan` at rest, proved zero: "Recourse: move the geometry so this edge is not vanishingly short; an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect or a damaged file, worth reporting".
- The same decision at rest, reversed span (`Refused::Negative`, either route): `KERNEL_OR_FILE_DEFECT_ENDING`.

Both readings say the stored span contradicts the stored edge, yet they end in two stories.

## Why it is not a per-decision fix

`StoredDefinite` is one bit per decision, but the meaning differs by sign within a decision. For `DIHEDRAL_ARM`, a zero arm (an edge of no length, a cone's point) is a geometry case the lever reaches, while a negative arm (a stored radius below zero) is a contradiction (its `no_size` note says so). Making a proven zero read `ContradictsStoredData` for every `Contradiction` decision would end `DIHEDRAL_ARM`'s cone point as a defect.

## Question (design)

Should a decision state its stored-contradiction set per sign (for example `StoredDefinite` keyed by `Sign`, or a pass set of "signs that contradict stored data"), so a proven zero and a negative can each end as what they mean? Or does D4 ¶1 (iv) intend that only a refused *sign* is a stored contradiction, so a proven zero always keeps the lever and the note? Cite D4 ¶1 (iv) and `DIHEDRAL_ARM`'s doc (`crates/geom-brep/src/dihedral.rs`).

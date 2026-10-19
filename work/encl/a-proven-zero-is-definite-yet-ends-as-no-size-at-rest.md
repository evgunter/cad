---
id: a-proven-zero-is-definite-yet-ends-as-no-size-at-rest
kind: issue
title: geom-core/geom-brep: a symbolic-tier proven zero classes NoSize, so at rest it never takes RefusedReading::ContradictsStoredData; a proved-zero ParamSpan ends in lever plus note while a reversed span ends in the defect ending
status: closed
pr: 5349
opened: 2026-10-14
branch: encl/proven-zero-impl
closed: 2026-10-19
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

## Design pair (2026-10-14; three rounds, converged)

The two designers converged on one recommendation. It is put to Ev because it reads D4 ¶1 (iv) more precisely and extends it to the build door.

- **One kind per sized decision** replaces `StoredDefinite { Contradiction, Lever }`. It says what the decision's refused side is: a lever, a stored contradiction, or unminted. The no-size class and the past-band refused class both read it.
  - `Lever`: the lever plus the note at every door.
  - Stored contradiction: the lever plus the note at a build, and `KERNEL_OR_FILE_DEFECT_ENDING` at rest.
  - Unminted: `KERNEL_DEFECT_ENDING` at a build, and `KERNEL_OR_FILE_DEFECT_ENDING` at rest.
- **Readings that may show a size keep the lever.** That covers the offer, the floor, an unresolved reading and a straddle. The import door within ε_in is unchanged.
- **A proved zero ends as its exact and in-band siblings do** (D4 ¶1 (i)). Proof never changes a message.
- **Rows:**
  - `ParamSpan`: unminted.
  - `Transversality` and `TangentSecondOrder`: stored contradiction.
  - `DIHEDRAL_ARM`: `Lever`. Its note drops "a radius below zero is a defect", because a negative arm needs a datum that a profile refuses and that at-rest representability reports first.
  - `ParamWinding`: stored contradiction, unless the implementer shows nothing mints a winding past a full turn. In that case it is unminted.
- **Rejected:**
  - Classing a proved zero apart from a numeric one, because it forks exactly-on by tier against (i).
  - A per-sign `StoredDefinite`. No decision needs one once `DIHEDRAL_ARM` is `Lever`.
  - `Unminted` for `ParamSpan` alone, because it leaves a stored tangency at rest ending in a lever.
  - The two-valued form, because it leaves `ParamSpan`'s build ending with a lever its note denies.
- **Found on the way:** PR 5101 said `NotTransverse` at rest "was already the defect ending". In fact a Transversality or TangentSecondOrder zero verdict at rest ends in the lever alone, while `validate::classify_certify`'s reason says the faces are "tangent where its description says they cross".

## Ev's question (2026-10-14, again 2026-10-19): aren't stored contradictions kernel bugs?

Yes, past the import door. `[import]` already says no state is reachable only through import, and that a body ships only once the kernel's own at-rest gate passes it. So after import, a stored contradiction can only come from a kernel bug, and `[failure]` (D2) says it panics. The import door is the one place it is input; there it is the file's, refused typed and naming the file.

The revised clause (iv) says this. In the code:

- At the import door, `KERNEL_OR_FILE_DEFECT_ENDING` becomes a damaged-file refusal.
- At a build and past import, an unminted side and a stored contradiction panic.

**Gaps against `[import]` today** (survey on PR 5134): adoption keeps the file's carriers verbatim instead of recomputing them; a negative surface radius goes unchecked. These are EXCH defects against ratified text (`can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`, `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification`). The panic arms cannot land until those gaps close, or a damaged file would panic.

## Ev's decision (2026-10-19, PR 5134: "looks good!")

The revised clause (iv) is ratified. The implementation unit carries out "What it decides in the code" from PR 5134's body. Its panic arms wait on EXCH closing the import gaps listed above.

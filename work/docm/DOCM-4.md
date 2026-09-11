---
id: DOCM-4
kind: unit
title: Evaluation carries its document's identity; A4's refusal sentence narrows to the seam (DI2, DI3)
status: closed
opened: 2026-09-04
closed: 2026-09-04
pr: 1808
branch: docm/4-evaluation-identity
---


## Spec

`docs/DOCM-IDENTITY-DESIGN.md` DI2, DI3. `Evaluation` gains
`document: DocumentId`, stamped by `evaluate`; the memo lookup and every
door taking a (document, evaluation) pair — `product`, `assemble`,
`SolvedPoses::placement` — refuse a mismatch typed. `crates/editor-core/ASSEMBLY.md`
A4's refusal sentence narrows to "an evaluation that crosses the seam
refuses a moved pin", and the `pncad-py` audit page's wording is checked
against it. The finding it answers is `memo-admission-and-resolver-state`
(closed 2026-09-04, pointing here); the session's `Reevaluate` re-mount
(DI2) is CHROME's, on `document-seam-no-in-session-change-detection`.

## Closed (2026-09-04)

Merged as PR 1808 (ordinal 1800, sample #126). `Evaluation::document`,
the three pairing doors (A2a in `crates/editor-core/ASSEMBLY.md`), the
memo's whole-prior drop recorded as `Evaluation::prior_refused`, one
`ident::mispaired` predicate behind all four. A4's refusal sentence
narrowed to the seam. Residue with its own file:
`pair-doors-outside-the-three-do-not-check-document-identity` (the
doors beyond the three, and the class fix). The session's `Reevaluate`
re-mount is CHROME's on `document-seam-no-in-session-change-detection`.

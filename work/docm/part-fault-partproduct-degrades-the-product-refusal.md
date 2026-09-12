---
id: part-fault-partproduct-degrades-the-product-refusal
kind: issue
title: PartFault::PartProduct degrades a ProductError to message: String — trivially fixable now that ProductErrorKind exists
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the `checks-product-refusal-degrades-to-string`
lane's sweep, PR 2344. Reported by the lane, placed here by the
orchestrator; DOCM has no open PR touching it (checked).

`crates/editor-core/src/eval/parts.rs:420` builds
`PartFault::PartProduct { message: error.to_string() }` from a
`product::ProductError` — the typed refusal degraded to prose at a
consumer door, the same shape `CheckEvidence::SeparationUnavailable`
carried until `BooleanErrorKind` landed and `ChecksError::Product`
carried until PR 2344.

**It is now trivially fixable, which is the only reason this is worth
filing rather than noting.** PR 2344 landed
`editor_core::product::ProductErrorKind` (10 fieldless variants) with
an exhaustive `ProductError::kind()`, re-exported from the crate root
and from `pncad::document`. The change here is one field and one call
site; the class it needs already exists and is guarded in its owning
module.

Note `PartFault` already carries a typed `cause` on its
`PartRootFailed` arm for the chaining case, so the door is not
uniformly prose — this arm is the odd one out, which makes a consumer
that learned to branch on one still substring-match the other.

`crates/editor-core/src/eval/parts.rs` is DOCM's territory glob, which
is why this is filed here rather than taken by FIX.

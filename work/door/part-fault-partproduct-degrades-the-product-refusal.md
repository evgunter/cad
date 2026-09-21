---
id: part-fault-partproduct-degrades-the-product-refusal
kind: issue
title: PartFault::PartProduct degrades a ProductError to message: String — trivially fixable now that ProductErrorKind exists
status: closed
opened: 2026-09-11
priority: P1
cost: E
branch: door/part-fault-carries-the-kind
pr: 2986
closed: 2026-09-21
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

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/door/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the fix is written in the row and it is one PR on a file another program owns, which is DOOR's test. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Closed (2026-09-21) — PR 2986

`PartFault::PartProduct` carries `kind: product::ProductErrorKind`
beside `message`, both read off the one error at `product_fault`'s early
return — the only construction of that arm in the tree. The rendered
sentence is byte-identical, so nothing reading a `PartFault`'s prose
moved.

**Two judgement calls, both grounded in the precedents rather than
invented:**

- **The message stays.** `ChecksError::Product` and
  `CheckEvidence::SeparationUnavailable` both keep the sentence beside
  the class, and `checks.rs` says why: the rendered half carries node
  ids, the colliding name and the finding lists a fieldless class drops.
  Replacing the string would have deleted the diagnosis rather than
  typing it.
- **`kind` is NOT optional**, unlike `ChecksError::Product`'s. That
  `Option` encodes *"no resident asked for a subject"* — a state with no
  refusal behind it. At this door there is always an error in hand, so
  an `Option` would have been a field that is never `None` pretending
  otherwise.

**The pin, where there was nothing at all.** Nothing in the tree
constructed or asserted this arm — not in `editor-core`, `pncad`,
`pncad-py`, `viewer` or the demos — so the arm could have carried
anything. `asm2a_instantiate::a_gather_refusal_crosses_as_its_class_beside_its_sentence`
instantiates two part documents that refuse the gather differently (a
body-less one, `NoBodyRoots`; one whose only root is poisoned through a
failed ancestor, `RootPoisoned`) and asserts both arrive as
`PartProduct` with **different** classes, that `means_no_body` answers
differently for them, and that the gather's own sentence still travels.
Mutation-checked: hard-coding the call site to `NoBodyRoots` reds the
poisoned case.

**The pin's home is another program's suite, and that is the right
call.** `parts.rs` has no `#[cfg(test)]` module and the row needs the
stub resolver and two instantiated part documents, so it sits beside its
sibling arms' pins in `crates/editor-core/tests/asm2a_instantiate.rs`
(S-TCOST's and S-TINT's). It **adds** a case and removes none, which is
the line this wave already drew: narrowing another program's coverage is
not a side effect a unit gets to have; adding to it, announced, is
ordinary.

**Python:** the type crosses as a tag only. `part_fault_tag` already
matches `{ .. }`, nothing projects `PartFault`'s payload, and
`ProductErrorKind` is already bound. No stub line, attribute or tag
moves — disclosed, and correctly **no row filed**, because nothing there
needs a decision.

**Filed:** no new row. The sibling defect one arm over —
`PartRootFailed`'s `message: failure.kind.to_string()` — is already
EDIT's `D366`, and the lane appended the evidence that makes that row
concrete: it **cannot** be fixed the way this one was, because
`PartFault` derives `Clone + PartialEq + Eq` while `NodeErrorKind`
derives `Debug` alone, so there is nothing cloneable to carry until
D366's projection exists. That is the live consumer saying what the
projection buys.

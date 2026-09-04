---
id: two-refusals-carry-no-recourse-sentence
kind: issue
title: MateFault::Contradictory and AssemblyError::NoAtRestRecord carry no recourse sentence
status: open
opened: 2026-09-03
---


Finding 2 of `pin-mismatch-recourse-emitted-twice` (GitHub 947, from the
ASM-DEMO exit walk), carried out of that file when LIB-MECH1 took finding
1 and closed it. Not taken there because authoring a recourse sentence
into a kernel crate's diagnostic is a judgment call, not a mechanical
repair — the words are the deliverable.

## The finding, as filed

The demo's refusal walk prints four typed refusals. Two of them tell the
author what to DO:

* `MateFault::Under` ends on `UNDER_RECOURSE` ("add the complementary
  mate, or delete the mate if free relative motion was intended").
* `WorkspaceError::PinMismatch` ends on `PIN_MISMATCH_RECOURSE`.

Two do not:

* **`MateFault::Contradictory`** — "mates 3 and 5 cannot both hold:
  predicate `mate_member_translation_zero` measured a clash of
  0.010000000000000009 m where their cosets would have had to meet".
  Names both mates and the measured clash, which is the diagnosis; says
  nothing about the repair (delete one of the two, or re-author the one
  whose datum is wrong).
* **`AssemblyError::NoAtRestRecord`** — quotes the class table's reason
  (a tangency's record is a `CurveContact` keyed by a witness edge, and
  an assembly at rest has none) and ends "the record is not minted with
  an invented witness". That explains the refusal; it does not tell the
  author that the way to declare this contact today is a `Rest`, or that
  curved contact verification at rest is R3/M9 work.

Both are honest and both name their subject, so this is polish rather
than a defect — but the ASM ladder's own exit criterion is "everything
outside v1 refuses typed **with recourse text naming its rung**", and
for these two the rung is not named.

## What taking it involves

The two recourse constants and their prose, sited beside
`UNDER_RECOURSE` and `PIN_MISMATCH_RECOURSE`; the demo's refusal walk
reads all four out loud, so whatever is written is read by a user on the
next run. Check for pinned message text on both arms before wording them
— and note `crates/pncad-py/src/errors.rs`'s `reads_as_prose` rule
applies to anything these arms render.

## Home

S-MATE's `keep_out` assigns the refusal-display prose to LIB, whose
charter carries the library's user-facing surface — the same routing the
parent issue carried.

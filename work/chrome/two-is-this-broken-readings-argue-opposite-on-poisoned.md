---
id: two-is-this-broken-readings-argue-opposite-on-poisoned
kind: issue
title: bounds::Verdict and tree::has_faults argue opposite rules for Poisoned, one enum apart, neither citing the other
status: open
opened: 2026-09-22
priority: P3
cost: E
---


## Finding

Two functions in `crates/viewer/src` answer *is this document broken*
over the same evaluation, one enum apart, and each argues the
`Poisoned` case at length at its own site without citing the other.

- `bounds.rs`, `Verdict` — *"Failed only, never poisoned: a poisoned
  node is the recorded consequence of an ancestor's failure, so
  counting it would make one failure register as many and make the
  verdict depend on how deep the recipe happens to be below the
  break."* `Verdict::of` filters `NodeResult::Failed(_)`.
- `tree.rs`, `has_faults` — *"[`RowStatus::Poisoned`] is a fault. A
  poisoned row's failure is someone else's, but the document it
  belongs to is no more building for that."*

A reader who has just been convinced by one meets the opposite rule,
argued as confidently, in the next module.

## A documentation defect, not a behaviour one — stated precisely

Both readings are right for their own question, which is exactly why
neither site noticed it had a counterpart:

- a verdict is a SET whose SIZE decides whether a value got worse, so
  a poisoned node inflates it;
- `has_faults` is a boolean, which nothing inflates.

And on a well-formed evaluation they cannot disagree: a poisoned node
has a failed ancestor in the same run, so an empty `Verdict` implies
no poisoning either. **Where they can actually differ is one state**:
a poisoned row whose chain does NOT end at a failure, which
`tree::poisoned_through` renders as `RowStatus::Poisoned { message:
None }` and which `crates/viewer/tests/tree_badges.rs`'s
`a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on` now
builds. There `has_faults` says not-building and a verdict over the
same run is empty.

## What a taker owes

One sentence at each site naming the other and the question it
answers, so the two rules read as two questions rather than as a
contradiction. `tree::has_faults` carries its half already (added by
`chrome/rowstatus-exhaustive`, citing this row); `bounds::Verdict`
does not.

## Filed from outside the fence

Found by the style review of PR 3055 (`chrome/rowstatus-exhaustive`),
whose fence was `tree.rs` and `crates/viewer/tests/`. `bounds.rs` is
claimed by CHROME jointly with AUTHOR, VGEOM and VIEW; filed on CHROME
because the subject is what the chrome calls broken.

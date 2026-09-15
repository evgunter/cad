---
id: subject-refused-accepts-the-one-refusal-that-must-not-go-through-it
kind: issue
title: Subject::refused accepts NoBodyRoots, the one refusal the registry must not treat as unavailable
status: open
opened: 2026-09-15
---


Found by WIRE's `nobodyroots-classification-has-two-homes` lane while
giving the empty-document reading one home on `ProductError`.

## The door and the arm it must not take

`checks::Subject::refused` is `pub` and takes any `&ProductError`:

```rust
pub fn refused(source: &product::ProductError) -> Self {
    Self::Unavailable { kind: Some(source.kind()), reason: source.to_string() }
}
```

`Subject::Unavailable`'s own doc says a subject-reading resident that
is enabled and meets this arm makes the door refuse
`ChecksError::Product`. So handing `ProductError::NoBodyRoots` to
`refused` turns a merely EMPTY document into a checks refusal — the
exact outcome `Subject::NoBodyRoots` exists to prevent, and the one the
classification now stated on `ProductErrorKind::is_empty_document`
rules out.

Every caller in this tree routes correctly: `run_checks`,
`viewer::session`'s landing and `pncad_py::product_memo::checks_report`
each test the arm first and reach `Subject::NoBodyRoots`. Three callers
getting it right by hand is what a door is for.

## Why it is a finding and not a style note

- **Nothing states the precondition.** `refused`'s doc argues only that
  `kind` and `reason` come off ONE error; it does not say which errors
  may be handed to it. A caller reading the door learns the pairing
  invariant and not the routing one.
- **The state is constructible and is already written down.** A
  `ChecksError::Product { kind: Some(ProductErrorKind::NoBodyRoots) }`
  literal is pinned as test data in `crates/pncad-py/src/tests.rs`
  (`check_registry_tags_are_stable`) — a value the registry cannot
  produce today, standing in a test as though it could.
- **The fix is cheap now and was not before.**
  `ProductErrorKind::is_empty_document` (`crates/editor-core/src/product.rs`)
  is the predicate `refused` would assert on, so the guard is one line
  and the sentence it would state already has a home to cite.

## What the shape of the fix is NOT settled

Three readings, and this row does not pick one: (a) `refused` refuses —
returns `Subject::NoBodyRoots` itself for the empty-document arm, which
makes the routing a property of the door and deletes it from all three
callers; (b) `refused` `debug_assert!`s the precondition and documents
it; (c) the door keeps its posture and only its doc states the rule.
(a) is the one that removes the hand-routing, and it is FIX's call
because `Subject`'s three-arms-are-three-facts contract is stated in
this file.

Signed: (WIRE implementer lane `wire-n1`)

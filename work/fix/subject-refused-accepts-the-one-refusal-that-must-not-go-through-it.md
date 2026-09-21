---
id: subject-refused-accepts-the-one-refusal-that-must-not-go-through-it
kind: issue
title: Subject::refused accepts NoBodyRoots, the one refusal the registry must not treat as unavailable
status: review
opened: 2026-09-15
branch: fix/subject-refused-routes-no-body
pr: 2943
priority: P0
cost: E
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
classification now stated on `ProductErrorKind::means_no_body`
rules out.

**The callers, re-derived with `git grep 'Subject::refused' -- crates/`**
— an earlier draft of this row named three from memory and got all
three wrong, so the list is the grep's:

- `crates/editor-core/src/checks.rs`, `run_checks` — production. Tests
  the arm first and reaches `Subject::NoBodyRoots`.
- `crates/pncad-py/src/product_memo.rs`, `checks_report` — production.
  Same, written again for the memoized gather.
- `crates/editor-core/src/checks.rs`, in this module's own tests.
- `crates/editor-core/tests/docm5_subject.rs`.

So **two production callers, both routing correctly by hand, and two
test callers**. `viewer::session`'s landing is NOT one of them: its
`means_no_body` test decides whether to run the checks at all, not
which `Subject` to build.

## Why it is a finding and not a style note

- **Nothing states the precondition.** `refused`'s doc argues only that
  `kind` and `reason` come off ONE error; it does not say which errors
  may be handed to it. A caller reading the door learns the pairing
  invariant and not the routing one.
- **The state is constructible.** Nothing in the type system or in
  `refused`'s own contract keeps `NoBodyRoots` out of
  `Subject::Unavailable`; only the two production callers' hand-routing
  does.
- **The fix is cheap now and was not before.**
  `ProductErrorKind::means_no_body` (`crates/editor-core/src/product.rs`)
  is the predicate `refused` would assert on, so the guard is one line
  and the sentence it would state already has a home to cite.

## What the shape of the fix is NOT settled

Three readings, and this row does not pick one: (a) `refused` refuses —
returns `Subject::NoBodyRoots` itself for the no-body arm, which makes
the routing a property of the door and deletes it from **both**
production callers; (b) `refused` `debug_assert!`s the precondition and documents
it; (c) the door keeps its posture and only its doc states the rule.
(a) is the one that removes the hand-routing, and it is FIX's call
because `Subject`'s three-arms-are-three-facts contract is stated in
this file.

Signed: (WIRE implementer lane `wire-n1`)

## Ruled (FIX orchestrator, 2026-09-20): reading (a), `refused` refuses

This row named three readings and declined to pick one. The seat picks
**(a)**: `Subject::refused` returns `Subject::NoBodyRoots` for the
no-body arm itself, and the hand-routing comes out of **both**
production callers.

**Why this is FIX's to rule and not a re-homing.** Ev ruled in chat on
2026-09-20 that this program takes no design decisions — a row whose
blocking question is a design decision goes to the track owning the
surface. Here FIX **is** that track: `crates/editor-core/src/checks.rs`
is FIX's by `paths` and territory names no other owner, `Subject`'s
three-arms-are-three-facts contract is stated in that file, and no
README or design page in the companion table states it (checked:
`Subject::refused` appears in no `docs/` or `crates/*/README.md` text).
So this is a door decision on this program's own code, which is what
the seat is for, and it leaves a written fix rather than an open
question.

**The argument.** (b) and (c) both leave the defect constructible and
only describe it: a `debug_assert!` is absent in release, and a doc
sentence is the thing that was already missing. (a) is the only reading
that makes the routing a property of the door, and the predicate it
asserts on already has one home —
`product::ProductErrorKind::means_no_body`, given it by WIRE's
`nobodyroots-classification-has-two-homes` (PR 2629). A door that
routes on that predicate cites the classification instead of
re-deriving it, which is the same move the four consumers WIRE
converted already made.

**What the lane still establishes rather than assumes.**

- `refused` is `pub`. Its callers are the four this row's grep names,
  two production and two test — re-derive with
  `git grep 'Subject::refused' -- crates/` at your merge base, because
  an earlier draft of this row named three callers from memory and got
  all three wrong.
- Whether either production caller's existing arm becomes dead once the
  door routes, and delete it if so — leaving both the guard and the
  hand-routing is the half-fix shape.
- The pin: a row that goes red if `refused(&NoBodyRoots)` ever yields
  `Unavailable` again. Instruction 3 applies — say whether any existing
  row discriminates the old routing from the new one, and if none does,
  that is part of the defect.

---
id: three-emission-bugs-do-not-speak-the-framing-written-once-for-them
kind: issue
title: Three NamingError emission bugs do not speak EMISSION_FRAMING, the sentence written once for that category
status: closed
opened: 2026-09-15
priority: P1
cost: E
branch: emit/emit-rs-drivebys
closed: 2026-09-23
pr: 3099
---


## What

`crates/editor-core/src/names/emit.rs` declares `EMISSION_FRAMING` as
**"the one sentence every emission-inconsistency refusal opens with",
written once**, with the argument that *"a reworded copy would let
refusals of one category read as several"*.

Three variants of that category do not speak it: `NamingError::Duplicate`,
`NamingError::Unnamed` and `NamingError::MissingUpstream` each render a
bespoke sentence of their own. Three do speak it (`Emission`,
`SplitLineage`, `FragmentLineage`).

So the constant's own justification is only two-thirds true of its
category, and a reader cannot tell an emission bug from the opening
clause — which is exactly the failure the constant exists to prevent.

## Why it matters

It is not a wrong classification: all six ARE bugs and every sentence
says something true. What is missing is the property the constant
claims, and the claim is load-bearing now that a second framing
(`UNRULED_FRAMING`, for the missing-rule category) sits beside it: with
two framings in the file, "which framing a refusal opens with" became a
reader's shortcut for "which category", and that shortcut is valid for
one category and not the other.

Two ways out, and the choice is the owner's: give the three the shared
framing (user-visible text moves — `Duplicate`'s and `Unnamed`'s
sentences are asserted in `display_tests` and reach Python through
`naming_error_tag`'s carrier), or retire the "every … opens with"
sentence from the constant's doc and say what it actually covers.

## Found by

`two-emitter-refusals-a-legal-declared-union-reaches` (lane `wire-e2`),
review round 2. The unit added the second framing and its
variant-to-framing test, which is what made the gap visible; closing it
is a decision about three refusals the unit does not touch.

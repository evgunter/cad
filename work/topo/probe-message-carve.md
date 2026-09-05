---
id: probe-message-carve
kind: issue
title: review_d18_probes carves an unreachable! message with a line window and the first ')', not a balanced read
status: open
opened: 2026-09-05
refs: [D261]
---

## What

`crates/topo/src/review_d18_probes.rs:276-292`
(`d18_no_unreachable_message_can_impersonate_the_postcondition`) now
reads `test_utils::source::code_and_literals`, so a commented-out
`unreachable!` is no longer a site and prose after code on the same
line is no longer one either. **The carve of the MESSAGE inside the
call is still textual**: four lines joined, split on `unreachable!(`,
then everything up to the first `)`.

Two ways that is wrong, both in the silent direction:

- a message containing a `)` — `unreachable!("mev(: the proof
  outlived its key")` — truncates at that paren, so a `postcondition`
  after it is never read and the offender is missed;
- a call whose message is wrapped over more than four lines is read
  only as far as the window reaches, and the tail is not scanned.

## Why it was not fixed with the conversion

The shared home's balanced operations state their precondition:
`test_utils::source::balanced_end` is correct only over a view that
drops literals AND comments, because that is what makes every bracket
a real bracket. This guard needs the literal — the message IS the
thing being judged — so it cannot use `balanced_end` over its own
view, and `D261` did not mint a fourth operation to close it.

The general shape is worth naming: **the class of guards whose needle
is a literal has no balanced-carve operation in the shared home**, only
the blanked-view one. A guard wanting the ARGUMENTS of a macro
invocation whose payload is a literal has to locate the brackets over
`code_only` (where they are real) and then read the same byte range out
of `code_and_literals` — two views over one text, at the same offsets,
which the lexer's offset-preserving contract already makes possible.
That is the operation, and it belongs beside `balanced_end` rather than
in this guard.

## Fence

Track P — `topo`'s Euler surgery, liveness and the generator, plus
the review-and-fixture readers (`review_d18_probes.rs` among them).
The shared-home half is Track W / `tcost`'s.

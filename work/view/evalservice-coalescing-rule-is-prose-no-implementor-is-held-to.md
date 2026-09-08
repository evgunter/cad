---
id: evalservice-coalescing-rule-is-prose-no-implementor-is-held-to
kind: issue
title: EvalService's at-most-one-outstanding rule lives in module prose and no trait clause or row holds an implementor to it
status: open
opened: 2026-09-06
---


## What

Split out of
`work/view/the-unreachable-eighth-combination-lost-its-only-assertion.md`
when that closed on #2055: the assertion it asked for is restored, but
the contract underneath it is still unenforced.

`DocSession::outstanding` reads `Outstanding::Current` for
`!busy() && running()`, and the reason that combination is unreachable
has two mechanisms. The first is local and checkable —
`DocSession::request_eval` bumps the generation on every submit
(`crates/viewer/src/session.rs:1888`), so `!busy()` means the newest
generation submitted is the one that landed. The second is not:

> at most one request is ever outstanding, and a submit while one is
> outstanding REPLACES the waiting request rather than adding to it
> — `crates/viewer/src/evalseam.rs:34-38`

That is module prose about the two shipped implementations. The
session holds `Box<dyn EvalService>` (`crates/viewer/src/session.rs:177`),
so the claim is about every implementor, and:

- `EvalService`'s own doc comments
  (`crates/viewer/src/evalseam.rs:192-207`) say `submit` "cancels an
  in-flight run for an older generation and replaces any queued one",
  which is the rule — but nothing checks it, and a trait comment is
  not a check.
- **An in-tree implementor already departs from it.**
  `HeldEvaluator` (`crates/viewer/tests/review_gui2_r2.rs:1272-1288`)
  queues submits instead of replacing them and makes `cancel` a no-op.
- No row drives an arbitrary implementor against the rule; the row
  that pins the coalescing behaviour drives the two shipped seams.

## Why it is filed rather than fixed

A conformance row over `EvalService` implementors is a test-shaped
piece of work with a design question in it (what a shared row can
assert about a seam it does not own, and whether `HeldEvaluator` is a
violation to fix or a deliberate stub to exempt). #2055's fix pass
handled the coverage gap it created and did not take this.

The consequence today is bounded: the unreachability is a statement
about the shipped seams and `outstanding()` is total either way, so
nothing is unsound — an off-contract seam gets `Current` while
claiming work, which is now an executed row
(`a_current_picture_reads_current_even_when_the_seam_claims_work`,
`crates/viewer/tests/eval_seam.rs`) rather than an unstated case.

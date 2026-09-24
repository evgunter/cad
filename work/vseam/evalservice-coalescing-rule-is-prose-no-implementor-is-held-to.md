---
id: evalservice-coalescing-rule-is-prose-no-implementor-is-held-to
kind: issue
title: EvalService's at-most-one-outstanding rule lives in module prose and no trait clause or row holds an implementor to it
status: open
opened: 2026-09-06
priority: P1
cost: D
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
(`crates/viewer/src/session.rs:2015`), so `!busy()` means the newest
generation submitted is the one that landed. The second is not:

> at most one request is ever outstanding, and a submit while one is
> outstanding REPLACES the waiting request rather than adding to it
> — `crates/viewer/src/evalseam.rs:34-38`

That is module prose about the two shipped implementations. The
session holds `Box<dyn EvalService>` (`DocSession::eval`,
`crates/viewer/src/session.rs:224`),
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

## The same exposure now exists on `IndexService`, at a new site (2026-09-21, `view/seam-residue`)

Evidence added rather than a second row opened, because it is this
row's shape at a second trait.

`PickCache::indexing` stopped reading `IndexService::busy` and is now
the cache's own record alone (`crates/viewer/src/pickcache.rs`,
`PickCache::indexing`). The deletion's argument is that a standing
`Attempt::Asked(k)` implies the seam holds `k`, and that is a property
of the two SHIPPED implementations — `ThreadIndexer`'s `Coalescing`
and `InlineIndexer`'s `pending` slot — where `PickCache::new` takes a
`Box<dyn IndexService>` and `IndexService` is public. So the same
sentence this row makes about `EvalService` now holds at
`pickcache.rs`: the trait's doc states the rule (`IndexService::submit`
— *"A build already in flight runs to completion and its answer is
dropped; a request only WAITING is replaced"*) and nothing checks an
arbitrary implementor against it.

The consequence is bounded the same way: an off-contract indexer that
went idle under an unanswered attempt would light `indexing…` with
nobody answering, which is the #2637 symptom — not unsoundness, and
not a state any shipped seam can be in, because a seam that loses its
worker now panics (`Coalescing::crashed`). A conformance row over
`IndexService` implementors is the same test-shaped work with the same
design question in it, and `CountingIndexer`
(`crates/viewer/tests/frame_policy.rs`) is a delegating wrapper rather
than a second machine, so the in-tree population that would have to be
exempted is smaller here than `HeldEvaluator` makes it there.

---
id: door-zero-asks-only-the-first-attempt
kind: issue
title: door_zero asks only the first attempt's door form, so a registration contradicted only at a retry goes uncounted
status: open
opened: 2026-10-13
priority: P3
---



## The gap (DECIDE-10 delta review, NOTE-4; predates PR #4929)

Under a definite numeric sign, `Decide for Sym<T>` asks `door_zero`
(`crates/geom-core/src/sym.rs:5237`) whether the zero rests on a
registration, and counts `registrations_contradicted` if so.
`door_zero` (`sym.rs:4483`) asks two things:

- `door_form(sess, id)` (`sym.rs:4040`), which walks the door at
  attempt 0 under the session's first rules and
  `rational::COEFF_BITS`;
- `ladder(sess, id, Path::Contradiction)`, which returns before the
  retry loop (`sym.rs:4393-4395`).

Neither asks a retry. The definite-sign path never runs the decision
ladder, so no retry attempt is walked for that node either. The retries
are `SymRetry`'s wider ring and its kept-atom masks. A registration
whose door form reaches zero only at one of them is therefore never
charged, even though a contradicted axiom is supposed to be counted in
release. The decision itself is unharmed, because the numeric answer
stands and nothing is folded. What is lost is the receipt column.

The dispute column has the same shape: `contradicts` asks the same
first-attempt ladder, so a theorem that closes only at a retry under a
definite sign is not counted as `theorems_disputed` either. The two
columns stay disjoint, because they ask one ladder.

## Reach

Only sessions that install a retry ladder and are not deferring:
`with_session_memo_retry` and `with_session_retry` in
`crates/editor-core/src/drive.rs:1629,2034` and
`crates/editor-core/src/eval/mod.rs:3013`. A deferring session (the
coincidence replay) returns from `decide_sign` before the definite-sign
check (`sym.rs`'s `deferred::defer`), so it never asks `door_zero`. Only a registration
that is false over the box, and whose door form the first attempt
cannot close (ring refused, or an atom the first rules leave opaque).
No case is shown; the review found it by inspection. It is P3 as a
receipt that can stay silent, not a wrong answer.

## What a fix would have to do

Have the contradiction path walk the same attempts the decision path
would, at least for the door, and count a zero at any attempt. Price
it first: a retry is paid today only where every first-attempt rung
failed, and the definite-sign path would pay it at every definite
margin that has a registration in its session. Doing it only where the
registry is non-empty bounds that to documents with arcs.

---
id: a-value-with-no-expression-minted-outside-a-session-is-named-per-thread
kind: issue
title: A value with no expression minted outside a session is named by a per-thread count, so two threads' values carried into one session can share a name
status: open
opened: 2026-11-22
---



`Sym::opaque` names a value by the mint scope open on its thread and
its ordinal there (`geom_core::sym::mint`). Under no opened scope and
outside a session the scope is the thread's root, whose count runs for
the life of the thread: two threads each mint a root name `k`, for two
different numbers. Nothing is proved outside a session, so no wrong
answer is made there; the exposure is a `Sym` built outside any
session on two threads and then carried into one session, where the
two share an id and their difference is the zero polynomial. The same
held of the per-thread sequence this replaced.

**Not executed, and no caller is known**: the evaluation service opens
a scope per node, `k_stats::map_detached` one per unit, and a drive
replays inside a session. Found by the node-key unit's sweep for names
minted from a counter
(`work/boxfinish/a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression.md`).
Closing it would take a root name that no other thread can mint (a
process-wide count costs the name its determinism; a refusal to mint
under no scope costs every direct kernel caller a scope).

**What the node-key PR closes and leaves** (PR 7470), against
`an-opaque-minted-before-a-session-shares-a-name-with-the-sessions-first`:

- Closed: a value minted under no scope before a session and one
  minted under no scope inside it. A session's root scope has a name of
  its own (`sym::mint`'s `SESSION_ROOT`), pinned by `geom-core`'s
  `a_value_minted_before_a_session_and_the_sessions_first_do_not_cancel`.
- Left: two threads' root names outside any session (above); two
  sessions' root names, when a root-scope value of one session is
  carried into another (each session's root counts from zero); and one
  opened scope's names outside a session against the same scope's
  inside one, when a value built by an evaluation under no session is
  carried into a session that evaluates the same node. No caller of
  any of the three is known. A name unique across sessions would cost
  the root's names their determinism, and decisions are taken over
  root-scope values (the reads after an evaluation: 53,065 such mints
  over `editor-core`'s suite).

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

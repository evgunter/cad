---
id: a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state
kind: unit
title: A node's result at the symbolic scalar reads per-replay session state: the expression table, the registry and the retry cap belong to a replay, not to a node or the drive
status: review
opened: 2026-11-22
priority: P1
cost: H
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/session-state
pr: 7492
---


The last two of the four changes in point 2 of the parent row's
weighed state (`[ev]` PR 7368): the expression table belongs to the
drive, and there is no per-replay session state a node can read. Each
piece is either a cache whose misses never change an answer, or
belongs to a node and travels with its value (the identities it
registered, its counts, its deferred entries); the registry is scoped
to a node's inputs, and the retry ladder's per-replay cap
(`SymRetry::max_forms`) becomes per node.

Measured (M9): 23.0 s of the tour's 177.6 s of leaf time is the
session's per-replay teardown (publishing the leaf's forms to the
drive's memo, the `frozen` column, dropping the tables), which a
drive-scoped table pays once. Untested by any measurement: a drive in
which the retry ladder runs or an identity is registered.

**Sequenced after the decide path's unit** (SYM's row, in flight on
`boxfinish/decide-path-check`), which removes four of the session's
count columns and two of its walks: this unit moves the same counts
onto a node's value, so it stacks on that branch once its PR is open.
After `a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`.

**Letter H; dual tier, a concurrent pair** (the symbolic session is
read by every decision at a symbolic scalar; a piece of state moved to
the wrong owner changes an answer without a red row, and the two
units after this rest on a node's result depending on its key alone).

**Dispatched 2026-11-22, stacked on `boxfinish/decide-path-check`**
(PR 7394, in review), which removes four of the session's count
columns, its `free` table and the walks after a definite sign: this
unit moves what is left. The lane merges `boxfinish/node-key` when
that PR opens (it gives a value with no expression a name scoped to
its minting node, the scope this unit extends), and reports a census
of the session's state and its plan before it builds.

**The split rule rests on this unit** (split-reads lane, 2026-11-22).
`drive::bisect` splits only what the deciding node reads
(`Doc::free_reads`), and a deciding node that reads nothing of the box
refuses `ReadsNoVariable`: "no split changes it". That is exact when a
node's result is a function of its inputs. While a node can read
per-replay session state (the retry ladder's per-replay cap, the
registry), an earlier node that reads another variable can move that
state, and a split of that variable could change what the deciding node
says. Not observed: no retry runs in the drives measured, and the
corpus differential compares content keys, which do not see the session.

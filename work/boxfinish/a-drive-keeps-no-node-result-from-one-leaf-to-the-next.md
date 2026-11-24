---
id: a-drive-keeps-no-node-result-from-one-leaf-to-the-next
kind: unit
title: A drive keeps no node result from one leaf to the next: one store per drive under the evaluator's memo rule, and receipts that count what a result rests on
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
---


Points 1, 3, 5 and 6 of the parent row's weighed state (`[ev]` PR
7368; `docs/ERROR-DESIGN.md` E6's leaf-protocol bullet): a node's
result (build, finish or refusal, with what it recorded) is served
from one store per drive under (the node's own id, the content key,
the naming key), each entry computed once; a leaf's receipt counts
what its result rests on, served or built; the drive's total counts
each distinct node result once, so the pinned totals of multi-leaf
drives re-baseline once; the store is a dial like `plain_memo`, and
verdicts, leaf receipts and the drive's total are the same with it on
or off and on one thread or many. A deferring session takes no prior.

Measured (M8, M9): on the tour 87% of build and 91% of finish time
inside node ops is on a key already seen, most of it the holes'
refusals, and every repeat of a key carries a byte-identical refusal
and count delta.

**Open inside the unit, to measure and not to ask:** whether handing
a leaf its parent box's replay is enough (no shared state). It needs a
document with two features that both refine, under the new split
rule; if cousin-only repeats are rare there, the parent-only prior is
the simpler final state (Ev was told on PR 7368 it would be taken and
logged).

After `a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`.

**From the node-key unit's fix pass (PR 7470), two things the store
should know:**

- **A name is a function of the document only inside a session.** A
  value with no expression is named by its node's mint scope and
  ordinal inside a session, and by its minting thread outside one
  (`geom_core::sym::mint`). A walk that maps its units in parallel
  where decisions are thread-portable and runs them in order under a
  session (`topo::props`' face walk, the evaluator's level map) so
  names the same work two ways (`geom-core`'s
  `the_two_arms_of_a_walk_share_no_name`). A drive replays every leaf
  inside a session, on the serial arm, so every entry of one store is
  named one way; a store that ever took an entry built under no
  session would hold names no replay reproduces.
- **A held axis keys apart from the unboxed build at `Sym`.**
  `analysis::var_env_over` binds a `Fixed` axis `nominal + 0` and the
  unboxed environment binds `nominal`: one number, two expressions, so
  two content keys now that the key hashes the expression. The `+ 0`
  is the arithmetic a varying axis binds through (`nominal + offset`),
  kept so that a held axis and a degenerate varying one are the same
  bits, a negative zero included; binding a held axis as the bare
  literal at `Sym` alone would make the symbolic lane's numeric channel
  differ from the plain lane's there. Every leaf of a drive goes
  through `var_env_over`, so inside one store a held axis is one
  expression and nothing misses; the miss is between an unboxed build
  and a boxed replay, which no store shares.

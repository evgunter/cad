---
id: a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression
kind: unit
title: A node's content key at the symbolic scalar hashes the number and not the expression, and a value with no expression is named by a per-replay counter
status: closed
opened: 2026-11-22
priority: P1
cost: M
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/node-key
pr: 7470
closed: 2026-11-24
---


The first two of the four changes in point 2 of the parent row's
weighed state (`[ev]` PR 7368): the content key hashes the expression
as well as the number (`ContentBits for Sym` in
`crates/editor-core/src/eval/memo.rs` feeds the value only), and a
value with no expression is named by the node that minted it and its
ordinal there, not by a per-replay counter (two replays can give one
name to two numbers, a false zero once values cross replays).

Latent today: every evaluation at a symbolic scalar passes no prior
(`drive::classify_leaf`), and M8 found no two replays holding a node
with equal keys and different expressions, and no built result holding
a value without an expression. The statement "the key is the
correctness proof" (DESIGN `[determinism]`) is false at the symbolic
scalar until this lands, and the two units after it rest on it.

**Letter M; single FULL review** (a soundness statement about a key,
with claims a reader has to try to break: what else a node's result
reads that the key does not hash).

## Closed

PR 7470 (2026-11-24). At the symbolic scalar a node's content key
hashes each value's number and then its expression's node id
(`ContentBits for Sym`), so two spellings of one number key apart and a
prior over the other is not served. A value with no expression is named
where it was minted (`geom_core::sym::mint`): inside a session by the
open mint scope and its ordinal there, the evaluator opening one scope
per document node and one per member inside it, with the session
keeping a closed scope's count; outside a session by the minting
thread's token and its never-restarting count. So a node's k-th mint
has the same name in every replay, and no two values share a name
within a session or outside one.

Kept by design, and said in the module: two sessions give one scope's
k-th mint one name, which is what makes the names the same in every
replay; a value reaches another session only as a node's result served
under its content key, which hashes everything the node read.

What it left: a name given outside a session is not reproducible run
to run (nothing reads one into a key); `nominal + 0` and `nominal` are
two spellings at `Sym`, so an unboxed build and a boxed replay key
apart (its cost to a store is on
`a-drive-keeps-no-node-result-from-one-leaf-to-the-next`); no row
covers two members minting as two scopes, since no fixture's members
mint (the same row).

---
id: a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression
kind: unit
title: A node's content key at the symbolic scalar hashes the number and not the expression, and a value with no expression is named by a per-replay counter
status: review
opened: 2026-11-22
priority: P1
cost: M
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/node-key
pr: 7470
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

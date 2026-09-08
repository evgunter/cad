---
id: node-value-kind-answers-a-transform-by-node-kind
kind: issue
title: eval::node_value_kind answers "body" for every Transform, but a transform's value family is its input's
status: review
opened: 2026-09-08
refs: [transform-refuses-a-patterns-instances-value]
pr: 2195
branch: eval/11-transform-value-kind
---

`crates/editor-core/src/eval/mod.rs`'s `node_value_kind` is the
recipe-side reading of `ValuePayload::kind_name` — the word a refusal
uses on the one road that re-derives a node from its expressions and
never holds its value (the mate solve's derived offset, refusing a
circular rule's `axis` operand: `crates/editor-core/src/mate/member.rs`,
`axis_datum`). It matches on node kind alone and answers `"body"` for
`Node::Transform`. Since the placers are shape-preserving over the
value (EVAL-6: `Body → Body`, `Instances → Instances`), a transform's
family is its INPUT's: a transform of a pattern evaluates to
`"instances"`, and the recipe-side word for the same operand is
`"body"`. The two words disagree exactly on that shape, which is what
`msolve3_placer_refused` pins them to agree on for the two families it
reaches (a datum and a body).

Fixing it needs the document — read through the placer chain by node
kind, as CHROME's `denotes_body` will — so the signature grows a
`doc` and node id, and the one caller in `mate/member.rs` (MSOLVE's
file, one line) passes them. EVAL owns the function; the caller edit
is announced to MSOLVE when this lands.

## Closed

`node_value_kind` takes the document beside the node and reads a
`Transform` through its `input` edge to the first non-transform node,
whose family it answers; `Pattern` stays `"instances"`; a dangling
transform input refuses `MissingInput` — the kind the transform's own
evaluation raises, there being no family word for a value that never
lands — so the signature returns `Result`. No cycle guard: the edge is
fixed at insert to a live node and never rewritten, over a recipe
checked acyclic at every edge-adding edit. `axis_datum` in
`mate/member.rs` passes the document (MSOLVE's line, announced in the
PR). `msolve3_placer_refused` gains the transform-of-pattern row (red
on the base: `"body"` against `"instances"`) and the
transform-of-transform-of-body row (both `"body"`). The viewer's
`denotes_body` stays CHROME's
(`body-seat-reads-through-the-placer-chain`).

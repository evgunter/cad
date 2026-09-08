---
id: node-value-kind-answers-a-transform-by-node-kind
kind: issue
title: eval::node_value_kind answers "body" for every Transform, but a transform's value family is its input's
status: spec
opened: 2026-09-08
refs: [transform-refuses-a-patterns-instances-value]
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

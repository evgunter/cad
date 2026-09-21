---
id: python-transform-door-doc-says-an-upstream-body
kind: issue
title: pncad-py's Node.transform rustdoc says 'a rigid placement of an upstream body', but the placers accept Instances
status: open
opened: 2026-09-21
priority: P4
---



(DOOR lane, from the class sweep on
`work/door/node-placer-field-docs-say-body-where-instances-are-accepted.md`.)

`crates/pncad-py/src/py/doc.rs`'s `Node.transform` staticmethod opens
*"A rigid placement of an upstream body."* (~`:2363`). Under the ruling
on `work/eval/transform-refuses-a-patterns-instances-value.md` (PR 2137,
recoverable at `git show 9c515cb1:work/eval/transform-refuses-a-patterns-instances-value.md`)
the placers are shape-preserving over the VALUE: `wire::placeable_operand`
admits a `Body`, a boolean's non-empty result and an `Instances` value
taken whole, so `Node.transform(input=<a pattern>)` evaluates and yields
`Instances`. The kernel-side declaration was repaired by the DOOR row
above (`crates/editor-core/src/node.rs`, `Node::Transform` and
`Node::Pattern`); this is the same sentence at the Python door.

The fix is one doc sentence. Note the two neighbours are already right
and are the model: `crates/pncad-py/pncad.pyi`'s `transform` stub says
"A rigid placement" with no operand narrowing, and `Node.pattern`'s
Python docstring narrows nothing about its input either. What the
sentence should say is what the kernel's now says — one map over every
body the input's value carries, shape-preserving.

Not swept here: `py/doc.rs`'s `Node.pattern` staticmethod doc was read
and carries no body narrowing. The sweep was
`grep -rn "upstream body\|the body placed\|body replicated\|takes one body"`
over `*.rs`/`*.md`/`*.py`/`*.pyi`; it cannot see a sentence that says
"one body" in other words.

- `crates/pncad-py/src/py/doc.rs` — `Node::transform`'s rustdoc, first line

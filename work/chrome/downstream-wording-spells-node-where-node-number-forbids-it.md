---
id: downstream-wording-spells-node-where-node-number-forbids-it
kind: issue
title: tree::downstream_wording writes 'node N' in a surface sentence that node_number's own doc rules out
status: closed
opened: 2026-09-22
closed: 2026-09-22
pr: 3090
branch: chrome/badge-attribution
priority: P3
cost: E
---


## Finding

`crates/viewer/src/tree.rs` holds both of these, twenty lines apart.

`node_number` — *"**How the chrome names a node when there is nothing
more to say**: `feature 3`. One home for the phrase, because it is the
word a person carries between surfaces … and a surface that spelled it
`node 3` would be talking about something a reader has to translate."*

`downstream_wording` — the sentence a poisoned row draws:

```rust
format!("upstream failure at node {} — that row carries the cause", through.0)
```

That is a surface sentence, drawn in the feature tree under the row,
and it spells the thing `node_number` rules out. Two spellings of one
concept in one module, one of them forbidden by the other's doc.

**It is live, not theoretical.** The fixture written for
`a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on` (PR
3055, `crates/viewer/tests/tree_badges.rs`) hand-wrote the expected
message as *"upstream failure at feature 1"* — the author reached for
the vocabulary the module's own rule teaches, and got a string the
code does not produce. Caught in review; the fixture no longer carries
a message at all.

## What a taker owes

Either `downstream_wording` composes `node_number` — *"upstream
failure at feature 3 — that row carries the cause"* — or
`node_number`'s doc gives the reason this one surface is exempt. The
first is a one-line change plus its assertions; `tree_badges.rs` and
`review_gui3_r2.rs` both assert the wording through
`tree::downstream_wording(…)` rather than by literal, so the blast
radius is small and re-baselining is the repair, not a cost.

## Filed from outside the fence

Found by the style review of PR 3055, whose unit was `has_faults`.
`tree.rs` is claimed by CHROME jointly with AUTHOR, VIEW and VNEWS;
filed on CHROME, which owns what the chrome calls things.

## Closed

Fixed in PR 3090 (`chrome/badge-attribution`). `tree::downstream_wording`
composes `tree::node_number`: *"upstream failure at feature 3 — that
row carries the cause"*. `tree_badges.rs`'s
`a_failing_document_renders_failed_and_poisoned_from_the_typed_payloads`
now asserts the pointer contains `node_number(extrude)`, and reverting
the wording reddens it there.

One consequence stated rather than hidden: the row a poisoned row
points at carries the KERNEL's `NodeError` text, which says `node N
failed`, so the pointer and its target now use two words in one pane.
That is `work/author/chrome-calls-one-node-two-names`'s subject (the
feature-vs-node direction is a decision held open there); the
adjacency is recorded on that row as evidence.

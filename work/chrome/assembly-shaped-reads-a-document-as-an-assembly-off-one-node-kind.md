---
id: assembly-shaped-reads-a-document-as-an-assembly-off-one-node-kind
kind: issue
title: session::assembly_shaped decides whether an A5 badge is taken with a matches! over one Node kind
status: closed
opened: 2026-09-24
priority: P3
cost: E
closed: 2026-09-29
branch: chrome/create-residue
pr: 3450
---


## Finding

`crates/viewer/src/session.rs`, `assembly_shaped` (~`:2776`):

```rust
doc.order()
    .iter()
    .any(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
```

Its doc says this *"is what decides whether an A5 badge is taken at
all"*. That is a policy over `Node`, an `editor-core` enum: which node
kinds make a document declare cross-instance rest. It is not identity.
A node kind that instantiates parts some other way would silently leave
the document un-gated, with nothing red. `crates/viewer/README.md`, *A
policy over an enum names every variant*, is the rule, and the fix is
an exhaustive `match` over `Node` (the `combine::denotes_body` shape)
answering *does this node put an instance in the document*.

## Why it is filed rather than fixed

Found at the review of `chrome/subset-policy` (PR 3140). It sits on
`chrome/empty-doc-badge`'s ground, and that lane was in its own fix
pass, so this one goes to whoever next opens the empty-document /
at-rest badge code.

## Closed 2026-09-29 (`chrome/create-residue`, PR 3450)

`session::assembly_shaped` now asks `puts_an_instance` of every node,
an exhaustive `match` over `Node` with a reason per group: only
`InstantiatePart` puts another document's part in; `PlacedUnion`,
`Pattern` and `Part` place this document's own prototype; `Mate` and
`Declare` relate instances and put none. Behaviour unchanged (the same
single variant answers true).

`display::instances_by_root` asks the same question with the same
`matches!`; it is outside this lane's fence and is filed as
`display-reads-an-instance-off-one-node-kind`.

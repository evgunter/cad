---
id: display-reads-an-instance-off-one-node-kind
kind: issue
title: display::instances_by_root asks which nodes are instances with a matches! over one Node kind, beside session's exhaustive answer
status: open
opened: 2026-09-29
priority: P3
cost: E
---

## Finding

`crates/viewer/src/display.rs`, `instances_by_root`, filters a root's
ancestry with
`matches!(doc.node(id), Some(Node::InstantiatePart { .. }))`. This is
the question `session::assembly_shaped` asks: does this node put
another document's part in? Since `chrome/create-residue` (PR 3450),
`session::puts_an_instance` answers it with a `match` naming every
`Node`.

`crates/viewer/README.md`, *A policy over an enum names every variant*,
says a decision asked in several places has one home, and the places
call it. The fix is to make `puts_an_instance` reachable from
`display` (it is private to `session` today) and call it there.

## Why it is filed rather than fixed

Found by `chrome/create-residue`'s sweep for `matches!` over `Node`.
`display.rs` was outside that lane's fence, and OFFER and VNEWS also
claim it.

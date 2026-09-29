---
id: display-reads-an-instance-off-one-node-kind
kind: issue
title: display::instances_by_root asks which nodes are instances with a matches! over one Node kind, beside session's exhaustive answer
status: dispatched
branch: chrome/viewer-small
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

`display::instance_check` asks it too: `Some(Node::InstantiatePart { .. })`
is `Ok`, and `Some(_)` refuses `NotAnInstance`, a wildcard over `Node`.

**Weigh the Identity exemption first.** The README exempts a question
that IS the variant ("is this op an `Open`"). `instances_by_root`'s doc
says display state names *"the thing with an identity a user hides or
probes"*. That may read as "is this node an `InstantiatePart`", which
is identity and correctly `false` for any new kind, rather than "does
this node put a part in", the policy `assembly_shaped` asks. If it is
identity, both sites stay and this row closes saying so; if it is the
policy, both call `puts_an_instance`.

## Why it is filed rather than fixed

Found by `chrome/create-residue`'s sweep for `matches!` over `Node`.
`display.rs` was outside that lane's fence, and OFFER and VNEWS also
claim it.

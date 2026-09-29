---
id: display-reads-an-instance-off-one-node-kind
kind: issue
title: display::instances_by_root asks which nodes are instances with a matches! over one Node kind, beside session's exhaustive answer
status: closed
closed: 2026-09-29
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

## Closed (2026-09-29, `chrome/viewer-small`)

**Ruled identity; the code stays.** Both `display` sites ask "is this
node the instance G3's display state is keyed on", and that is the
`InstantiatePart` variant itself, not `session::puts_an_instance`'s
policy:

- **The state is defined on that node.** Hide and free-move are G3's
  per-instance display state; `mates_naming` scans `Node::Mate`
  references to it, and `instance_check`'s doc names the reason a
  sibling kind is out (a `Pattern` draws several copies, so it has no
  single pose to probe or body to hide). Admitting another kind is a
  change to G3's state, not an arm.
- **`puts_an_instance` answers a different question, for different
  reasons.** Its arms say why a node does or does not put another
  document's part in for the A5 badge ("the rest between them is the
  placement rule's"). The answers coincide today; a new kind that put
  several instances in at once would be `true` there and still have no
  single pose here, so sharing the home would admit it to display
  state on A5's reasoning.
- **A new kind correctly answers no, and loudly.** It is refused at
  the door as `AdmissionFault::NotAnInstance`, a typed refusal, until
  someone designs its display state.

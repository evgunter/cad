---
id: rows-do-not-cross-a-boolean-remap
kind: issue
title: declaration rows do not cross a boolean's key remap (unreachable today: an instance carrying one is multi-solid)
status: open
opened: 2026-09-06
---


Filed from DOCM-6's dual review (PR #2035), which measured a claim the
PR body made too broadly and then measured the true bound.

## The class

DOCM-6 carries a part's mate bookkeeping across the instantiation seam
on the evaluation VALUE — `NodeValue::carried`, filled at the
instantiate op and empty on every other op, beside the records channel
`NodeValue::contacts`. The gather re-keys both through the graft's own
descendant map (`product.rs`'s `carry_contacts` and
`carry_declarations`).

A BOOLEAN does not go through that map. Its result's records are built
by the kernel's own remap (`crates/topo/src/boolean/ops.rs:~1222`,
`remap_contacts`, which chases operand views and the D5 descendant
chain and DROPS a record whose entity was consumed), and its value is
`OpOut::plain` — so `sources_of` reads `norows()` for it
(`crates/editor-core/src/product.rs:~296`). Records can therefore reach
a product through a path that carries no rows.

## Why it is not a live gap today

Two facts, both measured on this head AND on the merge base:

1. **A boolean's own records are not a mate's.**
   `wire_boolean` reads its operands through `body_operand`
   (`crates/editor-core/src/eval/wire.rs:~555`), which takes the BODY
   and nothing else, so a boolean's `BooleanValue::Body::contacts` are
   its own `Declare` node's pairs remapped — pairs no mate authored.
   `Attribution::Unattributed` is the correct answer for one of them.
2. **No instance carrying a declaration can BE a boolean operand.** A
   mate is between two members of the document that authored it, so a
   document with a mate has a product of at least two solids, and the
   pair boolean refuses a multi-solid operand outright. The acceptance
   row `no_carried_declaration_can_reach_a_boolean_operand`
   (`crates/editor-core/tests/docm6_seam_declarations.rs`) pins that at
   three seats — penetrating, resting, gapped.

So the two readings of `Attribution::Unattributed` — "no declaration
THIS PRODUCT holds a row for" and "no declaration in the tree" —
coincide today, and `crates/editor-core/src/assembly.rs`'s
`Attribution::Unattributed` states the narrower one with the bound
beside it.

## What would make it reachable

A boolean over MULTI-SOLID operands. The day the pair boolean (or a
successor: the n-ary union's fold is the obvious one) admits an operand
that is an instantiated sub-assembly with mates, that instance's
records cross into the boolean's result through `remap_contacts` and
its rows do not — and a finding against one of those records goes back
to naming nobody.

Closing it then means re-keying the rows through the same lineage the
records take: `BooleanNaming`'s operand views and graft lineage
(`crates/topo/src/boolean/ops.rs:~188-213`), plus the drop rule
`remap_contacts` applies — a row whose face is consumed by the boolean
has no image and must refuse or drop exactly as its record does.

**Not scheduled, and deliberately not attempted at DOCM-6**: the fix
lives at the boolean's own key remap, which is `crates/topo`, and the
n-ary union's half is DOCM-7's fence.

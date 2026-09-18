---
id: instantiate-part-crossings-are-names-payload-names-does-not-list
kind: issue
title: An InstantiatePart's crossing references are names payload_names does not list
status: open
opened: 2026-09-17
refs: [interface-crossing-heads-are-bare-stable-names, 2814]
---

(Found by the style review of PR 2814, which made an interface
crossing's two references `FaceName`s and so made it plain that they
are names.)

## The finding

`Node::InstantiatePart` is listed in `name_free_node!`
(`crates/editor-core/src/node.rs`, the macro at the top of the file) —
the pattern for "the [`Node`] variants whose payload REFERENCES no
[`StableName`]". It is not name-free. Every `InstantiatePart` carries an
`InterfaceRecord`, and every `InterfaceCrossing::Mate` in it carries
two names: `outer`, a REMAINDER name, and `inner`, a name in the
PART's own id space. Both are `FaceName`s as of PR 2814, so the type
now says out loud what the list denies.

Three consequences follow, each from the one list:

1. **`payload_names`' "single answer" claim is false for this
   variant.** Its doc (`Node::payload_names`, `node.rs`) says it is
   *"The single answer to 'which payloads carry a name': every reader
   reads this rather than its own copy of the list"*. For an
   `InstantiatePart` it answers the empty vector, so a reader that
   trusts it sees a node with no names where there are two per
   crossing.

2. **`Rebind` never reaches a crossing's `outer`.**
   `Node::rebind_payload_names` (`node.rs`) shares `name_free_node!`
   with `payload_names` — the read and the rewrite are one answer read
   two ways — so the `InstantiatePart` arm rewrites nothing. `outer` is
   a remainder name: it denotes a face in THIS document, on a node this
   document can delete or a name this document can rebind, and N5's one
   repair does not reach it. The mate's own heads are rebound (the
   `Node::Mate` arm), so after a rebind the record and the mate it
   records can disagree. (`inner` is a part-side name and correctly
   out of reach — it lives in the part's id space, not this one.)

3. **The insert door's liveness check never sees either reference.**
   `edit.rs`'s `InsertNode` arm loops `node.payload_names()` and
   refuses `DeclareNamesMissingNode` for a name whose node is not live
   — *"the ONLY door that checks, for every payload that carries a
   name"*. `Node::instantiate_part_with` is public, so a record naming
   a dead node inserts unrefused.

The same list is **spelled a second time** in `refactor.rs`'s crossing
walk (`split`, the `for &id in doc.order()` loop that collects
`crossings`), which knows perfectly well that a crossing carries two
names: it classifies each with `derivation_nodes(name)` against the cut
and remaps the part-side one. That is a second, independent answer to
"which names does this node hold" — exactly the shape
`document-stablename-carriers-have-no-enumeration` closed over for the
appearance store, one rung further in.

## Why this row and not the fix

PR 2814's fence was the record's TYPE. Listing `InstantiatePart` as a
name carrier is a behaviour change with three doors behind it — what
`Rebind` does to a crossing it can now reach, what the insert door
refuses, and whether `inner` must be excluded by name rather than by
the variant — and each wants a ruling before code. The record was
inert data until ASM-R2b D-4 inhabited it; it is document data now.

---
id: name-order-was-insertion-order-under-the-counter
kind: issue
title: Where a name's canonical form picks the least name, the counter made that the earliest-inserted node and the mint makes it an arbitrary one
status: open
opened: 2026-09-30
priority: P2
cost: M
design: true
parent: sibling-branches-mint-one-node-id-for-different-nodes
---


## What

A name's canonical form orders the names inside it by `StableName`'s
`Ord`, which compares the minting node's id first. Under the counter,
the lower id was the earlier insert, so every "least name" rule read
as "the node the author placed first". Under the mint
(`crates/editor-core/src/mint.rs`), ids are digests and the least name
is an arbitrary one: deterministic, fixed for one set of ids, and
free of member order, but it no longer means seniority.

The rules that pick by it:
- `names/emit_union.rs`, `Flush`: a flush stretch of several members
  is named for "the least member entity that holds it, in the order of
  the `FromMember` names it publishes";
- `names/seam_pair.rs` and `canonical.rs`: a seam's pair is `a ≤ b`,
  and its chain is ranked along the canonical pair, so which face is
  `a` decides which way the ranks run
  (`seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them`
  is the same orientation seen from another side);
- every name-ordered position (`Borders` walls, a junction's lines).

## Evidence

Measured on `emit/node-id-digest-mint`: every registered corpus
document's name tables, node by node in document order, with every
`RecipeNodeId` and `StepId` blanked, are the same multiset of
`name=entity` rows before and after the mint. So no rank reversed and
no flush stretch changed hands in the corpus. The difference shows
where a member is re-authored: under the counter a re-inserted member
took the highest id, so the older member kept the stretch; under the
mint the re-inserted member's id may sort first, and then a name held
on the stretch, spelled through the older member, goes `Vanished`.
That is loud, not a rebind (a seam's rank direction flips only with
the names that spell the seam), but a person sees a held reference
lost where the counter kept it. This is reasoned from the rules, not
measured.

## The question

Whether "least name" should keep meaning seniority. Options seen so
far, unweighed:
- accept it: the choice is a function of the ids and nothing else,
  and a re-authored member is a new node;
- make seniority data the emitter can read, such as each node's
  position in `Doc::order`, which the naming key would then have to
  carry.

`eval::schedule`, the mate solve's tree edge and the resolve lanes'
evidence order were restated in `Doc::order` on the same branch,
because they have the document in hand. The emitter works from a
node's inputs and does not.

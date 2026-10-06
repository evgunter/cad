---
id: name-order-was-insertion-order-under-the-counter
kind: issue
title: Where a name's canonical form picks the least name, the counter made that the earliest-inserted node and the mint makes it an arbitrary one
status: open
opened: 2026-09-30
priority: P2
cost: M
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

Measured on `emit/node-id-digest-mint` against `origin/main` at
3bc4df6595, with
`crates/editor-core/tests/name_tables_by_position.rs` (its module doc
has the command): every registered corpus document's name tables, node
by node in document order, with every `RecipeNodeId` read as its
position in `Doc::order` and every `StepId` as its profile's position,
loop and step. 29 documents, 359 tables (189 non-empty), 13,420 rows,
819 `FromMember` segments over 21 distinct members:

- 353 tables are identical, row for row;
- 6 tables, all in `nested_islands_105` and `nested_islands_106_*`,
  respell 30 rows, and only the order of the walls inside a
  `Fragment(Borders([...]))`: with every list sorted the tables are
  identical, and each respelled row names the entity it named before;
- no `FromMember`'s member moved, so no flush stretch changed hands,
  and no rank moved.

The walls reorder because `Borders` lists them in name order, which
reads their step ids, and the two trees mint different step ids for
one step: digests on both, over different chains. The corpus has no re-authored member, which is where a stretch
would change hands: under the counter a re-inserted member took the
highest id, so the older member kept the stretch; under the mint the
re-inserted member's id may sort first, and then a name held on the
stretch, spelled through the older member, goes `Vanished`. That is
loud, not a rebind (a seam's rank direction flips only with the names
that spell the seam), but a person sees a held reference lost where the
counter kept it. This is reasoned from the rules, not measured.

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

## Reachability (designer runs, 2026-10-06)

The re-authored-member loss in "Evidence" is not reachable through
today's doors. `DeleteNode` of a live union member is refused
(`DeleteWouldDangle`). Re-declaring a re-drawn member that `SetMembers`
added is refused `DeclaredNameNotUpstream` by `declared_side_fault`
(`node.rs`), which reads document position where it means "an operand
can hold this name". The loss becomes reachable once that refusal reads
membership. What is visible today, with no edit: digest order names a
flush stretch for the later-placed member about half the time.

## Ruled (Ev, PR 4156, 2026-10-06)

Ev approved putting seniority in the id, and removing `Doc::order`. On the
form: "hopefully you can use some custom type instead of doing weird bit
packing stuff directly in the integer field. (i'm fine with a like
(32, 64) for (order, id) also, unless there's a specific need to have it
fit in one integer)". The names README (N1's id clause, the flush
paragraph) states the pair; fork-log row 74 records it.

**What to build:**
- **The id type.** `RecipeNodeId`, `StepId` and `VarId` become a pair
  `(ordinal: u32, digest: u64)`. The ordinal is the mint log's length when
  the id is drawn, plus one, and the digest is the first 64 bits of the
  chain digest. `Ord` compares the ordinal first. Make it one shared type
  if that fits.
  - Before choosing the representation, check every place that relies on an
    id being one integer: serde and wire formats, display tags (the
    high-48-bit tag), hashing, the Python bindings, slotmap/BTreeMap keys,
    and golden digests.
  - If one of them genuinely needs a single integer, stop and report it
    before working around it; that is the exception Ev named.
- **Retire stored order.** Delete `Doc::order`, `Doc::positions` and
  `Doc::var_order`, deriving each from the ids. Repoint every reader to id
  order: `eval::schedule`, the mate solve's tree edge, the resolve lanes'
  evidence order, the refactor, `check_declared_sides`, and pncad-py's
  node map. Retire `VarOrderMismatch` and the `order` list on the wire.
- **Collision refusals.** `NodeIdCollides`, `VarIdCollides` and
  `StepIdFault::Collides` become unreachable within a document; remove
  them. Keep the load door's check that the mint log ascends.
- **`emit_union::Flush`** keeps "least", which now means first minted. Its
  docs change to say so.
- **Re-baseline** every id-bearing corpus document and golden, and say in
  the PR what moved. On the corpus, `name_tables_by_position` should not
  move.
- **Test:** re-drawing another member never takes a held flush stretch. Use
  the designers' fixture, with `declared_side_fault`'s refusal set aside, or
  wait for the doors row that relaxes it.

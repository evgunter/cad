---
id: sibling-branches-mint-one-node-id-for-different-nodes
kind: issue
title: Two inserts applied to one base mint the same node id for different nodes
status: review
opened: 2026-09-29
priority: P1
cost: H
branch: emit/node-id-digest-mint
blocked_on: [3565]
refs: [name-order-was-insertion-order-under-the-counter]
pr: 3594
---


## What

`RecipeNodeId`s are still minted from `Doc::next_id`, a counter that is
part of the document value (`crates/editor-core/src/doc.rs`, `Doc`'s
`next_id` field; the insert door in `crates/editor-core/src/edit.rs`,
`apply_maintaining`'s `InsertNode` arm). Two inserts applied to one
base — an undo followed by a different insert, or two copies of one
file edited apart — mint the same id for two different nodes, and a
`StableName` carried from one branch into the other (its `node` half)
denotes the other branch's node without a report.

This is the node-id half of
`work/emit/sibling-branches-mint-one-step-id-for-different-steps.md`.
That row's ruling (Ev on #3262, option (b)) moved profile STEP ids to a
digest chain (`crates/editor-core/src/step_mint.rs`) and left node ids
on their counter "for now; moving them too is a later row, because the
blast radius is large". This is that row.

## Evidence

`crates/editor-core/tests/asm_parent_held_names.rs`,
`sibling_versions_mint_one_node_id_for_different_nodes`: two inserts
applied to one base mint one `RecipeNodeId` for two different
extrudes. The row pins the behaviour and is the one that turns.

## What it also fixes

A step id's uniqueness across branches inherits the node id's. The
mint preimage (`MintingEdit` in `crates/editor-core/src/step_mint.rs`)
hashes the node the edit mints for and the profile's plane, both
`RecipeNodeId`s. Two branches whose different nodes share a node id and
author byte-identical programs mint the same step ids, so a step id is
only as branch-unique as the node ids it hashes. Moving node ids off
the counter closes that for step ids too.

## Where to start

- The step-id mint (`StepMint::mint`, `MintingEdit`) is the shape the
  ruling chose; node ids are spelled across the whole tree (every
  `StableName`, every input edge, `refactor.rs`'s `NodeMap`
  precomputation from `next_id`), so the width and the precomputed
  remaps are the cost to size first.

## Sizing (2026-09-30)

### How it was measured

A throwaway probe, committed nowhere, changes one thing: the insert
door mints a fixed bijective scramble of the counter, and `has_minted`
inverts it. Ids stop being small and sequential, and nothing else
moves. To reproduce the numbers below, apply it by hand:

```rust
// crates/editor-core/src/doc.rs, beside `Doc`:
pub(crate) fn proto_scramble(mut x: u64) -> u64 {
    x ^= 0x9e37_79b9_7f4a_7c15;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
pub(crate) fn proto_unscramble(mut x: u64) -> u64 {
    x = x ^ (x >> 31) ^ (x >> 62);
    x = x.wrapping_mul(0x319642b2d24d8ec3);
    x = x ^ (x >> 27) ^ (x >> 54);
    x = x.wrapping_mul(0x96de1b173f119089);
    (x ^ (x >> 30) ^ (x >> 60)) ^ 0x9e37_79b9_7f4a_7c15
}
```

and substitute:

- `Doc::has_minted`: `proto_unscramble(id.0) < self.next_id`;
- `edit.rs`, `InsertNode`: `RecipeNodeId(proto_scramble(new.next_id))`;
- `refactor.rs`: `proto_scramble(doc.next_id + i as u64)` (inline) and
  `proto_scramble(i as u64)` (split);
- `mate/member.rs`'s test fixture: `doc.next_id = proto_unscramble(MATE.0) + 1`;
- on a tree before `part-suites-name-every-parts-body-by-one-constant`
  only, `fixture::resolver::PART_BODY` and `pncad`'s `WS_PART_BODY`:
  `RecipeNodeId(10905525725756348110)`, which is `proto_scramble(2)`.
  Those constants are gone since; the suites read each body from its
  part.

- **Before unit 1:** editor-core failed 519 of 2376 tests.
  - 290 of those came from one fixture constant, `PART_BODY`.
  - About 80 came from one committed corpus, the tour die.
  - The rest were spread over about 45 suites.
- **Tree-wide:** there were 728 literal `RecipeNodeId(<n>)` spellings in
  135 files. Every one in `crates/viewer/src` sits in a `#[cfg(test)]`
  module, and most across the tree are shape-only: they build a name or
  a message and never insert a node.

### Where node ids are minted or precomputed

- `edit.rs`, `apply_maintaining`'s `InsertNode` arm.
- `refactor.rs`: the inline's and the split's `NodeMap`, which
  `step_map_of` simulates the step mint over. A forward name reference
  is already refused (`DeclareNamesMissingNode`), so a sequential
  simulation has no cycle.
- `doc.rs`: the `next_id` field, `has_minted` and `bit_eq`.
- `persist/check.rs`: `IdBeyondCounter`.
- The `Rebind` source check and `resolve`'s `node_gone` now ask
  `has_minted`, which unit 1 did.

### Settled by #3262 or by PR 3455, not open

- **What a node's id hashes:** its `InsertNode` edit's canonical bytes,
  display units erased (D6). A refactor records plain `InsertNode`s, so
  hashing each insert is the only choice replay agrees with.
- **Width:** u64, as for step ids.
- **File format:** the change breaks it, and an old file refuses typed
  with the regenerate recourse, as the step mint did. No migration.
- **Order:** anything keyed by id becomes hash order, as `select` did
  for steps.
- **Mint shape:** both designers recommend one document mint for node
  and step ids, `StepMint` becoming the document's `Mint`, with one
  chain and one log. That is unit 2's intended shape.

### The units

1. **Tests take node ids from the insert door.** No behaviour change.
   Done on `emit/tests-take-node-ids-from-the-door` (41 test files).
   Under the probe, editor-core, viewer and pncad now fail 123 of 3283
   tests. Every one is in a class listed under "Left for unit 2", and
   more than half are loads of the committed corpora.
2. **The node mint.** The mint and its log, the load-door checks, the
   `refactor.rs` precomputation by simulation, and regenerating or
   re-baselining everything under "Left for unit 2". N1, IDENTITY.md
   DI1, ASSEMBLY.md and REFERENCES.md say "counter" and need rewording.
   `sibling_versions_mint_one_node_id_for_different_nodes` turns here.
   Unit 2 waits on unit 1 and on
   `work/emit/part-suites-name-every-parts-body-by-one-constant.md`.
3. **How a node is shown.** The viewer labels a node "feature {id}",
   refusals say "node {id}", and Python prints `NodeId({id})`. With
   digest ids all of these print 20-digit numbers. The question is with
   Ev as [ev] #3565 (ruling item `work/emit/how-a-person-sees-a-node.md`).
   Unit 3 waits on it. Unit 2 should not land before it, or the GUI
   shows raw ids in between.

### Left for unit 2

The probe's residue after unit 1.

**Committed bytes that spell ids.** Regenerate them, or re-freeze
them, in unit 2:

- `crates/editor-core/tests/corpus/tour/die_composed_tour.pncad`
  (about 80 rows through `corpus::die_composed_tour`) and
  `tests/corpus/die_tool.pncad` (`msolve6`, `msolve7`).
- `crates/viewer/tests/gallery_ring.pncad` (`doc_io`, `display_budget`,
  `index_memo`, `pick3_acceptance`, `review_gui2_*`, `review_pick_r2`,
  `creation_ops`) and `crates/pncad/tests/plate_param.pncad`.
- `tests/golden/golden.cad` (`m4_pr6_golden`).
- The samples `unreadable_by_this_build::OLDER_SHAPED`,
  `bool13_r1_probes`' older-shaped document, `lib_tube_node`'s older
  document and `wire_rv_unknown`'s control document.
- The counter-specific load-door rows: `m4_pr6_refusal`'s `next_id`
  clip and `persist::check`'s `rv_the_name_pass_refuses_in_document_order`.
- `pncad-py/tests/test_document.py`'s `"next_id"` tamper.

**Digests and goldens that hash ids.** Re-baseline them, and say what
moved:

- `m4_pr3_names_ci`, `m4_pr4_ci`, `seat4_verb_lowering`,
  `seat8_split_lowering`, `m10_6_ci_rows_interval`,
  `m10_sym_profile_interval` and `asm2b_multisolid` row 5.
- `drive.rs`'s `DriveReport::serialize` writes `node.0` into its
  goldening text (`render_reason`, `" {}:structure:{:?}"`, and the flip
  and status rows beside it). That text is compared by the `m10_*`
  interval rows, not persisted to a user file.

**Kernel behaviour that orders by id.** Unit 2 decides each case: keep
id order, which becomes arbitrary, or order by position in `Doc::order`.

- `eval/schedule.rs` breaks topological ties with
  `BinaryHeap<Reverse<RecipeNodeId>>`. `Evaluation::order` is what
  `tree::rows` draws, so the feature tree would draw in hash order.
  Unit 2 must tie-break by position in `Doc::order`.
- The mate solve's spanning tree takes the first pair by id:
  `msolve1_transform_aware::a5_*` and `msolve2_member_chain::a3b_*` ("the
  first pair is the tree edge"); `msolve2_member_chain::a2a_*` too,
  under a probe salted per document.
- Which member holds a flush stretch of a shared rim:
  `emit_union_rim_piece_ranks::fam010_*` and
  `emit_shared_rim_several::the_chord_is_named_as_the_rim_piece_it_lies_on`.
- A diff's report order, and `resolve_upstream_scope`'s "R is first in
  deterministic order".
- Unit 1 rewrote some rows to state their rule rather than a literal
  that holds only under the counter: `review_decl_r1`, `docm7`, `docm8`
  (a contact pair spelled lower id first and refused in id order),
  `m4_pr7_appearance` (an ambiguous loss sited at the first carrier by
  id), and `edit_set_program` (the later profile in id order holds a
  repeated step id). Each of those rules is id order, and unit 2 should
  decide whether it stays.
- `review_m4_pr1_die::r7_*` relies on two authorings in different
  orders minting one id set. Under a digest they share none, so the row
  needs rewriting.

### What the probe cannot see

The probe keeps every coincidence between documents that the counter
has: the n-th insert of any document gets the same id. A digest does
not keep it where the minting edits differ.
A variant salted per document does break those coincidences, and its
run is in `work/emit/part-suites-name-every-parts-body-by-one-constant.md`:
the part suites' one body constant is gone, and what it still fails
beyond the unsalted probe is one more id-order row (above) and a recipe
replayed under another document id, which a mint of the edit bytes
keeps. Rows that compare two
documents built by the same edits are safe under D9. Those the probe
fixed read ids from each document, but the premise needs unit 2's mint
to confirm it.

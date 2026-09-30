---
id: sibling-branches-mint-one-node-id-for-different-nodes
kind: issue
title: Two inserts applied to one base mint the same node id for different nodes
status: open
opened: 2026-09-29
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

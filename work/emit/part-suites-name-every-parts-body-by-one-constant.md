---
id: part-suites-name-every-parts-body-by-one-constant
kind: issue
title: Assembly suites name every part's body by one constant id across part documents
status: open
opened: 2026-09-30
priority: P1
cost: M
parent: sibling-branches-mint-one-node-id-for-different-nodes
---


## What

`fixture::resolver::PART_BODY` (`crates/editor-core/tests/fixture/resolver.rs`)
is one `RecipeNodeId`, `RecipeNodeId(2)`. `in_part(instance, cap)` names
a face of whatever part an instance pins through that one id, and
`PartStore::insert` checks it, through `assert_part_body`, against every
part document it stores. `pncad`'s suite has its own copy,
`WS_PART_BODY` (`crates/pncad/tests/all.rs`).

So every part document an assembly suite builds must mint its extrude
under the same id. The counter makes that true for any frame, profile,
extrude part. A digest-chain mint does not: the id hashes the minting
edit, and the suites' part builders differ in size (`msolve2`'s
`part_doc(label, w, h)` builds two), so their bodies get different ids.

## Size

- There are about 350 `in_part(` calls in 30 files: editor-core
  `tests/` (`msolve*`, `mate1*`, `mate6*`, `asm_r2a_mate_solve`,
  `asm_r2b_assembly`, `edit_instance_crossing_names`,
  `docm6_seam_declarations` and more) and viewer `tests/`
  (`common/asm.rs`, `msolve3_placer_refused` and more).
- There are also the direct `PART_BODY` spellings in `node:` fields
  (`asm_r2b_assembly`, `edit_one_predicate`, `msolve5`,
  `mate1_member_vocab`), and `WS_PART_BODY` in `crates/pncad/tests/all.rs`.

## Fix

`in_part` takes the part's body, and the body comes from the part
document (`blank_of`-style: its one extrude). Each suite's part builder
then hands back the body with the document.

## Why it is its own row

Unit 1 of `sibling-branches-mint-one-node-id-for-different-nodes` is
checked with a scramble probe that permutes the counter. That probe
keeps "the third insert of every document mints one id", so it cannot
tell this fix from the constant. The probe moves `PART_BODY` along with
its scramble. This row must land before unit 2, which makes the
coincidence false.

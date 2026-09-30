---
id: part-suites-name-every-parts-body-by-one-constant
kind: issue
title: Assembly suites name every part's body by one constant id across part documents
status: closed
opened: 2026-09-30
priority: P1
cost: M
parent: sibling-branches-mint-one-node-id-for-different-nodes
closed: 2026-09-30
branch: emit/part-body-from-the-part
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
`part_doc(label, w, h)` builds a base and a top of different sizes),
so their bodies get different ids.

## Size

- `PART_BODY` and `in_part(` occur 329 times in 34 files, the
  definitions included: editor-core `tests/` (`msolve*`, `mate1*`,
  `mate6*`, `asm_r2a_mate_solve`, `asm_r2b_assembly`,
  `edit_instance_crossing_names`, `docm6_seam_declarations` and more)
  and viewer `tests/` (`common/asm.rs`, `msolve3_placer_refused` and
  more).
- `pncad`'s copy, `WS_PART_BODY` in `crates/pncad/tests/all.rs`.

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

## Closed

`in_part(instance, body, cap)` names a cap of `body`, and each suite's
part builder hands `body` back with its document: the id its extrude
insert minted, through `PartStore::insert_part` where the part goes into
a store. `msolve6_part_extent` reads it off the part instead
(`body_node`: the one extrude or revolve), because its builders' plain
documents feed pins and product rows too. `PART_BODY`, `assert_part_body`
(whose one job was to hold the constant to the builders) and `pncad`'s
`WS_PART_BODY` are gone. A suite that mates two different parts takes
each side's body separately (`msolve10`'s `mate_across`, `msolve6`'s
`clocked`, `part_depth_bound`'s `mated`).

Four rows leaned on the constant's coincidence with a HOST id and are
restated: `edit_instance_crossing_names`' inner-rebind row spells its
`inner` on the live node it collides with; `docm6`'s own-mate row named
the stand's cubes as `RecipeNodeId(1)` and `(0)` and now takes them from
`resting`; `pncad`'s crossing probe spelled its `outer` as the part's
body cap, which was live in the assembly only because the mate there
was node 2, and now spells it as the first instance's cap;
`msolve1`'s A6 compared two documents' raw instance ids and now
compares which of each document's `(base, top)` the refusal names.

**The probe.** Throwaway, not committed: the insert door mints
`scramble(counter ^ salt(doc id))` and `has_minted` inverts it — unit
1's scramble, salted per document, so two part documents from one
builder mint different body ids. The refactor precomputations and
`mate/member.rs`' fixture take the same salt. Under it, `--profile ci`
over editor-core, viewer and pncad:

- `origin/main`: 419 of 3233 fail.
- this branch: 125 fail, every one also failing on main.
- The 294 main-only failures are the part suites (31 `asm_r2a`, 25
  `asm_r2b`, 22 `msolve3`, 18 `msolve6`, …, and every `PartStore`
  user through `assert_part_body`).
- Of the 125, 108 fail under unit 1's unsalted scramble too, the
  classes "Left for unit 2" lists. 16 are the salt's own artefact: a
  recipe replayed into a document of another id (twins, save-as, a
  round trip, `asm1`'s id-excluded pin), which a mint hashing the edit
  bytes alone does not tie to the id. The last,
  `msolve2_member_chain::a2a_*`, is the spanning tree's first pair by
  id — "Left for unit 2", beside `a3b`.

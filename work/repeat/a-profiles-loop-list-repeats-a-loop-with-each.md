---
id: a-profiles-loop-list-repeats-a-loop-with-each
kind: unit
title: A profile's loop list repeats a loop with each k { loop }, one profile holding every copy
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: a-loop-cannot-repeat-inside-a-profile
blocked_on: [profile-readers-read-kernel-steps-through-the-written-address-map, a-profile-piece-name-carries-its-member]
---


Unit 3 of `a-loop-cannot-repeat-inside-a-profile` (Ev, PR 7263): the
first block, around a whole loop. Examples are a bolt circle of holes,
or a row of slots, in one plate's profile.

## The ruled text it implements

`docs/PATHS-DESIGN.md` §2 *Composition*: "in the loop list
`each k { loop }` is one loop per value", and "a step that reads `k`
with no braces makes the whole profile a family, one profile per value;
a read of `k` outside the braces, in a profile that has them, refuses".
`docs/DESIGN.md` `[repetition]`: the block "takes the index whole as
`union` takes a `Bodies`".

## Done when (each a test)

- **One profile.** A plate whose loop list is an outer rectangle and
  `each k { circle(c(k), r) }`, with `k = index(6)` and `c(k)` a polar
  point in `cos`/`sin` of `scalar(k)·turn/6`, evaluates to one profile
  with seven loops. `ev.value(profile).members` is empty: the profile
  is not a family. Its extrude is one body with six inner walls.
- **Names.** Hole 3's wall is `Lateral(Piece { step, role, member: [(k, 3)] })`,
  on that one body. Setting `N` from 6 to 4 makes hole 5's name resolve
  `Vanished`.
- **The count is a variable.** `N` read from a parameter works. A count
  that does not evaluate refuses as the existing `IndexCount` fault,
  naming the index.
- **The stray read refuses.** In a profile holding a block over `k`, a
  step outside the braces that reads `k` refuses at the edit door and
  at the load door, with a new typed `ProgramRefusal` that names the
  index and the slot. The frame row (`DrawnOn`) is outside the braces,
  so a frame that reaches `k` refuses the same way.
- **The family stays.** A profile with no block whose step reads `k`
  is still a family: `a_profile_reading_the_index_is_a_family_named_by_its_pieces`
  (`crates/editor-core/tests/g_index_families.rs`) passes unchanged.
- **Lockstep.** Another node reading the same `k` (a bolt placed at
  `c(k)`) is a family of six whose member `k` sits on hole `k`. This
  holds by `[repetition]` as it is, and the test pins that the
  block's whole-taking did not change what other readers reach.
- **The lowering is flat.** `ProfileProgram::resolve*` hands
  `profile::replay` seven ordinary loops. The map from unit 1 says
  kernel loops 1..6 are written loop 1 at members 0..5. `crates/profile`
  is unchanged.
- **Persistence.** The block saves and loads. A document with no block
  is byte-identical to before. The JSON depth bound
  (`crates/editor-core/src/persist/nesting.rs`, measured by
  `tests/expr_nesting_bound.rs`) is re-derived if the block deepens
  an expression's position.
- **Census.** The new form reaches a witness in
  `tests/switch_program_vocabulary.rs` (`ALL_NAMES`).

## The mechanism, as found in the tree

- **Representation.** A new form in the `document_vocabulary!`
  invocation in `crates/editor-core/src/program.rs`. A new
  `LoopProgram` variant holding the index read and the loop keeps every
  existing document's bytes. Changing `ProfileProgram::loops`'s element
  type would re-bless every saved profile listed under
  `crates/editor-core/tests/` and `crates/viewer/tests/gallery_ring.pncad`.
  Prefer the variant, and say in the PR if it cannot be done. The
  braced loop is **one written loop**: `ids` keeps one list for it, with
  one id per written step, and its `SlotId::Profile`s are its written
  steps'.
- **Reach.** `Reach::of_node` (`crates/editor-core/src/repetition.rs`)
  walks `node.exprs()` with no exception, so a braced loop's rows would
  make the profile a family. The block's rows stop contributing `k` to
  the node's reach: either a hook on `ProfilePayload`, or a new
  `OperandSlot` (`crates/editor-core/src/operand.rs`) seating `k` with
  `takes_whole` extended to it. The implementer chooses, and the PR
  says which.
- **Lowering per member.** `LoopProgram::resolve_with` takes a
  `VarEnv` and no `Doc`, but the member envs come from `Doc`:
  `Doc::member_envs` (`repetition.rs`) and `Doc::bind_member`
  (`doc.rs`). The profile arm in `crates/editor-core/src/eval/mod.rs`
  has the doc in scope, and `resolve_written_loops` has its scratch
  doc. The resolve signature gains whatever hands it per-member
  environments. The member's count faults come from `MemberCountFault`
  and map to `IndexCount`.
- **Doors.** `ProfileProgram::lattice` judges the braced loop as it
  judges any loop. `persist::check::first_program_fault` goes through
  the same path.
- **Words.** `spoken.rs` and `names/words.rs` say a member's piece as
  "… of hole 3" or similar. The wording is the implementer's, in the
  refusal and naming register.

## Not admitted in this unit

- **Nested blocks** (a grid of holes) are refused at the door, typed,
  until the nesting question in the cut is answered. The refusal is the
  conservative state, and lifting it is additive.
- **A block of outer loops** meets `MultipleOuterLoops` exactly as
  hand-written outer loops do
  (`work/profile/a-word-sketched-as-one-profile-refuses-multiple-outer-loops.md`).
  This unit does not touch that.

## Check first

1. `N = 0` is not settled by the ruled text (a question in the cut).
   Do not spec the `N = 0` case until it is answered.
2. Read `crates/editor-core/src/eval/members.rs`'s module doc against
   the block. A whole-taking seat reads the whole family even inside a
   member that reaches the same `k`. Confirm that a profile with a block
   over `k`, used by a node that is itself a family over `k`, gives that
   node's member `k` the whole profile with all N holes. That is what
   `[repetition]` says of a whole-taking seat. If the evaluator gives
   one hole instead, stop and report.
3. `ProfileProgram::structure` and `partition` are value-free, and
   their doc says "a piece absent here is absent at every value". The
   member count is a value. The maintenance checks in `edit.rs` (the
   referent-against-partition reads near lines 5918 and 6243) must
   admit a piece whose written piece is in the partition, at any member,
   and leave a member past `N` to resolve `Vanished` at evaluation. If
   the partition cannot say that without reading `N`, stop and report.

## Leaves for later

- Runs inside a chain: `a-chain-repeats-a-run-of-its-steps-with-each`.
- The viewer: `work/vseam/the-profile-editor-shows-and-writes-a-repeated-loop-or-run.md`.
- The Python door: `work/pydoor/python-writes-each-k-around-a-loop-or-a-run.md`.
- A selection keyed by an index expression:
  `a-selection-cannot-name-a-repeated-piece-by-an-index-expression`.

## Rows and ground

- With units 4 and 5 it closes the parent row's loop half.
- It crosses DOCTAIL (`edit.rs`, `persist/*`, `node.rs`), SELFREF
  (`program.rs`, `persist/check.rs`), BORDERS and TALLY (`eval/mod.rs`),
  and REFS (`names/role.rs`). It is persist ground, so announce on the
  away channel. SELFREF U1 is coupled in `program.rs`.
- Review: **dual**. It is the first vocabulary of the build. It touches
  reach, naming, persistence and the doors, and the whole-seat rule is
  the kind of thing one reviewer misses. Difficulty **H**.

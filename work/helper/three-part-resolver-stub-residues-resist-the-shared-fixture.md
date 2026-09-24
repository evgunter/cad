---
id: three-part-resolver-stub-residues-resist-the-shared-fixture
kind: issue
title: Three editor-core stub residues the shared part-resolver fixture cannot absorb as written
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Disclosed by SUITE's `editor-core-suites-carry-eleven-part-resolver-stubs`
migration, which collapsed sixteen `editor-core` suites onto
`crates/editor-core/tests/fixture/resolver.rs`. Three copies were left
where they are, each for a different reason, and each is a separate
decision this row owns.

1. **`asm2a_instantiate.rs`'s `StubStore` is a SUPERSET.** It carries a
   second map, `eps_seam`, and a third refusal arm returning
   `ResolveFault::EpsilonSeam` before the pin is ever computed — the
   only suite that exercises that fault through a resolver, at
   `row5c_epsilon_seam_refuses_typed`.
   Collapsing it onto the fixture would mean adding a knob one consumer
   varies to a store fifteen others share, so it was not forced. The
   suite also declares `CyclicStore`, a deliberately different resolver
   for the cycle probe, so it keeps a local resolver either way. The
   open question is whether the fixture should grow a seam-refusing
   constructor or whether asm2a's store is simply a different
   instrument that should say so in its own prose.

2. **`asm_r2a_mate_solve.rs`'s `in_part` has a different SIGNATURE** —
   `in_part(instance, part_node)`, taking the body node as an argument
   and hardcoding `CapEnd::Start`, where the fixture's is
   `in_part(instance, cap)` over `PART_BODY`. Neither is a widening of
   the other. A fixture `in_part_of(instance, body, cap)` with
   `in_part` as its `PART_BODY` specialization would hold both, and
   would also give the two `RecipeNodeId(1)` suites a spelling that
   says which node they mean; it was not written here because nothing
   in the migration needed it and a helper with one caller is its own
   defect.

3. **`asm4_split_inline.rs`'s `PART_BODY` is a different TYPE.**
   `const PART_BODY: usize = 2` is a positional index into
   `doc.order()`, not a `RecipeNodeId`; the two are numerically equal
   by coincidence of the same three-node part shape. One name, two
   types, in the same directory, is the shape that drifts — and
   `docm6_seam_declarations.rs`'s header already disclosed in prose
   that *"the suites' `in_part` spellings have already diverged once by
   a node index"* and nothing read it.

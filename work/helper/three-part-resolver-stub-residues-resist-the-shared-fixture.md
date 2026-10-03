---
id: three-part-resolver-stub-residues-resist-the-shared-fixture
kind: issue
title: asm2a_instantiate's seam-refusing StubStore is the one part-resolver stub left beside the shared fixture
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Disclosed by SUITE's `editor-core-suites-carry-eleven-part-resolver-stubs`
migration, which collapsed sixteen `editor-core` suites onto
`crates/editor-core/tests/fixture/resolver.rs`. Three copies were left
where they are, each for a different reason; two have since gone
(below).

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

Items 2 and 3 are done:

2. `asm_r2a_mate_solve.rs`'s own `in_part(instance, part_node)` folded
   into the fixture's `in_part(instance, body, CapEnd::Start)` once the
   fixture took the body
   (`part-suites-name-every-parts-body-by-one-constant`), and so did
   `edit_one_predicate.rs`'s copy of the same shape.
3. `asm4_split_inline.rs` names its positions `PLANE_POSITION`,
   `PROFILE_POSITION` and `BODY_POSITION`, and the fixture's
   `PART_BODY` is gone, so no name spans the two types.

Item 1 is what this row still owns.

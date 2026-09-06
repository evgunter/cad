---
id: member-space-declarations-are-order-shaped-across-a-chain
kind: issue
title: A member-space declaration across a CHAIN of contacts fuses or refuses by member-list order
status: open
opened: 2026-09-06
refs: [DOCM-7, 2028]
---


## What

Found by DOCM-7's dual review of head `723c0f9f` (R1 MAJOR-1, probe
`r1_c2_member_space_declarations_across_three_chained_members_by_order`
on `docm/7-review-r1`; measured again on the unit's branch as
`member_space_declarations_across_a_chain_are_order_shaped`).

Three blocks that touch in a CHAIN — `a` = x∈(0,1), `c` = x∈(0.5,1.5),
`d` = x∈(1.2,2.2), so `a` meets `c` and `c` meets `d`, and `a` does not
meet `d` — under one `Node::Union` with both contacts declared in
member space (four flush pairs per contact, all of them
`FromMember`-headed names of the members' own faces, no accumulation
row anywhere in the payload):

| member list | outcome |
|---|---|
| `[a, d, c]` | fuses; one body, volume 2.2, four `Merged` rows |
| `[d, a, c]` | fuses, the same body |
| `[a, c, d]` | refuses |
| `[c, d, a]` | refuses |

The refusal is `DeclareResolve { Vanished { name: FromMember{c, Cap},
diagnosis: RecipeEdit { NodeChanged(union) } } }` — `c`'s cap is not a
row of either operand table at the step that would need it.

## Why

`crates/editor-core/src/eval/wire.rs`, `wire_union`'s fold and
`route_declarations`: a declared merge CONSUMES the two faces it joins
and publishes `Merged([a.cap, c.cap])` in their place
(`names/emit_union.rs`'s `collapse`, and the pair emitter it wraps).
So in the orders that fold `c` in at step 1, `c`'s own cap is gone
before `d` arrives at step 2, and the `(c.cap, d.cap)` pair — routed
correctly to step 2, since `d`'s index says so — finds nothing to
resolve. In the orders that fold `c` in LAST, both contacts are still
member faces when their step runs, and the union fuses.

The recourse a user has is to spell the accumulation's `Merged` row
instead of the member's face. That is order-shaped too: which faces
are in the row, and how the rows nest, is the fold tree.

## Where it contradicts what is written

Three prose sites claimed order-independence without this bound, and
DOCM-7 rewrote all three to the MEASURED limit (a member-space
declaration resolves at its step only while the face it names is still
an operand-table row there) rather than pretend it away:

- `crates/editor-core/src/eval/wire.rs`, `route_declarations`'s doc
  ("# What a member-space declaration survives, and what it does not").
- `crates/editor-core/src/node.rs`, `Node::Union`'s "# The `declare`
  field, and why it records no position".
- `crates/editor-core/src/names/role.rs`, `RoleSeg::FromMember`'s
  identity paragraph.

DM4's amended bullet (`docs/DOCM-REFERENCES-DESIGN.md`, 2026-09-06) is
the ratified sentence, and it does not carry the bound either. That is
the ruling this file asks for.

## The three options, and what each costs

1. **Constituent look-through.** The declaration door, when a
   member-space name is absent from the step's tables, looks for a
   `Merged` row of the accumulation whose constituent SET contains it,
   and resolves to that row. Every order then fuses. Costs: the door
   stops being "resolve this name in these two tables" — it acquires a
   search, and a second name in the same merge group resolves to the
   same face, which the pair verb must be able to take. It also has to
   be decided whether a Vertex/Face carried pair looks through the same
   way. Touches `resolve_declarations`, which is the PAIR boolean's
   door too (one definition, two callers), so the pair boolean would
   inherit whatever is chosen.
2. **Flatten nested `Merged`.** `collapse` currently preserves the
   nesting the fold produced, which is the fold tree and so a position
   — in the one place DM4 says no position is recorded (R1 S8). A
   flattened `Merged` makes a re-merged face's NAME order-free, which
   makes the recourse spelling order-free. It does not by itself make
   the member-face spelling resolve, so it is a complement to (1) or a
   smaller fix on its own. Costs: a name change in published tables
   (corpus and golden churn), and a decision about what a constituent
   set means when two merges overlap.
3. **Narrow the contract.** DM4 says outright that a member-space
   declaration is answered while the face is still a member's, and that
   chains are declared against accumulation rows — with the refusal
   text saying so. Costs nothing to build (it is what the tree does
   today) and leaves the authoring hole: the accumulation spelling
   needs the intermediate name, which is exactly what
   `work/chrome/a-declared-union-has-no-one-pass-authoring-path.md`
   says a user cannot get at today.

## Where it stands

Open, on DOCM's slate, for Ev: it revises DM4's amended bullet whichever
way it goes. DOCM-7 ships the measured behaviour, the three rewritten
prose sites and the pinning row; nothing here blocks that unit.

The row flips when the ruling lands:
`crates/editor-core/tests/docm7_union_declare.rs`,
`member_space_declarations_across_a_chain_are_order_shaped` — it asserts
BOTH outcomes as they are today and names this file.

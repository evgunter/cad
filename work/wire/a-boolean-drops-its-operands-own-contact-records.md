---
id: a-boolean-drops-its-operands-own-contact-records
kind: issue
title: A boolean drops its operands' own contact records, so a union's record and 3′ verdict follow the member it folds last
status: open
opened: 2026-10-02
priority: P1
cost: M
---



## What

A boolean publishes only the contacts its own reduction decided between
its two operands. An operand's own record, the contacts that made it,
does not reach the result, although their entities survive untouched.

- `topo`'s ops take two bodies. An operand's own records re-enter only
  as `BooleanDeclarations::carried_a`/`carried_b`
  (`crates/topo/src/boolean/ops.rs`, `remap_carried`), and only the
  user's same-operand declarations fill those (`crates/editor-core/src/eval/wire.rs`,
  `resolve_declarations`).
- `wire_boolean` hands on its step's record alone. `wire_union` publishes
  "the LAST step's record", whose comment calls the earlier steps'
  records consumed. They are dropped, not consumed: the body still
  holds those contacts, and the at-rest census refuses them as
  undeclared.

So a union's record and its tier-3′ verdict depend on which member it
folds last, while its body does not.

## Witnesses

All from `crates/editor-core/tests/union_pinch_member_order.rs`
(`DROPPED_RECORDS`), on branch `tang/pinch-union-order`.

- **Plate and two blocks touching along a vertical line** (`pinch`): the
  body is identical in all six member orders. The two orders that fold
  the plate last publish no v-v record, and 3′ refuses them with
  `UndeclaredContact` `VertexVertex` at (1.5, 1, 1.7) and
  `EdgeEdgeOverlap` at (1.5, 1, 1.35). The other four publish that pair
  and pass. The side-face pinch and blocks through the plate split the
  same way.
- **Three blocks** (`two_pinches`, 24 orders): only the orders that fold
  `p1` last pass, because `p1` is the one block that touches both of the
  others.
- **Topo level, no editor**: `union(&plate, &union(&q1, &q2))` publishes
  `vv = 0` and fails 3′, while `union(&union(&plate, &q1), &q2)` publishes
  `vv = 1` and passes. Here `q1 = brick((1,1.5),(-1,1),(0.5,2))` and
  `q2 = brick((1.5,2),(1,3),(0.47,1.7))`. Passing `q1 ∪ q2`'s own v-v rows
  in as `carried_b` (class `Tangent`) makes the first pass, with
  `vv = 1`.
- **Pair nodes**: the plate ∪, ∖ and ∩ the joined blocks, both ways
  round, all fail 3′. So do P − X, X − P, X ∪ P and P ∪ X, where
  X = slab − p1 − p2 and P is the plate. They fail on main too.

## Owed

- Carry each operand's surviving records through the op, under
  `remap_carried`'s drop rules. Whether the fold step's comment is the
  rule or the defect is the ruling to make first.
- When this lands, the test's `DROPPED_RECORDS` assertions turn red:
  replace them with one record and one verdict across orders.

Same class as `transform-and-pattern-drop-a-values-contact-records`: a
node hands on a body without the records that certify it.

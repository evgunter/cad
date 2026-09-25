---
id: step-arg-roles-are-spelled-in-three-homes
kind: issue
title: Every StepArg role except the target pair is assigned three times in program.rs: the resolvers, the enumeration and the accessor macros
status: open
opened: 2026-09-25
priority: P1
cost: D
---

## Finding

PR 3141 (row `res-target-slot-roles-are-unguarded-and-duplicate-spec-slots`)
reduced the point-target pair to one spelling, `target_roles(second)` in
`crates/editor-core/src/program.rs`. Four places read it: `target_slots`,
`target2_slots`, `target_coord` (under both accessor macros) and
`res_target`. Every other role still has three homes, and all of them
decide which `StepArg` a given argument of a given spec or step is:

- **resolution:** `res_step`'s literal roles, and `res_spec`'s
  `pick(A::X, A::X2)` calls (via, centre, carrier radius, bulge, sweep,
  arc length);
- **enumeration:** `spec_slots` and `step_slots`;
- **addressing:** `spec_arg_access!` (keyed on `second`) and
  `step_arg_access!`.

One thing holds them together:
`switch_program_vocabulary::every_enumerated_slot_is_where_its_refusal_reports`.
It asserts that resolution and enumeration agree over every slot in the
corpus, and it goes through `expr_mut`, so it checks addressing too.
Its blind spot is the corpus's own: step shapes the corpus omits.

`StepArg::dimension` and `StepArg::is_radius`
(`crates/editor-core/src/node.rs`) also list every role. They classify
each variant rather than assign roles to arguments, so they are not a
fourth home of this rule.

## What a taker owes

One table. For each (spec or step shape, argument, `second`) there
should be one role, and all three consumers should read it. That table
could generalise `target_roles` to a per-role twin function, or it could
be a single visitor that yields `(role, &Expr)` pairs, which the
resolvers, the enumeration and the accessors would all consume. Each
option trades against the macros' borrow-generic shape, and choosing
between them is the design half. That is why this row is D, not E.

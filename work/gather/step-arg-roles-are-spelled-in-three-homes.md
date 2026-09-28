---
id: step-arg-roles-are-spelled-in-three-homes
kind: unit
title: Every StepArg role except the target pair is assigned three times in program.rs: the resolvers, the enumeration and the accessor macros
status: closed
opened: 2026-09-25
priority: P1
cost: D
branch: gather/step-arg-roles-one-home
pr: 3264
closed: 2026-09-26
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

## The unit

Branch `gather/step-arg-roles-one-home`. Every `StepArg` role, the
carrier forms' included, is now declared once, in `loop_roles` in
`crates/editor-core/src/program.rs`. It is a borrow-generic macro that
lists `(role, expression)` rows for authored step `step`. Match
ergonomics bind the same rows as `&Expr` under `LoopProgram::roles` and
as `&mut Expr` under `LoopProgram::roles_mut`.

- The enumeration (`step_args`, `step_radii`) reads the roles.
- The addressing (`LoopProgram::expr` / `expr_mut`) finds the row with
  the requested role.
- The resolution goes through one leaf, which tags a refusal with the
  role `role_of` finds for the refusing expression's own address. The
  resolvers name no role, and `second` no longer reaches them.

The two agreement censuses now hold by construction, so the unit adds
`every_enumerated_slot_resolves_into_the_field_its_role_names`. It
checks each role against an independent reading of which kernel field
the role names, which catches a table row that pairs a role with the
wrong field.

## Closed (2026-09-26)

Merged as PR 3264, on a green local `ci-local.sh` battery (Ev,
2026-09-26: merge on local green), with `[skip ci]`. Tier: single FULL
review, raised from STYLE at landing. Every `StepArg` role is declared
once, in `loop_roles!`. Chain steps resolve through their own rows.
`radius_arg_of` is derived from the table. Residue filed: EDIT's
`node-slot-tables-are-spelled-three-times-in-node-rs` (P1), and LIB's
`polygon-door-refuses-at-point-slots-its-corners-do-not-live-at` (P3).

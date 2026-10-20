---
id: editor-payload-lists-other-than-naming-are-kept-by-hand
kind: issue
title: The feature-tree census's EvalError, SeedError, placement and ResolveError lists are kept in step with their enums by hand
status: closed
opened: 2026-10-19
priority: P4
cost: E
closed: 2026-10-19
pr: 5407
branch: wire/p4-refusal-bundle
---


Found by `wire-census`'s sweep (PR 5324) for census lists that skip
an arm of the enum they forward.

`editor_payloads` in
`crates/editor-core/tests/refusal_concision_chains.rs` renders each
forwarded editor-core refusal through a per-enum list: `eval`
(`EvalError`), `seed` (`SeedError`), `placement`
(`PlacementRuleFault`), `naming` (`NamingError`) and `resolve`
(`ResolveError`). Only `naming` is tied to its enum:
`every_naming_arm_has_a_row` takes its roster from `display_contract`'s
`NAMING_ERROR` census (an `f6_variants!` match rustc holds exhaustive)
and names each row by its payload's variant, so an arm added or a row
removed reds.

The other four are kept in step by hand, and the sweep found two
missed arms in two of them: `PlacementRuleFault::NonRigidFrame` and
`ResolveError::ReadGone`. PR 5324 adds both rows, but nothing stops
the next one.

## Repair shape

Weld each list the way `every_naming_arm_has_a_row` welds `naming`: an
`f6_variants!` roster per enum (in `display_contract` where one fits,
or beside the list), with each row named by its payload's
`test_utils::f6::variant_identifier`, and a set difference that reds
in both directions. Show for each that a removed row turns it red.

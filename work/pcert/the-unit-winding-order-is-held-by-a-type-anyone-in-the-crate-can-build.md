---
id: the-unit-winding-order-is-held-by-a-type-anyone-in-the-crate-can-build
kind: issue
title: UnitDecided's fields are crate-visible, so unit_winding's doc claims an order its type does not enforce
status: open
opened: 2026-11-12
priority: P4
cost: E
---


## Finding

Filed by the orchestrator from PR 6845's TT delta review (style, three items). None is a soundness defect.

- **The doc claims an order the type does not enforce.**
  - `pcurve_cache.rs` `UnitDecided { pub(super) value, pub(super) residue }` is visible to the whole crate. The reviewer's `probe_unit_decided_is_constructible_anywhere` (`review/6845-full` @ a921b128f) builds one from a test module.
  - So `closed_bound.rs` `unit_winding`'s doc ("the one way to hold one … never before") overstates what the type guarantees.
  - Fix: move the type into its own module, with private fields and a getter. Or correct the doc.
- **A floor comment is incomplete.** `closed_bound.rs:145-148` says every `floor` is written by `spiric_floor` or `focal_floor`, but `:107` and `:335` write `T::one()`. That is correct, since the multiplier there is 1. The comment should say so.
- **A test hook shapes a production signature.** `closed_bound.rs` `spiric_copy_divisor` carries `hi_c` only so the `spiric-copy-divisor-high` mutant can read it (`let _ = hi_c;`). Find a mutant site that does not change the production signature.

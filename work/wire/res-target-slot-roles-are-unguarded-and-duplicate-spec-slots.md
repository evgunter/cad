---
id: res-target-slot-roles-are-unguarded-and-duplicate-spec-slots
kind: issue
title: res_target's StepArg roles are a second spelling of spec_slots' role assignment and nothing tests them - the Target2 twins can be dropped with 1324 tests green
status: open
opened: 2026-09-12
---



(WIRE orchestrator) Found by the full review of PR 2445 (R2), measured.
Pre-existing; PR 2445 did not introduce it but did widen the surface,
so it is filed against the shape rather than the PR.

A fused arc's SECOND target lives at different slot ids than its first
(`StepArg::Target2X`/`Target2Y` against `TargetX`/`TargetY`). Two
places in `crates/editor-core/src/program.rs` decide which pair a
given target uses:

- `res_spec`'s `tgt` (`:936-944`) — `pick(A::TargetX, A::Target2X)` /
  `pick(A::TargetY, A::Target2Y)`, passed into `res_target`
  (`:826-841`), which is what a REFUSAL's `SlotId` is built from.
- `spec_slots` / `step_expr` (`:496-505`, `:566-595`) — the same role
  assignment, written independently, and what the slot census walks.

They must agree and nothing checks that they do.

## The measurement

The review lane made `res_spec`'s `tgt` always pass
`A::TargetX, A::TargetY` — so a fused arc's second target reports its
refusals at the INCOMING spec's slot — and ran the whole editor-core
suite: **all 1324 tests green.** Nothing anywhere exercises
`res_target`'s role arguments.

`every_enumerated_slot_addresses_a_distinct_expression` cannot see it:
it walks `spec_slots`/`step_expr`, i.e. the other spelling, and only
that one is tested. So the two can disagree about which slot a fused
arrival's target lives in and the tree stays green — while a user is
pointed at the wrong field by a refusal that is otherwise correct.

## Why it is worth a row now

PR 2445 collapsed `res_spec`'s `tgt` closure into `res_target`, which
is the right move (one construct hop where there were two, and the
whole point of D364). The roles were two adjacent `pick(...)`
expressions; they are now two positional arguments travelling through a
six-parameter call. Strictly more room to get it wrong, with the same
zero coverage.

This is a Q1 hit — two spellings of one rule with only one home tested
— and a Q7 hit: the call could take the `second: bool` that `res_spec`
already has and derive both roles from it, which would make the two
spellings one. That is probably the fix, but the row's first obligation
is a test that can see the disagreement at all.

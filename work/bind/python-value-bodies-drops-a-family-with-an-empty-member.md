---
id: python-value-bodies-drops-a-family-with-an-empty-member
kind: issue
title: Python Value.bodies() returns no body for a family of bodies with an empty member
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; no test pins it yet. Filed on PYDOOR first and re-homed here, because BIND's `paths` name `crates/pncad-py/src/py/*`.

**The defect.** `Value.bodies()` (`crates/pncad-py/src/py/value.rs`) lists the bodies of an `Instances` value and returns `[]` for any other payload. Take a per-`k` cut where one member cuts away everything. That family is a family of bodies with an absent member, and it evaluates to `ValuePayload::Family`. So Python is told the family holds no body, and no error is raised. The `members()` docstring says the same thing: "a family of bodies also answers them flat (`bodies`)".

**What does not work.** A py `Value` keeps the payload and its members, not the kernel's `NodeValue`. `ValuePayload::member_body`, the kernel's rule for "is this one body", is crate-private. So the binding cannot call `NodeValue::member_bodies`; it would have to re-derive that classifier itself. The unification row `work/refs/a-family-of-bodies-has-two-payloads.md` would remove the second payload, and this row with it.

**Workaround.** It works today: `value.members()`, then each member's `bodies()`, which is `[]` at the empty member.

**The fix.** Wait on the unification row, or list each present member's body from `members` here. Whether an absent member keeps its place (as `Value.split()` keeps an empty side as `None`) or is left out is this door's choice.

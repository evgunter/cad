---
id: python-value-bodies-drops-a-family-with-an-empty-member
kind: issue
title: Python Value.bodies() returns no body for a family of bodies with an empty member
status: closed
opened: 2026-11-06
priority: P3
cost: E
parent: a-family-of-bodies-has-two-payloads
closed: 2026-11-06
---


Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; no test pins it yet. Filed on PYDOOR first and re-homed here, because BIND's `paths` name `crates/pncad-py/src/py/*`.

**The defect.** `Value.bodies()` (`crates/pncad-py/src/py/value.rs`) lists the bodies of an `Instances` value and returns `[]` for any other payload. Take a per-`k` cut where one member cuts away everything. That family is a family of bodies with an absent member, and it evaluates to `ValuePayload::Family`. So Python is told the family holds no body, and no error is raised. The `members()` docstring says the same thing: "a family of bodies also answers them flat (`bodies`)".

**What does not work.** A py `Value` keeps the payload and its members, not the kernel's `NodeValue`. `ValuePayload::member_body`, the kernel's rule for "is this one body", is crate-private. So the binding cannot call `NodeValue::member_bodies`; it would have to re-derive that classifier itself. The unification row `work/refs/a-family-of-bodies-has-two-payloads.md` would remove the second payload, and this row with it.

**Workaround.** It works today: `value.members()`, then each member's `bodies()`, which is `[]` at the empty member.

**The fix.** Wait on the unification row, or list each present member's body from `members` here. Whether an absent member keeps its place (as `Value.split()` keeps an empty side as `None`) or is left out is this door's choice.

## Closed

**Closed by the unification row** (`a-family-of-bodies-has-two-payloads`). A family with an empty member is now `ValuePayload::Instances`, `None` at the empty member. `Value.bodies()` lists each present member's body, and the empty member contributes none. That follows this door's existing rule: an empty boolean gives `[]`, and a split's empty side is left out. `Value.split()` is the door that keeps a side's place, as `None`. `kind` for such a family is `"instances"`, the family-of-bodies word, so `"family"` now means only a family of non-bodies, as `members()`'s docstring says. `body()` and the recorded-body rule now read the kernel's one-body rule, `ValuePayload::one_body`, which is public now; the copy of that classifier in the binding is gone.

**Pinned** by `test_pattern.py` `TestTheRowIsAFamily.test_a_family_with_an_empty_member_lists_its_bodies`: `row[k] − row[1]` reads kind `"instances"`, its members' `bodies()` counts read `[1, 0, 1]`, and `bodies()` gives two boxes of volume 4.

---
id: a-range-refusal-of-a-structural-slot-states-no-recourse
kind: issue
title: RangeRefusal::StructuralSlot states no recourse, and no rendered-row gate reaches it
status: open
opened: 2026-11-02
priority: P4
cost: E
---



Found by the recourse sweep of WIREREFUSE's
`refusals-forwarding-a-name-state-no-marked-recourse` (PR 6291).

`crates/editor-core/src/range.rs`, `RangeRefusal`'s `Display`, the
`StructuralSlot` arm: "the {slot} slot of {node} is structural (Count)
— a structural slot selects between shapes and has no interval to
certify over". It states the problem and no way through. No rendered-row
gate holds it: it is not a row of `refusal_concision_chains.rs` (nor
admitted under `FILED_NO_RECOURSE`), and `docm9_range.rs` matches the
variant only.

## What is owed

A `sentence::Recourse` clause (certify a continuous slot's variable,
or none: a count's domain is enumerated, not certified), and the
variant brought under a rendered-row gate so the next unmarked
`RangeRefusal` arm shows.

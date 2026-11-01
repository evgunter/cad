---
id: transform-and-pattern-drop-a-values-contact-records
kind: issue
title: Transform and Pattern nodes drop a value's contact records, so a touching boolean result refuses at rest one node downstream with no change to it
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [3856]
closed: 2026-10-29
---


## What

In `crates/editor-core/src/eval/wire.rs` only instantiate and the boolean fill a value's contact records; every other node's `OpOut::plain` defaults them empty. A `Transform` or `Pattern` of a boolean result whose pieces touch (a declared corner kiss, say) therefore hands on a body with no records, and the at-rest pseudomanifold census refuses it as `UndeclaredContact` though the geometry is unchanged. Owed whichever way the split pinch question is answered (cross-operand records can never become shared keys). A row: transform a declared-touching boolean result and run the at-rest gate.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## The kernel half (2026-10-11, CONTACTHOLD)

`topo-surgery-verbs-drop-a-bodys-contact-records` added the topo carry:
`ContactRecords::carried_through` (since CONTACTHOLD's one carry, it
takes an op's `topo::Lineage`) and each surgery verb's `carry_contacts`.
`transform_rigid` keeps every key, so a `Transform` node's carry is
`carried_from(0)`, as `wire_place_in_world` already does. A `Pattern`
grafts copies, so it re-keys each copy's records through its graft keys,
as `product.rs::carry_contacts` does. The editor-core nodes still
default to `OpOut::plain`'s empty records.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. Both `wire_transform` and `wire_pattern` still return `OpOut::plain`. The Transform half is `a-transform-drops-the-contact-records-a-placement-carries` and retires with stage 3 D; the Pattern half is owed again in index-variable copies, so this row waits on the pattern unit. Parked on `patterns-are-index-variables`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Closed (2026-10-29, D10 exit walk)

The code is gone. `Transform` retired into a placement (stage 3 D, #5506), and `wire_place` carries its source's records (`crates/editor-core/src/eval/wire.rs`, `carried_from(0)` at the placement arm). Patterns are index variables (#5954). A family's value carries each member's own records (`crates/editor-core/src/eval/members.rs:331-345`, `Carry::of_family`), so a placed copy of a touching boolean result keeps its records per member.

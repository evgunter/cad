---
id: a-finished-body-holds-its-contact-records-at-every-door
kind: unit
title: AtRestBody holds its contact records; every door carries its operand's records through its own lineage and gates each finished body it returns at tier 3′
status: open
opened: 2026-10-28
priority: P1
cost: X
points: 16
---


Ratified by Ev on PR 5967 (2026-10-28), decision 1. The text is in `crates/topo/README.md` tier 3, finished-body paragraph.

## What

`AtRestBody` becomes a body together with its contact records, minted only by a gate that ran tier 3′ against those records. Every door that takes a finished body does three things itself:
- carries its operand's records through its own lineage;
- adds the records of the coincidences it decided;
- gates each finished body it returns.

What retires:
- Caller-side `carry_contacts`: `SplitResult::carry_contacts`, `Shelled::carry_contacts`, `MergeCoplanarOutcome::carry_contacts`, and replace-face's and blend's carries.
- `BooleanInputs` and `BooleanBody::contacts`, which fold into the operands and the result.
- Record drops that become unrepresentable:
  - the shorthand `union`/`subtract`/`intersect(a, b, tol)`, which today passes default inputs;
  - `wire_split`'s empty `Carry.contacts` (WIRE's `a-split-in-the-document-drops-its-operands-contact-records`).

The Boolean cannot hold a tier-3′ verdict until REACHHOLD's cross-solid curved lane lands (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`). Until then its result names the bar it paid (tier 3, or 3′) rather than letting one verdict mean both.

## Price

X, 16 points. It touches every door and editor-core, roughly two H rows. Review tier: dual. It changes a type every door reads, and changing it later would be hard.

## Done when

- Every door that returns a finished body returns one that holds its records.
- No caller-side carry remains.
- The split's door gates each side at tier 3′ against what it carried, measured on the A-cause tests in the tier-3′ row's measurement (`divided_cell_carry`, `one_record_carry`, `review_4730_divided_carry_probes`, `census::split_wedge_tests`, `a_carried_record_no_reading_covers`).

The gate stays at tier 3 for the split's other causes until this track's later rows land.

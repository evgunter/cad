---
id: a-blend-joint-re-decides-its-operands-junction
kind: issue
title: A blend's joint row re-decides the tangency its operand already decided, so its finding is the operand's verdict asked again
status: open
opened: 2026-10-30
priority: P2
cost: M
---

A fillet or chamfer whose chain runs through a junction of two rim
edges decides the junction tangent itself
(`crates/sweep/src/blend/battery.rs` `chain_turns`, `chain_g1`), and
records a `Tangent` row at `DecisionSite::BatteryJoint` over the two
edges. The junction is the operand's: a profile junction its
constructor made (a circle split into arcs, a tangent arc), or one the
profile's joint pass decided and recorded at
`DecisionSite::ProfileJunction`, or a boolean's tangent glue.
`[coincidence.booleans]` says a meeting inside one operand is that
operand's decision, carried. So the joint row asks the operand's
verdict again, and its finding is loud wherever the replay cannot
re-read it, although the operand already proved or recorded it.

`[ev]` #6086's answer (`a-gap-over-adjacent-cells-states-nothing-of-the-row`,
item 5) asks the blend to carry the operand's verdict: no row where the
operand built the junction tangent, and the operand's own row where it
decided it. That needs the battery's joint, which names edges of the
blended body, mapped back to the operand's junction and its record:
the sweep crate reads a `Body` with no provenance, so the carry belongs
where the fillet node is wired (`crates/editor-core/src/eval/wire.rs`,
the blend's rows named through `coincide::name_rows`), reading the
operand value's coincidence rows. Too large for the ATREST unit that
found it, so filed here; until it lands the joint row stays loud and
never refuses (no assertion says a tangency between two edges at a
point: `checks::at_rest::saying` has no arm for it).

Rows owed:
- a disc's rim (a circle split into two arcs) filleted records no joint
  row; `coincidence_door.rs`
  `a_filleted_disc_records_its_coaxial_supports_and_tangent_joints`
  moves;
- a profile with a decided `ProfileJunction`, extruded and filleted
  along it, carries that row as the joint's, reported once.

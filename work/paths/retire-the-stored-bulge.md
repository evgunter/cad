---
id: retire-the-stored-bulge
kind: unit
title: 5a: the stored bulge retires; validate checks carrier consistency; lifts copy the stored carrier; the writer emits Center
status: parked
opened: 2026-09-30
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [one-arc-carrier-type-in-geom-core]
---


5a of the #3453 ruling. It lands after `one-arc-carrier-type-in-geom-core`.

## Spec (PATHS orchestrator, 2026-09-30; details settled when the type unit merges)

- **Storage.** `ProfileLoop` drops `bulges`, and `ValidatedSegment`
  drops `bulge`. The emission layer pushes `(vertex, Segment)`. Every
  mode's carrier is still derived as today (`lower_arc`); 5b changes
  that.
- **Consistency checks.** `build_seg` decides q1's three checks as
  named predicates through the existing band, with a typed refusal:
  - start on carrier;
  - landing;
  - range 0 < |Δθ| ≤ 2π.

  Unit 3 adds the full-turn arm.
- **Lifts copy.** Pinned lifts, `map_scalar`, `reversed` and
  `lift_onto` copy the stored fields.
- **Registrations.** The sweep's unconditional rim and span
  registrations go, in the same commit as the copying lifts (otherwise
  the Interval `Contradicted` abort follows). The derived lowering
  registers the 2-D facts through the shared type's one spelling, and
  the sweep registers only rigidity. Pin a Sym unit test that the chain
  3-D → 2-D → r closes, including inside a larger expression.
- **Readers.** They move off the bulge:
  - `build_seg`'s sagitta and apex become chord-scale
    `(len/2)·tan(|Δθ|/4)`;
  - `derive_naming` matches the kind;
  - the `stackup` digest hashes the carrier;
  - `viewer::flatten` samples the arc.
- **Writer.** `lift::chain_form` writes `ArcTo(Center { … })`, and
  `compare` compares canonical segments.
- **Bits.** Disclose every moved bit with its cause. A flipped decision
  is shown sound or stops the unit.
- **Review tier: dual.**

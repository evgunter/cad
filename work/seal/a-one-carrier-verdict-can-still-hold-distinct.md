---
id: a-one-carrier-verdict-can-still-hold-distinct
kind: issue
title: topo: CarrierVerdict::One and CarrierEqError::Undecided carry a CarrierRelation that can hold Distinct
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by PR 6634's review (SEAL, `carrier-readings-keep-the-margins-they-decided-on`).

## What

`CarrierVerdict::One { relation: CarrierRelation, margin }` (`crates/topo/src/boolean/carrier_eq.rs`) can be built with `relation: Distinct`, which no rung builds. `CarrierEqError::Undecided { relation }` says "never `Distinct`" in prose too.

The readers then each pick an arm for the state that cannot occur:
- `rest_pair_verdict`'s `Ok(_)` folds it into "two carriers";
- the glue door's `GlueClass::of_senses(relation).map(..)` reads it as no glue.

## Shape

A two-valued orientation (aligned / opposed) inside `One` and `Undecided`, mapped to `CarrierRelation` only where a reader needs the three-valued relation.

The readers that build or match `relation`:
- glue.rs, contact_verify.rs, mod.rs (`GluedPair::read`, `carriers_read`);
- vtxfac.rs and recl.rs;
- census.rs and census/in_plane.rs, through `.one()`;
- editor-core's `coincide/replay.rs`.

It reaches past carrier_eq, so it was not done in PR 6634.

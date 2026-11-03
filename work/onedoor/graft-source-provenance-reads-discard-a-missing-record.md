---
id: graft-source-provenance-reads-discard-a-missing-record
kind: issue
title: graft_solids_impl skips a source entity whose provenance record is missing, where tier 1 says every entity has one
status: open
opened: 2026-11-03
---


## Finding

- **Where**: `crates/topo/src/boolean/combine.rs`, `graft_solids_impl`'s
  provenance pass (the six `if let Some(p) = src.<kind>_provenance.get(k)`
  reads, vertices through shells).
- **Importance**: low
- **Confidence**: sure of the shape; the posture change needs the
  premise checked per caller.
- **Raised by**: the D95 sweep on `onedoor/one-spelling-trio`, 2026-11-03.

A source entity with no provenance record is skipped: its graft lands
with no record, and nothing says so. Tier 1 forbids that state
(`validate.rs` pass 12, `ValidationError::MissingProvenance`: every live
entity of all seven topology arenas has a record). The same function
already answers the solid arena's read of the same table as row 4
(`Targets::Minted`: "every entity of a tier-1-valid body does, and every
public door keeps the body tier-1-valid"), so the six siblings answer one
premise two ways: a panic for solids, a silent discard for the rest.

A fix makes the six reads the solid read's posture (D2 row 4,
`unreachable!` naming the entity and the premise), after confirming that
every graft source (the boolean's operands mid-reduction, and the stage
the commit re-grafts) carries a record per entity — if one does not, the
premise is the defect, not the read.

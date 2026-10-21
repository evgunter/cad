---
id: the-door-panics-resolving-a-removed-face-loop-or-half-edge
kind: issue
title: mass_properties panics resolving a removed face, loop or half-edge of a corrupt body, where its contract is a typed Corrupt
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## Measured

Found through PR 5556's review (`review/5556-01D2cmWe`,
`crates/sweep/tests/review_5556_probes.rs`, `review_5556_corrupt_bodies`).
The probe takes a union of two balls and removes one entity at a time
(`Body::with_entity_removed_for_tests`): each face, each outer loop,
each edge and each half-edge. Six of those removals make
`topo::mass_properties` panic, where its contract is a typed
`MassPropsError::Corrupt`:
- four faces (`FaceKey(1v1)`, `2v1`, `3v5`, `6v5`);
- one loop (`LoopKey(1v1)`);
- one half-edge (`HalfEdgeKey(25v1)`).

Every one panics in the door's own face resolution
(`crates/topo/src/live.rs`, `proven`), on a shell's or a face's record
that names the removed key, before any seam region is read. It panics
the same with PR 5556's seam pass turned off, so it predates that PR.
`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`,
`a_corrupt_body_is_refused_typed_at_the_door`, admits these six and no
other.

## What closes it

The walk resolves each face it reads through a typed lookup, and
refuses a record naming a removed key `MassPropsError::Corrupt`. Then
the row's admitted list empties.

---
id: seat6-germ-end-to-end-awaits-seat7
kind: issue
title: SEAT-6's germ end-to-end acceptance (a document reaching the cyl×cyl germ) awaits SEAT-7's extrude/revolve flow
status: open
opened: 2026-09-04
refs: [SEAT-6, 1593]
---


## Finding

VERB-SEAT-DESIGN §6's second acceptance bullet for the parameter-identity
channel — a DOCUMENT declaring one shared radius parameter reaching the
cyl×cyl equal-radius germ end to end — is **not met by SEAT-6 (PR 1593)**,
and the unit says so rather than implying it.

What is pinned is the two halves, each executed against the real thing:

- the document half — `crates/editor-core/tests/seat6_param_source.rs`:
  real documents evaluated through the ordinary door, then the kernel's
  own `topo::field_source_evidence` asked over the evaluated fillet
  carriers (`Declared` across two bodies under one declared `r`; `None`
  for the kernel-direct twin at bit-identical radii; the scope and memo
  rows);
- the germ half — `crates/sweep/tests/seat6_germ_channel.rs` and the
  `frame_dispatch` rows in `crates/topo/src/boolean/join.rs`: kernel-built
  operands carrying tokens stamped by hand reaching the germ site's read,
  the closed form constructed and verified on the way.

What is NOT pinned anywhere is one run passing through both, because no
document can produce it today:

- a boolean OVER a filleted body refuses `FallbackExtentUnsupported` on
  the sphere octants every fillet result carries — a frontier that
  predates the channel and is pinned executed in
  `crates/editor-core/tests/m6_5_downstream.rs`;
- the only other parameter-fed cylinder carriers a document mints are
  extrude and revolve walls, and those verbs are not on the `Verb`
  substrate yet, so they declare no `param_flow` and attach no token.
  That migration is SEAT-7's territory (design §2, "migrated verb by
  verb"; SEAT's `plan.md`), which is what this item waits on.

## What closes it

Either of:

1. SEAT-7 migrates extrude (or revolve) with a flow row for the profile
   radius's carrier walls, after which the `verbs_germarms2` fixture is
   authorable as a document — two extruded circles at one declared `r`,
   spun off the pinch — and the end-to-end row is a document row in
   `seat6_param_source.rs` (or a sweep row over a document, whichever
   crate can reach both doors);
2. the boolean's sphere-octant frontier retires, after which two filleted
   cubes meeting at a blend band reach the germ from a document directly.

Until one lands, the acceptance bullet is met in halves and this file is
the durable statement of the gap.

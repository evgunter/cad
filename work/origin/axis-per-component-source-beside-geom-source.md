---
id: axis-per-component-source-beside-geom-source
kind: issue
title: The axis channel's per-component source beside GeomSource (step 3 of WIRE's ratified axis-channel cut)
status: open
opened: 2026-09-14
priority: P1
cost: H
---


## What

Step 3 of the cut in `work/wire/axis-shaped-identity-channel.md`
("the per-component source (TOPO) — the channel itself"), filed on
TOPO's slate when step 1 (`geom-source-absence-conflates-four-origins`,
PR 2576) landed and the WIRE row's named trigger fired. `GeomSource`
identifies a whole description; an axis is a COMPONENT of one, so two
cylinders sharing an axis have different `GeomSource`s and the shared
axis is not derivable from them. The channel is a per-component source
beside `GeomSource` in `crates/topo/src/source.rs`, composing through
placement as `SourceExpr::Placed` already does, on `GeomSource`'s side
of `crates/verbs/README.md` §3 P1's line (an axis is not a
motion-invariant field; P1 stands untouched — the WIRE row's round-3
correction, which a taker must not re-derive). Ratified design:
`docs/AXIS-DECLARATION-DESIGN.md` (Ev, 2026-09-12, PR 2404). In
principle parallel to step 2 (EXCH's
`step-import-discards-the-entity-ids-that-are-its-identity-channel`);
the WIRE row is parked on both. Not cut into TOPO's block order yet;
the program decides its slot when it takes it.

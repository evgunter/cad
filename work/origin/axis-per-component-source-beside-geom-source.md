---
id: axis-per-component-source-beside-geom-source
kind: issue
title: The axis channel's per-component source beside GeomSource (step 3 of WIRE's ratified axis-channel cut)
status: closed
opened: 2026-09-14
priority: P1
cost: H
branch: origin/axis-component-source
closed: 2026-09-29
pr: 3419
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

## Closed (2026-09-29, PR 3419)

Step 3 built: `AxisSource` (an opaque lowered base plus a readable
chain of `AxisPlacement { node, index }`), one row per axis-bearing
surface (`Body::surface_axis_sources`, `Source | Cleared`) beside the
origin record, attached through `Body::set_surface_axis_source`
(planes and splines refused `NoAxisOnKind`). The token names a LINE the
axis lies on — for a sphere, a line through the centre. Every
surface-transplanting door carries or drops the row through one home
(`Body::carry_surface_rows` / `drop_surface_rows`); a transform marks it
`Cleared` and the recipe layer's `place` re-stamps it, keyed on the
MAP (node, placement index), not the output-body ordinal. P2's
propagate half ships with it; attaching (editor-core) and the
`cs_pair_frame` consumer are step 4, WIRE's. Residues filed:
`axis-source-lowered-bytes-carry-no-minter-namespace`,
`axis-channel-serves-only-the-line-reading`.

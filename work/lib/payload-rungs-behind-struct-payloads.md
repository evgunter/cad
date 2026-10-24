---
id: payload-rungs-behind-struct-payloads
kind: issue
title: The payload-rung sweep reads one rung down through an uncurated struct payload, and finds thirteen discriminants no curation pass has decided
status: open
opened: 2026-10-10
---


Found by `nurbs/restriction-on-the-description` (PR 4491). Its hoist made
`MappedCurve` a struct over an enum (`MappedSource`), and
`scripts/payload-rung-sweep.py` reads only one rung below a curated
carrier. So the sweep stopped seeing the discriminant. The PR closes
that blind spot as (k): an uncurated struct payload's bare-`pub` fields
are now read as payloads of its carrier, one rung deep, and reported
`via` the struct.

The descent finds fifteen names. `MappedSource` and `Pcurve` are argued
in `crates/pncad/src/prelude.rs` group 4: the scaffold description's
rung is carried whole or not at all. The other thirteen have never been
decided, and this row holds them. Each is listed as payload, then
carrier via struct, with the site the sweep reports. Run
`python3 scripts/payload-rung-sweep.py` for the live table.

Uncurated:
- `AdoptionCandidate` (`step-import/src/error.rs`), under `StepImportError` via `AdoptionAttempt`;
- `BifurcationKind` (`editor-core/src/witness.rs`), under `NodeErrorKind` and `RefusalReason`, via `WitnessBifurcation`;
- `Implicated` (`editor-core/src/witness.rs`), under the same two carriers, via `WitnessBifurcation`;
- `DecisionSite` (`topo/src/coincidence.rs`), under `BooleanBody` via `Coincidence`;
- `Discharge` (`topo/src/coincidence.rs`), under `BooleanBody` via `Coincidence`;
- `RowCell` (`topo/src/coincidence.rs`), under `BooleanBody` via `Coincidence`;
- `OperandKeys` (`topo/src/boolean/ops.rs`), under `BooleanBody` via `BooleanNaming`;
- `Cell` (`topo/src/boolean/mod.rs`), under `BooleanBody` via `BooleanNaming` (its `merge_substitutions`, added by `fuse/fused-into-is-live`, PR 4398, after the descent's count above);
- `EntityKey` (`editor-core/src/names/table.rs`), under `UnnamedEntity` via `EntityRef`;
- `JoinReading` (`topo/src/boolean/edge_join.rs`), under `BooleanError` and `ValidationError`, via `JoinUndecided`;
- `UncrossableCarrier` (`topo/src/splitting/containment.rs`), under `BooleanError` via `Uncrossable`;
- `Origin` (`editor-core/src/coincide.rs`), under `Residual` via `Construction`;
- `Map` (`editor-core/src/coincide.rs`), under `Residual` via `Construction`;
- `Relation` (`topo/src/coincidence.rs`), under `BooleanBody` (prelude) and `StepImport`, via `Coincidence`. It read as cross-list while the sweep resolved by spelling: the `Relation` `document` carries is `editor_core::assembly::Relation`; this one is reachable only through `document::coincidence`, re-exported whole.

Each needs the usual decision: carry it where its carrier is carried, or
argue the non-carriage beside the carrier. When one is decided, move its
`DISPOSITIONS` / `CROSS_LIST_DISPOSITIONS` entry from `filed` to `argued`.
The descent's own limit: a struct inside the struct is not followed.

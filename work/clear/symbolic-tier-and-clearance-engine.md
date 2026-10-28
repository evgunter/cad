---
id: symbolic-tier-and-clearance-engine
kind: issue
title: The symbolic identity tier and the clearance engine do not compose
status: closed
opened: 2026-09-03
priority: P1
cost: H
branch: clear/symclear
closed: 2026-10-28
pr: 5928
---

The E12 symbolic identity tier (`geom_core::sym`) replays a driver leaf
at `Sym<Interval>`. Every lane trait it has to satisfy is scalar-generic
and runs there unaltered — `topo::props::{PropsQuadLane, AtRestPolicy}`,
`topo::chart_region::ChartRegionLane` and the fitted door
(`geom_brep::FittedLane`), which `AtRestPolicy::fitted_lane()` answers —
with one exception. (The plane × NURBS lane is no longer a trait: its
certified body `geom_brep::plane_nurbs_limbs` is bounded
`Decide + Bounds + CertifiedEnclosure`, and `Sym<T>` satisfies each of
those exactly when its base scalar does, so that tier needs no arm.)

`editor_core::measure::MinClearanceLane` cannot. The engine behind it,
`editor_core::clearance::min_separation`
(`crates/editor-core/src/clearance.rs:1588`), is written at
`geom_core::Interval` CONCRETELY: `MinSepSelection` borrows a
`&topo::Body<Interval>` (`clearance.rs:1589`) and the inner subdivision
is spelled in that type throughout. The leaf replay holds a
`Body<Sym<Interval>>`, and `topo` offers no scalar remap of a body to
strip one — `geom::Curve3::map_scalar` and `geom::Surface::map_scalar`
exist (`crates/geom/src/scalar_lift.rs`), but `topo::Body`'s arenas are
`SlotMap`s whose keys a rebuild cannot preserve.

**What ships instead** (M10-7, deviation D3):
`impl MinClearanceLane for Sym<T>` answers `None`
(`crates/editor-core/src/measure.rs`), and `drive` REFUSES a document
carrying a `min_clearance` measure up front when the tier is on —
`DriveRefusal::SymbolicClearanceUnsupported`
(`crates/editor-core/src/drive.rs`). The refusal is up front rather than
degraded because the trait's honest `None` reads downstream as
`ValuePayload::MeasureUnavailable`, which is a VALUE: the leaf would
certify with the clearance measure silently missing from it.

So such a document gets the numeric-only replay — the pre-E12 answer, at
the pre-E12 ε-scale ceiling. The M10-6 clearance suites drive with
`SymbolicDials::off()` by name, and `m10_6_ci_rows_interval`'s registry
falls back on the driver's own refusal rather than on a document-name
list.

**Two ways out**, neither taken here:

1. A scalar remap on `topo::Body` (`map_scalar<U>(f: impl Fn(T) -> U)`),
   which the geometry enums already support and the arenas do not.
2. Making the clearance engine generic in its lane scalar, as every
   other lane door already is.

Until one lands, a study whose measure is a `min_clearance` cannot use
the tier, which is exactly the class of study E7 exists for.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/m10/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-homed at M10's exit sweep (2026-09-13)

Here because the half that cannot compose is the engine: `min_separation` is written
at `geom_core::Interval` concretely in `crates/editor-core/src/clearance.rs`, this
program's file, and SHELL-3 is the unit that moves its body-level half into `topo`
behind `interval` — the same question from the other end. SYM owns the tier side and
the `Sym<Interval>` replay that asks for it.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## Note from SCALAR (2026-09-29)

`PropsQuadLane` and `ChartRegionLane` are gone as well (LANE-1, LANE-2): the quadrature and region doors are values (`topo::QuadLane`, `topo::RegionLane`) formed at `Sym<T>` like the fitted door, so the paragraph's "lane traits" are now one policy trait (`AtRestPolicy`) and three door values; its conclusion (all run at `Sym<Interval>` unaltered, `MinClearanceLane` excepted) is unchanged.

## Decided (CLEAR orchestrator, 2026-10-27)

Two designers weighed this (`docs/DESIGN-FORK-LOG.md`, the CLEAR rows
of 2026-10-27). After one round they recommended the same final state,
so the orchestrator decided it. No ratified text changes:
`docs/DESIGN.md` `[scalars]` already says measurements are generic
over `Real`, and the engine was the exception.

- **The door is generic and the engine stays interval arithmetic.**
  - The body-reading half takes `&Body<T>` for `T: Decide + CertifiedBounds`: `window_of`, `boundary_of`, `topo::cell_region` and `topo::station_region`.
  - It lifts each surface, curve and point it reads through `Interval::from_certified`, the idiom of `topo::QuadLane::certified()`.
  - Cells, enclosures, the funnelled `d − c` and the bracket stay at `Interval`. At `Interval` the numbers are bit-identical.
  - Point-in-face decisions stay generic, through `contfp` and `curved_face_containment`.
  - A station handed to a generic door enters as `T::from_computed`, never `from_f64`.
  - A `Sym`-native sweep was rejected. It would mint a fresh unknown per cell, and at every indeterminate margin it would run `Sym::decide_sign`'s identity attempt on a `sqrt` form that E12's frontier says can never be `Zero`.
- **One door value on the per-scalar policy, no trait.**
  - `AtRestPolicy::clearance_lane() -> Option<ClearanceLane<Self>>` is `Some` at `Interval` and at `Sym<T: CertifiedBounds>`, and `None` at `f64`, `Probe` and `Dual`.
  - `editor_core::measure::MinClearanceLane` is deleted, following H5's ruling 3 on PR 2701 and LANE-1/LANE-2.
  - The bracket re-enters `T` as the hull of `T::from_computed(lo)` and `T::from_computed(hi)`: the same bits at `Interval`, a fresh unknown at `Sym`.
  - Whether `f64` answers is one policy line. It stays `None`; `min-separation-tightening-crosses-the-drive`'s `## Decided` explains why the census question does not land there.
- **Deleted:** `DriveRefusal::SymbolicClearanceUnsupported` and the drive's up-front check. The M10-6 suites stop naming `SymbolicDials::off()`. `the_symbolic_tier_refuses_a_clearance_measure_by_its_spoken_node` becomes a positive pin: the tier on, and the bracket bit-identical to the numeric run.
- **Also closed by this row:** `clearance-replays-a-leaf-off-the-verdicts-lane`.

Built in two steps. `SHELL-3` moves the engine with the generic door
signature, with `Sym` still answering `None`. This row's unit then
turns `Sym` on, deletes the refusal and the trait, and replays each
leaf on its verdict's lane.

## Scope added (CLEAR orchestrator, 2026-10-28, from PR #5909's review)

- When `MinClearanceLane` goes, narrow `NodeErrorKind::MeasureClearanceRefused`'s carrier to what the measure door can raise. SHELL-3 split the refusal into `topo::clearance::EngineRefusal` and editor-core's `ClearanceRefusal`, but left the carrier wide.
- Add a row that runs the generic door at a `T` other than `Interval`. Today none does, and at `Interval` `from_computed` and `from_f64` coincide.
- The `clearance.rs:1588` references and the "written at Interval concretely" paragraph above describe the code before SHELL-3. The engine is now `topo::clearance`, with a generic door.

## Closed (2026-10-28, #5928)

- **What landed.**
  - `topo::ClearanceLane<T>` is a door value on `AtRestPolicy::clearance_lane()`. `Sym<T>` holds it exactly when `T` does, so it is `Some` at `Interval` and `Sym<Interval>`, and `None` at `f64`, `Probe` and `Dual`. A row pins each answer.
  - `MinClearanceLane` is deleted, and so is `DriveRefusal::SymbolicClearanceUnsupported`.
  - The bracket re-enters `T` as a computed hull: the same bits at `Interval`, a fresh unknown at `Sym`.
  - The measure refusal narrowed to `topo::clearance::MinSeparationRefusal`, with 4 arms.
- **Riders closed in the same PR:**
  - The clearance doors replay on the certified lane. `clearance_over` uses the verdict's tier. The per-leaf doors use `ClearanceQuery::symbolic`, which defaults to the default drive's tier.
  - The `NeedsEnclosure` door string is fixed.
- **Review.** A single FULL review gave APPROVE-WITH-FIXES, with two MAJORs: a 5 s-per-leaf replay path, and per-leaf doors still replaying off-lane. The fix pass root-caused the first: the old numeric replay refused before sweeping. It hoisted `cell_region`'s per-call lift, and fixed both MAJORs. The orchestrator read and accepted the delta.
- **Cost, measured.** A tier-on leaf is about 10× a numeric one. That is the tier's own evaluation, not the door. Ad hoc per-leaf queries now replay at the tier, and fifteen rows moved into the slow set.
- **Filed:** `work/pctail/loft-builds-at-interval-and-refuses-a-rational-column-at-sym.md` (P2).

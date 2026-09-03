---
id: symbolic-tier-and-clearance-engine
kind: issue
title: The symbolic identity tier and the clearance engine do not compose
status: open
opened: 2026-09-03
program: m10
---

The E12 symbolic identity tier (`geom_core::sym`) replays a driver leaf
at `Sym<Interval>`. Every lane trait it has to satisfy is scalar-generic
and runs there unaltered — `topo::props::{PropsQuadLane, AtRestPolicy}`,
`topo::chart_region::ChartRegionLane`, `geom_brep::{PcurveFittedLane,
EdgeNurbsLane}` — with one exception.

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

---
id: certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair
kind: issue
title: chart_region::certified_arms is an inf rate per kind with no InfSpeed tag, the rate pair's next member
status: open
opened: 2026-09-15
refs: [three-tables-of-the-chart-arms]
---



## Finding

`crates/topo/src/chart_region.rs`, `certified_arms`: the chart's
metring arms per kind — a plane's exact `(1, 1)`, a cylinder's
`(r, 1)`, and a certified INF assembly on every other carrier, gated
through `Margin::of` — are metres per chart unit, which is exactly
what `geom_core::InfSpeed` now names. They are the one shipped inf-side
rate producer the RATE-PAIR unit did NOT type, by the D283 ruling's
own scoping: the pair landed with `metered` and the sup door and the
three blurred sites, and "the remaining hand sites as their programs
touch them" is unit (iii).

Typing it would put the direction where the reader cannot miss it.
`chart_region.rs:105`'s module doc and the note at `:2576` both spend a
paragraph on *why* an inf is owed here and why quoting
`geom_brep::chart_stretch_sup` "would invert exactly that" — the
sup/inf confusion the pair exists to make unrepresentable, argued in
prose one file away from a `SupSpeed` that says it in the signature.
The assembly's own pin (`the two readings are 16x apart on one chart`,
in the same file) records that a full sup-swap of `certified_arms`
passed the entire topo+sweep suite green, which is the sharpest
statement of why prose is not enough here.

## Its neighbour, the same shape

`geom_brep::chart_stretch_inf`'s `ChartStretchInf` is the other half:
`inf_u`/`inf_v` are inf rates and `sup_u`/`sup_v` are the very numbers
`chart_stretch_sup` now answers as `SupSpeed` (the two doors report ONE
sup — `the_two_doors_report_one_sup` pins that equality). Its fields
stayed bare `f64`/`T` for the same scoping reason, and `area_inf` is an
AREA rate, which the linear pair does not name at all. Whoever types
`certified_arms` will pass through this struct on the way.

## Shape

`certified_arms` answers `InfSpeed<T>` per axis; the `Margin::of` gate
keeps its own reading of a collapsed arm (the pair is a tag, not a
positivity witness); the exact plane and cylinder arms are inf bounds
by being exact, as `param_rate`'s closed forms are. Small, and it
wants to land with — or just after — TRIM's
`three-tables-of-the-chart-arms`, which proposes making the same two
exact arms reachable through a `pub` door: whoever writes that door
chooses its return type, and this row says which one.

## Home

Filed by SCALAR's RATE-PAIR lane, which reached `chart_region.rs` only
to read it. `chart_region.rs` sits in no program's `paths:`; TRIM
already carries `three-tables-of-the-chart-arms` on the same function
from the same direction, so this lands beside it rather than opening a
second home for one file.

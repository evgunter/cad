---
id: a-boolean-over-an-instance-drops-its-parts-census-findings
kind: issue
title: A boolean over an instance drops the part's carried census findings, so the host reports none of the part's interference or could-not-look rows
status: open
opened: 2026-10-27
priority: P2
cost: M
---


## What

Found by the sweep in `an-instance-carries-its-parts-coincidences`;
read from the code, not yet witnessed by a row.

A part's census findings (`CarriedInterference`, `CarriedAtRestFinding`)
reach the host's report only through the product gather:
`product::sources_of` reads `value.carried` for a `ValuePayload::Body`
(an instance and its placements, which copy `carried` through) but
hands `norows()` for a `ValuePayload::Boolean` arm. Booleans build their
`OpOut` with `OpOut::plain` (`eval/wire.rs`), whose `carried` is the
default. So a host that unions (subtracts, intersects) an instance with
another body, and puts the result in its product, gathers no carried
rows: the part's interference and could-not-look findings are silent
there, and `Certified` reads the silence as at rest. A pick of an
instance's piece (`wire.rs`, the pick arm's `carried: Arc::default()`)
drops them the same way.

The coincidence rows the same PR carried do not have this gap: the
host's door reads every evaluated value's `carried.coincidences`
(`checks::carried_coincidences`), not the product's sources, so an
instance consumed by a boolean still reports its part's rows.

## Done when

A row: a host unioning an instance of the overlapping part
(`intent_part_census_seam::overlapping`) with a far block, the union
its product, reports the part's overlap once as `CarriedInterference`.
Whether the fix reads every evaluated value (as the coincidence rows
do) or carries the rows through the boolean's value is the lane's call.

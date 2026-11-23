---
id: the-chart-speed-bounds-endpoint-reads-are-undispositioned
kind: issue
title: pcurve_cache/speed.rs arrived with 38 certification endpoint reads, 34 outside an is_certified function, and the endpoint census has no disposition for them
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), reconciling the censuses with main `2c843a2e77`.

## What

`geom-core`'s `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` walks `crates/geom-brep/src/pcurve_cache/speed.rs`. That file arrived with PR 6943's merge (`ddccbc5dd7`, `tesslane/chord-rule`) and holds 38 `.lo()`/`.hi()` reads, 4 in a function that asks `is_certified()`. The census's message says each read that does not ask needs a disposition: a certification bracket's refusal is its decoration, so a one-sided read of a refused bracket takes the certifying branch. The census is in the slow set, so the PR gate did not run it.

The same merge moved `props/spiric.rs` 13/13 → 20/20. Every new read there sits in an asking function, and GREEN re-blessed it.

## Pinned meanwhile

GREEN pinned the roster at main's `(speed.rs, 38, 4)`, with a note naming this row as the place the 34 dispositions are owed.

## Owed

Read the 34 reads that do not ask. For each, either argue it at the site (a selection, a seed, an outward hull of finite values, a read behind a funnel), or ask `is_certified()` before reading. Then replace the roster note with the dispositions.

    cargo nextest run -p geom-core -E 'test(certified_endpoint_census)'

## 2026-11-23 — a spelling that disposes of the divisor reads (CHARTTAIL, PR 7509)

`pole_azimuth_speed` dropped its three endpoint reads by dividing first and reading the quotient's decoration: `σ_max² / (2·det M)` refuses (`Trv`) exactly when the divisor's enclosure holds or touches zero, so `if q.is_certified() { finite(q.mag()) } else { Err(ThroughSingularSet) }` asks the divisor's question with no census-counted read. `projected_speeds`'s `over` (`lever > 0.0`, then `(speed / lever).hi()`) and `root_down` (`x > 0.0` on a lower end) are the same shape: a lower end sign-tested in `f64` before a division or a root. `over` takes that spelling directly, which removes its `.hi()`. The levers themselves are `.lo()` reads in `Levers::of`, `line_levers` and `net_levers`, which this spelling does not reach.

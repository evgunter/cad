---
id: census-classifies-chart-region-refusals-beside-their-standing
kind: issue
title: The census classifies ChartRegionError by hand beside its standing, and the two disagree
status: parked
blocked_on: [6702]
opened: 2026-11-10
priority: P2
cost: E
---


Found by PR 6722's review (QML T4, `onecopy/refusal-standing`); filed rather than fixed there, on the orchestrator's ruling.

**Two answers to one question.** `ChartRegionError::refusal` (`crates/topo/src/chart_region.rs`, QML T4) is the one classification of a chart-region refusal: `Escalated` by its margin; `RayExhausted`, `TouchingBoundary` and `PeriodFold` in band; the rest limits or aborts. The census classifies the same refusals a second time, by hand, at two arms:
- `crates/topo/src/census.rs:6897-6939` (the sweep arm);
- `crates/topo/src/census.rs:12443-12477` (the confirming arm).

Each reads `Escalated` as `ValidationError::CensusEscalated` and every other variant as `CensusUnsupported { cause: ChartRegion(..) }`. So the three variants the standing reads in band are reported as unsupported at the census door, where a tighter tolerance may decide them, and `editor_core::attribute`'s `AtRest`/`Uncertified` split reads them as the wrong kind.

**Fix direction.** The census reads `refusal()`:
- `Margin` and `InBand` become the census's escalated (or in-band) report;
- `Limit` becomes `CensusUnsupported`;
- `Aborts` becomes what the census raises for a refusal of the whole query.

The two hand-listed arms go. A new variant is then classified once, where `refusal()`'s exhaustive match already makes it state its standing. `DegenerateLoop` and `TrimOffFace` abort until `chart-region-refusals-that-mix-classes-await-their-split` splits them, which the census then reads without a change of its own.

## Done when

Neither census arm names a `ChartRegionError` variant. Both read `ChartRegionError::refusal`, with a row pinning an in-band variant (`TouchingBoundary`) reported as in band, not unsupported.

## What `onecopy/chart-region-census` landed

Neither census arm names a `ChartRegionError` variant. Both hand their refusal to `census::chart_region_finding`, which reads `ChartRegionError::refusal`. `Margin(diag)` becomes `CensusEscalated { cause: diag }`, and every other standing becomes `CensusUnsupported { cause: ChartRegion(..) }`, as before. The behaviour is unchanged, and a new variant is classified once, where `refusal()`'s exhaustive match makes it state its standing. `census::tests::a_chart_region_refusal_reaches_the_census_by_its_standing` pins an escalation landing as escalated with its own diagnostic, and `TouchingBoundary` landing as unsupported with its pair.

## What remains: parked on `[ev]` PR 6702

The in-band refusals (`RayExhausted`, `TouchingBoundary`, `PeriodFold`) still read as `CensusUnsupported`. They carry no metred margin, so they cannot be `CensusEscalated { cause: Indeterminate }` without minting one, which `CensusUnsupportedCause::Containment`'s docs reject. Ratified text names no census report for a marginless in-band refusal. PR 6702's recommendation ("names carry no class; the standing is the class") would make the fix an application: `CensusUnsupported` becomes an ordinary word, and its render reads the standing. Under today's naming rule that would mislabel the finding, so this waits on 6702.

Whatever 6702 decides, an `Aborts` cause (`MissingCache`, `CarrierTilt`, `Corrupt`, and for now `TrimOffFace` and `DegenerateLoop`) has no census home of its own: the census has no report for a refusal of the whole query. They stay `CensusUnsupported` until one is named.

**The consumer claim above is stale.** No `editor_core::attribute` split reads these findings. `editor_core::checks::at_rest::could_not_look` sends `CensusEscalated` and `CensusUnsupported` alike to `NoLane`. The one difference between them is that `CensusUnsupported` carries the face pair (`finding_solids`) and `CensusEscalated` names no entity. Routing an in-band refusal to "escalated" would therefore drop its pair.

---
id: census-classifies-chart-region-refusals-beside-their-standing
kind: issue
title: The census classifies ChartRegionError by hand beside its standing, and the two disagree
status: open
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

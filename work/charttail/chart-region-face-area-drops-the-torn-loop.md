---
id: chart-region-face-area-drops-the-torn-loop
kind: issue
title: chart_region's face_area maps AreaMiss::Torn(loop) to a keyless ChartRegionError::Corrupt
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the sweep of `contain-error-drops-the-loop-its-carrier-named`
(INSIDE, lane `inside-contain-small`; the tuple-wildcard pass at the
blind spot of its `{ .. }` pass). Filed here because
`crates/topo/src/chart_region.rs` is CHARTTAIL's ground.

## The finding

`crates/topo/src/chart_region.rs`, `face_area`:

```rust
.map_err(|miss| match miss {
    crate::live::AreaMiss::NoOutline(no) => ChartRegionError::NoOutline(no),
    crate::live::AreaMiss::Torn(_) => ChartRegionError::Corrupt,
})
```

`AreaMiss::Torn(LoopKey)` names the loop that does not resolve or walk;
`ChartRegionError::Corrupt` is a unit variant, so the loop is dropped.
The census carries a chart-region refusal under a face-pair subject
(`CensusUnsupportedCause::ChartRegion`), so the faces reach a reader,
the loop does not. `ChartRegionError::Corrupt` is keyless at every one
of its raises (mostly `ok_or` on a lookup, which have no key to keep);
this one is the site whose source had one.

## The repair

Give `Corrupt` (or a sibling arm) the loop where the source names one,
and render it; or record that the face-pair subject is the whole
subject the chart region states, and close this as no defect.

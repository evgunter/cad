---
id: census-unexamined-recourse-lists-certifying-scalars-without-the-symbolic-tier
kind: issue
title: validate.rs's recourse sentence lists the certifying scalars as f64, the probe and the interval scalar, omitting the symbolic tier
status: open
opened: 2026-09-29
priority: P4
cost: E
---


## What

`crates/topo/src/validate.rs`, `ValidationError::CensusLaneUnsupported`'s
doc, gives the recourse as "the certified door family
([`validate_pseudomanifold`] and its siblings) at a certifying scalar —
`f64`, the telemetry probe or the interval scalar — where the same pair
is examined". The symbolic tier over any of those is a certifying scalar
for this door too: `RegionLane::certified()` is formed at `Sym<T>`
(`crates/topo/src/chart_region.rs`'s `wiring_rows`,
`sym_over_f64_is_wired_to_the_certified_region_doors`), and
`AtRestPolicy for Sym<T>` hands the base scalar's doors on.

## Proposed

Add "or the symbolic tier over one of them" to the list, as
`geom_brep::PcurveCertifyError::FittedLaneUnsupported`'s text now does.
Doc only; no row reads it.

## Found by

SCALAR-HYGIENE's sweep for the replay-list class
(`fitted-lane-refusal-text-omits-symbolic-and-cites-a-retired-hull`),
2026-09-29: `grep -rnE "telemetry probe,? (or|and) the interval|f64, the
telemetry probe" crates` — two hits, this one and
`geom-brep/src/edge_nurbs.rs`'s, which names the symbolic tier in its
next paragraph.

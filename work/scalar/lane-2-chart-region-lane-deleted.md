---
id: lane-2-chart-region-lane-deleted
kind: unit
title: LANE-2: ChartRegionLane deleted — the chart-region doors are one Option<RegionLane<T>> parameter through the census, None keeping today's typed refusal
status: closed
opened: 2026-09-21
closed: 2026-09-21
branch: scalar/lane-2
pr: 3038
---


## What

The second of the three kernel lane traits goes (`H5` §RATIFIED ruling
3, which supersedes the 2026-09-05 DEFER on this trait): `ChartRegionLane`'s
two methods forwarded to the two public doors `chart_region_overlap`
and `declared_pair_overlap`, already free functions at `Decide +
CertifiedBounds`; its `Dual` arm carried no logic. `topo::RegionLane<T>`
— two private fn-pointer fields, one constructor `RegionLane::certified()`
at `Decide + CertifiedBounds` — is taken as `Option<_>` by the twelve
census signatures and `pseudomanifold_certificate_via`, the certified
`validate_pseudomanifold_certificate` supplying `Some` by name and the
`_structural` twin `None`; `None` keeps today's `CensusLaneUnsupported`
at the two arms and the `false` fold in `pair_region_verified` exactly,
and the unit adds the first rows that observe them produced;
`AtRestPolicy: Decide + PcurveFittedLane`. Spec: `docs/LANE-2-SPEC.md`
(deleted at merge). Block SCALAR-B6 slot 0. Ground: CONTACT
(`census.rs`), CHART (`chart_region.rs`), ATREST (`validate.rs`), the
unowned `props.rs`/`lib.rs`, PROPS (`real.rs` prose, DL3 if named),
GUARD (a selftest fixture, two counts, one header line), WIRE (prose),
TCOST/TINT (`perf12_census_bvh_diff.rs`, the census test files);
announced.

## Closed (2026-09-21) — PR 3038

`ChartRegionLane` deleted with its five impls, its docs and the `pub
use`. `topo::RegionLane<T>` in `chart_region.rs` beside the two doors it
holds — `Copy`, two private fn-pointer fields, one constructor
`RegionLane::certified()` at `Decide + CertifiedBounds`, two
`pub(crate)` readers at `Decide` — taken as `Option<_>` by the twelve
census signatures (the four doors at `Decide + Bounds`, the eight
internals at `Decide`) and by `pseudomanifold_certificate_via` as its
third lane parameter; `validate_pseudomanifold_certificate` supplies
`Some` by name, the `_structural` twin `None`; the three read sites
keep the base's `None` arm read for read; `AtRestPolicy: Decide +
PcurveFittedLane`. **The verdict move, ruled to stand**: the
`_structural` doors hand the census `None` at every scalar, so at `f64`
on a declared straddle seat they answer a `CensusLaneUnsupported`
refusal plus the two crossings the declaration backed as
`UndeclaredContact`, where the base (Door 2 reached through the
supertrait) certified — no production caller and no Python route
reaches the doors; the door docs name the third absence, the PR body
the change, and the ATREST row (P3) the verdict and the `Display`'s
wrong subject and arm. The first rows that observe the variant
produced, incl. a `Dual64` row through the public door and a `f64`
declared-seat dump row; wiring by `fn_addr_eq` on both fields at four
scalars; the impl census generalised to both rosters; 318 dump rows
byte-identical base vs head. Reviews: dual, both APPROVE WITH FIXES,
the one MAJOR bilateral (both reviewers resumed on their arms after a
container restart); thirteen items, none declined; rows: ATREST
extended (P4 → P3), VACUITY filed (seven negative "backs no crossing"
rows stay green with the door gone). The second copy of the wiring
module (`QuadLane`'s and `RegionLane`'s) is LANE-4's to fold.

---
id: the-census-crossing-lane-reads-a-pair-sharing-no-point-key-from-their-starts
kind: issue
title: The census's crossing lane reads the closest point of two edges sharing no point key through the closed form from their starts, whose f64 rounding is |d|·u/sin φ: at ε = 1e-12, 646 of 60 000 true crossings read a definite no
status: closed
opened: 2026-10-18
priority: P0
cost: M
closed: 2026-10-18
---


## What

Found by PR 5259's first review (its randomized crossing falsifier,
branch `restread/crossing-lane-review1` @ `487543ab`,
`crates/topo/src/census/review_probes.rs`), on head `8d94ebec`. It
predates PR 5259; the same family as
`the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair`,
which that PR closed for pairs sharing a point key.

`crates/topo/src/census.rs` `ee_lines_closest` (census.rs:3308) reads the lines' closest
point from the ends the two edges share on one point key, else from
their starts. From two starts `d = eb.p0 − ea.p0` apart, the closed
form's `f64` rounding in each reach is about `|d|·u / sin φ`. Where that
exceeds `Kε`, a crossing strictly inside both spans can read a definite
span margin outside one edge (`pm_census_ee_span`), so `crossing_reading`
says a definite no: a missed crossing, not an escalation.

- Measured: 20 000 pairs at each of ε = 1e-6, 1e-9 and 1e-12 (60 000),
  each crossing at least `3Kε` inside both spans, angles from 1e-14 to
  1.6 rad, lengths from `21Kε` to 3.2, each read in both orders. 646
  `f64` readings are a definite no, all at ε = 1e-12; `Interval`
  escalates every one of them.
- Main reads the same pairs worse (the reviewer's run of main's
  `census.rs`: 10 869 crossing and 6 492 overlap false noes over the
  same batteries, most of them PR 5259's rows).

The census's own fuzzer, `census::segment_fuzz`
`crossings_strictly_inside_both_spans_never_read_apart`, excuses an
`f64` no only where `16·|d|·u / sin φ` reaches `4Kε` (`rounding_reaches`:
a crossing `3Kε` inside a span read `Kε` outside it), and logs how many
it excused: 564 of 40 000 readings at 1e-12 in one effort-40 run, none
at 1e-9 or 1e-6. Any other `f64` no fails it.

## Owed

Read the reach from an origin that does not carry the starts' distance
into the division: the pair of ends nearest each other, chosen without
branching on a value the generic scalar cannot order. Or bound the
closed form's rounding and escalate a span margin within it. Then lift
the fuzzer's rounding excuse; it is this row's test.

## Closed

`ee_lines_closest` now reads, for a pair sharing no point key, every
reach and the gap as an enclosure `[lower, upper]`. Each is read from
every pair of ends it allows (a reach from both ends of the other edge,
the gap from all four), charged `geom_core::rounding_charge` of
`|e|·(1 + s)/s` (gap) or `(|e| + |reach| + gap_hi/s)/s` (reach), and the
enclosures are intersected. `crossing_reading` decides both bounds
(`span_reading`, `gap_reading`): a definite answer only off the bound
that proves it, and a straddle escalated through `k_stats::straddle`
(`pm_census_ee_span_straddle`, `pm_census_ee_gap_straddle`).
`census::segment_fuzz` has no excuse left: f64 and Interval are both
asserted everywhere, each crossing measured exactly first, with a
past-an-end family (no definite yes) and an exact check that the closed
form's enclosures hold the real values. Measured on main at effort 100
(seed `0x1234`): 1 456 false f64 noes at 1e-12; none here. Pinned by
`census::edge_pairs_as_segments::nearly_parallel_crossings_sharing_no_point_key_are_never_read_apart`.

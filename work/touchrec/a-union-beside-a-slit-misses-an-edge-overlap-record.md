---
id: a-union-beside-a-slit-misses-an-edge-overlap-record
kind: issue
title: A union of a slit form with a part flush beside the slit, its edge along the slit's line, builds without the edge-overlap contact record: tier 3' refuses UnrecordedContact EdgeEdgeOverlap
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found by PR 5545's review (`cleave/fate-cut-ends-at-a-slit-tip`).

## What

The slit form open at one end of
`crates/topo/tests/a_keel_on_the_floor_subtracts_as_a_slit.rs`
(`[0,4]² × [0,1]` less the trench over `x ∈ [1.5, 4.5]`), ∪ a part
whose bottom lies on the floor on one side of the keel and whose edge
runs along the keel's line: the box `[1,3] × [2,2.5] × [0,0.5]`, or the
prisms over the `(y, z)` triangle `(2,0), (2,0.8), (1.2,0)` and its
mirror, along `x ∈ [1,3]` or `[1.5,2.5]`. Slit first, the union builds
right through tier 3 and the closed-form volume, one solid, but its
contact records hold no row for two of its edges overlapping along the
keel's line, so tier 3′ refuses
`UnrecordedContact { EdgeEdgeOverlap }` at `(3.5, 2, 0)` (the box's
case). Main ships the same body in `x ∪ y` and for the wall-to-wall
slit, with the same 3′ refusal.

The Boolean door certifies tier 3; its census over the result is
parked on REACHHOLD
(`work/reachhold/boolean-door-runs-the-census-over-its-result.md`), so
the body is an Ok whose 3′ is unproven, disclosed here.

## The ring's form

The same holds on the ringed plate (the plate less the pyramid lying
under its top, its ray a zero-area ring from `MEET` of length 0.5): a
pyramid standing on the top beside the ray, its apex on the ray at 0.1
or 0.2 and its edge along it to 0.3, 0.4, 0.5 or 0.8, its base
`[bearing(60, r, 0), bearing(150, 0.6, 0), bearing(20, 0.6, 0.7)]`
relative to its apex. `x ∪ y` and `y ∪ x` build right through tier 3
and the closed-form volume, alike, but tier 3′ refuses
`UnrecordedContact` `VertexOnEdge` and `EdgeEdgeOverlap` along the ray,
at ε default, 1e-6 and 1e-12, at rest, turned, and flipped and turned,
on main `be2b7a139c` and on `cleave/flush-partners-at-a-slit-build`
alike.

## Witness

`a_keel_on_the_floor_subtracts_as_a_slit::a_partner_flush_beside_the_keel_unions_with_the_slit_at_tier_three`
pins the five cells at every pose with the 3′ refusal asserted; it
fails once the record is carried, which is the cue to pin 3′ and close
this row.

## Owed

Carry the overlap's record into the result (`boolean/ops.rs`
`carry_rows`, or where the reduction's edge-edge rows are placed): the
two coincident edges along the keel bound the floor | wall material and
the part's, and the census needs the record citing them.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TOUCHREC holds a result's records of its own touches and the glue's kept copy; id and body unchanged. (CLEAVE orchestrator)

---
id: a-box-face-along-an-inflection-ruling-refuses-not-same-face-past-both-ends
kind: issue
title: A box face along the ruling where two tangent walls of opposite curvature meet refuses Join NotSameFace in every op when the box stands past both of A's ends
status: closed
opened: 2026-11-01
priority: P1
cost: M
refs: [a-box-along-a-fillets-line-arc-junction-reaches-classification-invariants, 6219]
closed: 2026-11-01
pr: 6249
---

Found by PR 6219's review (its junction variants), measured on the
branch `tangrim/fillet-line-arc-junction`.

## The pose

`A` is the square `[−1, 1]²` whose `(+, +)` corner runs a convex
quarter arc `r = 0.2` about `(0.8, 0.8)` from `(1, 0.8)` to `(0.8, 1)`,
then a concave quarter arc about `(0.8, 1.2)` up to `(0.6, 1.2)`, then
straight on along `y = 1.2`; extruded over `z ∈ [0, 1]`. `B` is the box
`[0.5, 1.5] × [0.5, 1]` over a `z` span. `B`'s face `y = 1` is tangent
to both arc walls along the ruling `x = 0.8`, the inflection: both of
the ruling's flanking faces are glued `Tangent` to `B`'s face and curve
to opposite sides of it.

Pinned in `crates/sweep/tests/a_box_along_a_fillets_line_arc_junction.rs`
(`a_box_along_an_inflection_ruling`), every op in both orders, at rest
and turned, at default ε, 1e-6 and 1e-12 alike:

- `B` over `z ∈ (−0.5, 1.5)`, standing past both of `A`'s ends: all 12
  runs refuse `Join(BooleanJoin(Euler(NotSameFace { .. })))`, raised by
  `euler_ring`'s `mekr` (`mekr_cycles` and its siblings) from the join
  (`boolean::join::bool_connect`): the join is handed a ring and a
  target loop on two different faces.
- `B` over `[0, 1]`, `[0, 0.5]`, `[−0.5, 1]`, `[0.25, 0.75]`: SOUND at
  the closed form (48 runs).

On `main` all 60 runs refused `ClassificationInvariant` ("odd number of
surviving crossing records at a vertex pair"); PR 6219's second-order
keying at the edge-sector site (`recl::resolve_edge_sector`, its
`curving` re-key where both flankers key `On`) built the 48.

## Where to look (unmeasured)

At `z ∈ (−0.5, 1.5)` both ends of `A`'s ruling lie inside `B`'s face,
so they are vertex-on-face sites (`vtxfac::classify_vertex_on_face`)
and the ruling has no edge-sector site; every other span has at least
one end at a vertex pair, and builds. At such a vertex both flanking
sectors are tangent lumps: PR 6219 orders one-carrier lumps before
tangent ones there, but two tangent lumps of opposite sign still both
write the ruling's shared bound and the later one wins, which is the
shape PR 6219 fixed for a coplanar flanker. The first thing to measure
is which class the ruling's entry ends with and whether the ring the
join receives is split across the two wall faces because of it.

## Fix

Read the shared bound of two tangent sectors at second order (as the
germ already does, `vtxfac`'s `TangentRead`) instead of letting either
lump claim it, and turn the pin's 12 refusals into SOUND rows.

## Closed

2026-11-01, PR 6249. Measured: at each end of the ruling the two wall sectors are tangent lumps of opposite sign (convex `In`, concave `Out`) sharing the ruling, which reads `On` at first order; the last lump written won, and orbit order reverses between the ends, so the ruling read `In` at one and `Out` at the other and the join refused `NotSameFace`. Now a bound two tangent sectors share takes neither lump: it reads its own side at second order through both sectors' `TangentRead::side` (equal at a legal pose, by Meusnier; disagreement refuses `ClassificationInvariant`), and an `On` folds order-free (`sectors::fold_on_bound`). 36 runs that refused on main build SOUND (108 over nine spans pinned, three ε, rest and turned). Full review: a firing census over 7243 tests showed only the intended resolutions moving. When `the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading` (PR 6253) deletes `TangentRead`, the block reduces to "the tangent-lump loop does not overwrite a shared bound".
